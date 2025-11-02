use std::sync::mpsc;

use godot::{
    builtin::{GString, Variant, VariantType},
    classes::{INode, Node},
    global::godot_error,
    meta::ToGodot,
    obj::{Base, WithBaseField},
    prelude::{godot_api, GodotClass},
};
use godot_tokio::AsyncRuntime;
use wbnova_api::v2::{
    objects::{
        io_boolean_value::IoBooleanValue, io_float_value::IoFloatValue,
        io_integer_value::IoIntegerValue, io_value::IoValue,
    },
    paths::{
        set_output_values::{
            set_output_values, SetOutputValuesPathParameters, SetOutputValuesResponseType,
        },
        stream_io_values::{
            stream_io_values, StreamIoValuesPathParameters, StreamIoValuesQueryParameters,
        },
    },
};

use crate::objects::nova_cell_context::NovaCellContext;

#[derive(GodotClass)]
#[class(init, base=Node)]
pub struct ControllerIO {
    base: Base<Node>,
    cell_context: Option<NovaCellContext>,
    io_state_channel: Option<mpsc::Receiver<IoValue>>,
    #[export]
    controller: GString,

    #[export]
    #[var]
    #[init(val = GString::from("digital_out"))]
    io: GString,

    #[export]
    #[var]
    #[init(val = 0.01)]
    float_threshold: f32,

    current_io_value: Option<Variant>,
}

#[godot_api]
impl INode for ControllerIO {
    fn physics_process(&mut self, _delta: f64) {
        let receiver = match self.io_state_channel {
            Some(ref receiver) => receiver,
            None => return,
        };

        let io_value = match receiver.try_recv() {
            Ok(io_value) => io_value,
            Err(_) => return,
        };

        println!("{:#?}", io_value);

        let value_variant = match io_value_to_variant(&io_value) {
            Some(variant) => variant,
            None => return,
        };

        let is_first_value = self.current_io_value.is_none();
        self.current_io_value = Some(value_variant.clone());

        if is_first_value {
            return;
        }
        self.base_mut()
            .emit_signal("value_changed", &vec![value_variant]);
    }
}

#[godot_api]
impl ControllerIO {
    #[signal]
    fn value_changed(value: Variant);

    pub fn setup(&mut self, cell_context: NovaCellContext) {
        let (sender, receiver) = mpsc::channel::<IoValue>();
        let cell = cell_context.cell.clone();
        let controller = self.controller.to_string();
        self.io_state_channel = Some(receiver);
        let io = self.io.to_string();
        let float_threshold = self.float_threshold;

        let ws_host = cell_context.get_websocket_base_url();
        let ws_headers = cell_context.get_websocket_headers();

        AsyncRuntime::spawn(async move {
            // request params
            let robot_data = StreamIoValuesPathParameters {
                cell: cell,
                controller: controller,
            };
            let robot_query_data = StreamIoValuesQueryParameters {
                ios: Some(vec![io]),
            };

            let mut socket =
                match stream_io_values(&ws_host, robot_data, robot_query_data, &Some(ws_headers))
                    .await
                {
                    Ok(socket) => socket,
                    Err(err) => {
                        println!("Failed to connect. {}", err);
                        return;
                    }
                };

            let mut previous_value: Option<IoValue> = None;
            loop {
                let io_values = match socket.read() {
                    Ok(message) => message.io_values,
                    Err(err) => {
                        println!("Failed to read motion state. {}", err);
                        break;
                    }
                };

                if io_values.len() != 1 {
                    println!(
                        "Only single io watch is supported. Give {}",
                        io_values.len()
                    );
                }

                let value = &io_values[0];

                let previous_value_ref = match previous_value {
                    None => {
                        previous_value = Some(value.clone());
                        if let Err(err) = sender.send(value.clone()) {
                            println!("Failed to send initial io value. {}", err);
                        }
                        continue;
                    }
                    Some(ref previous_value) => previous_value,
                };

                if io_value_equal(&value, previous_value_ref, float_threshold as f64) {
                    continue;
                }

                previous_value = Some(value.clone());
                if let Err(err) = sender.send(value.clone()) {
                    println!("Failed to send io value. {}", err);
                }
            }
        });

        self.cell_context = Some(cell_context);
    }

    #[func]
    pub fn read_io(&self) -> Variant {
        match self.current_io_value {
            Some(ref value) => value.clone(),
            None => Variant::nil(),
        }
    }

    #[func]
    pub fn write_io(&self, value: Variant) {
        let cell_context = match self.cell_context {
            Some(ref cell_context) => cell_context,
            None => {
                godot_error!("{}/{} no cell context found", self.controller, self.io);
                return;
            }
        };

        let cell = cell_context.cell.clone();
        let client_builder = cell_context.get_http_client();
        let base_url = cell_context.get_http_base_url();
        let controller = self.controller.to_string();
        let io = self.io.to_string();

        let write_result =
            AsyncRuntime::runtime().block_on(async move {
                let client = match client_builder.build() {
                    Ok(client) => client,
                    Err(err) => {
                        return Err(err.to_string());
                    }
                };

                let path_parameters = SetOutputValuesPathParameters {
                    controller: controller,
                    cell: cell.to_string(),
                };

                let io_value = match value.get_type() {
                    VariantType::BOOL => IoValue::IoBooleanValueValue(IoBooleanValue {
                        value: value.booleanize(),
                        value_type: "boolean".to_string(),
                        io: io.clone(),
                    }),
                    VariantType::FLOAT => IoValue::IoFloatValueValue(IoFloatValue {
                        value: value.to::<f64>(),
                        value_type: "float".to_string(),
                        io: io.clone(),
                    }),
                    VariantType::INT => IoValue::IoIntegerValueValue(IoIntegerValue {
                        value: value.to::<String>(),
                        value_type: "integer".to_string(),
                        io: io.clone(),
                    }),
                    _ => {
                        return Err(format!("Unsupported variant type: {:?}", value.get_type()));
                    }
                };
                let response =
                    match set_output_values(&client, &base_url, vec![io_value], path_parameters)
                        .await
                    {
                        Ok(response) => response,
                        Err(err) => {
                            return Err(err.to_string());
                        }
                    };

                match response {
                    SetOutputValuesResponseType::UndefinedResponse(_) => Ok(()),
                    SetOutputValuesResponseType::NotFound(error) => {
                        return Err(format!("Failed to write io. {:?}", error));
                    }
                    SetOutputValuesResponseType::BadRequest(error) => {
                        return Err(format!("Failed to write io. {:?}", error));
                    }
                    SetOutputValuesResponseType::TooManyRequests(error) => {
                        return Err(format!("Failed to write io. {:?}", error));
                    }
                }
            });

        if let Err(err) = write_result {
            godot_error!("{err}")
        }
    }
}

fn io_value_to_variant(io_value: &IoValue) -> Option<Variant> {
    match io_value {
        IoValue::IoFloatValueValue(value) => return Some(value.value.to_variant()),
        IoValue::IoBooleanValueValue(value) => return Some(value.value.to_variant()),
        IoValue::IoIntegerValueValue(value) => return Some(value.value.to_variant()),
    }
}

fn io_value_equal(io_value_a: &IoValue, io_value_b: &IoValue, tolerance: f64) -> bool {
    match (io_value_a, io_value_b) {
        (IoValue::IoBooleanValueValue(a), IoValue::IoBooleanValueValue(b)) => {
            return a.value == b.value
        }
        (IoValue::IoIntegerValueValue(a), IoValue::IoIntegerValueValue(b)) => {
            return a.value == b.value
        }
        (IoValue::IoFloatValueValue(a), IoValue::IoFloatValueValue(b)) => {
            return (a.value - b.value).abs() < tolerance
        }
        _ => return false,
    }
}
