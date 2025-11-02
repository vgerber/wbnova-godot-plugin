use std::{sync::mpsc, time::Duration};

use godot::{
    builtin::{Array, Basis, GString, Transform3D, Vector3},
    classes::{INode3D, Node3D, PackedScene},
    global::godot_error,
    obj::{Base, Gd, WithBaseField},
    prelude::{godot_api, GodotClass},
};
use godot_tokio::AsyncRuntime;
use wbnova_api::v2::{
    objects::{dh_parameter::DhParameter, motion_group_state::MotionGroupState},
    paths::{
        get_motion_group_description::{
            get_motion_group_description, GetMotionGroupDescriptionPathParameters,
            GetMotionGroupDescriptionResponseType,
        },
        stream_motion_group_state::{
            stream_motion_group_state, StreamMotionGroupStatePathParameters,
            StreamMotionGroupStateQueryParameters,
        },
    },
};

use crate::objects::nova_cell_context::NovaCellContext;

use super::controller_io::ControllerIO;

#[derive(GodotClass)]
#[class(init, base=Node3D)]
pub struct RobotController {
    base: Base<Node3D>,
    cell_context: Option<NovaCellContext>,
    dh_parameters: Option<Vec<DhParameter>>,
    motion_group_state_channel: Option<mpsc::Receiver<MotionGroupState>>,
    #[export]
    controller: GString,

    #[export]
    motion_group: GString,

    #[export]
    #[var]
    #[init(val=None)]
    tool: Option<Gd<PackedScene>>,

    #[export]
    #[var]
    #[init(val = 32)]
    refresh_rate: i32,

    joint_nodes: Array<Gd<Node3D>>,
}

#[godot_api]
impl INode3D for RobotController {
    fn ready(&mut self) {
        self.joint_nodes = RobotController::collect_joints(&self.base());
        self.attach_tool();
    }

    fn physics_process(&mut self, _delta: f64) {
        let dh_parameters = match self.dh_parameters {
            Some(ref dh_parameters) => dh_parameters,
            None => return,
        };

        let receiver = match self.motion_group_state_channel {
            Some(ref receiver) => receiver,
            None => return,
        };

        let state = match receiver.recv_timeout(Duration::from_millis(1)) {
            Ok(state) => state,
            Err(_) => return,
        };

        let joint_values = state.joint_position;
        if joint_values.len() != self.joint_nodes.len() {
            godot_error!(
                "Joint values {} do not match the node count {}",
                joint_values.len(),
                self.joint_nodes.len()
            );
            return;
        }

        for joint_index in 0..joint_values.len() {
            let joint_node = self.joint_nodes.at(joint_index);
            let joint_value = joint_values[joint_index];
            let dh_parameter = match joint_index {
                0 => &DhParameter {
                    a: Some(0.0),
                    alpha: Some(0.0),
                    d: Some(0.0),
                    theta: Some(0.0),
                    reverse_rotation_direction: Some(false),
                },
                _ => &dh_parameters[joint_index - 1],
            };
            self.apply_joint_value(joint_node, joint_value as f32, dh_parameter);
        }
    }
}

#[godot_api]
impl RobotController {
    pub fn setup(&mut self, cell_context: NovaCellContext) {
        for child in self.base_mut().get_children().iter_shared() {
            if let Ok(mut robot_controller) = child.try_cast::<ControllerIO>() {
                robot_controller.bind_mut().setup(cell_context.clone());
                continue;
            }
        }

        let cell = cell_context.cell.clone();
        let client_builder = cell_context.get_http_client();
        let base_url = cell_context.get_http_base_url();
        let controller = self.controller.to_string();
        let motion_group = self.motion_group.to_string();

        let specification_result = AsyncRuntime::runtime().block_on(async move {
            let client = match client_builder.build() {
                Ok(client) => client,
                Err(err) => {
                    return Err(err.to_string());
                }
            };

            let path_parameters = GetMotionGroupDescriptionPathParameters {
                cell: cell.to_string(),
                controller: controller.clone(),
                motion_group: motion_group.clone(),
            };
            let response =
                match get_motion_group_description(&client, &base_url, path_parameters).await {
                    Ok(response) => response,
                    Err(err) => {
                        return Err(err.to_string());
                    }
                };

            match response {
                GetMotionGroupDescriptionResponseType::Ok(specification) => Ok(specification),
                _ => {
                    return Err("Invalid motion group specification".to_owned());
                }
            }
        });

        let specification = match specification_result {
            Ok(specification) => specification,
            Err(err) => {
                godot_error!("Failed to fetch motion group specification. {}", err);
                return;
            }
        };
        self.dh_parameters = specification.dh_parameters;

        let (sender, receiver) = mpsc::channel::<MotionGroupState>();
        let cell = cell_context.cell.clone();
        let controller = self.controller.to_string();
        let motion_group = self.motion_group.to_string();
        let refresh_rate = self.refresh_rate;
        self.motion_group_state_channel = Some(receiver);

        let ws_host = cell_context.get_websocket_base_url();
        let ws_headers = cell_context.get_websocket_headers();

        AsyncRuntime::spawn(async move {
            // request params
            let robot_data = StreamMotionGroupStatePathParameters {
                cell: cell,
                controller: controller,
                motion_group: motion_group,
            };
            let robot_query_data = StreamMotionGroupStateQueryParameters {
                response_rate: Some(refresh_rate),
                response_coordinate_system: None,
            };

            let mut socket = match stream_motion_group_state(
                &ws_host,
                robot_data,
                robot_query_data,
                &Some(ws_headers),
            )
            .await
            {
                Ok(socket) => socket,
                Err(err) => {
                    println!("Failed to connect. {}", err);
                    return;
                }
            };

            loop {
                let state = match socket.read() {
                    Ok(message) => message,
                    Err(err) => {
                        println!("Failed to read motion state. {}", err);
                        break;
                    }
                };
                if let Err(err) = sender.send(state) {
                    println!("Failed to send motion state. {}", err);
                }
            }
        });

        self.cell_context = Some(cell_context);
    }

    fn apply_joint_value(
        &self,
        mut joint_node: Gd<Node3D>,
        joint_value: f32,
        dh_parameter: &DhParameter,
    ) {
        let reverse = match dh_parameter.reverse_rotation_direction {
            Some(reverse) => reverse,
            None => false,
        };
        let rotation_direction_factor: f32 = match reverse {
            true => -1.0,
            _ => 1.0,
        };
        let a = match dh_parameter.a {
            Some(a) => (a / 1000.0) as f32,
            None => 0.0,
        };
        let d = match dh_parameter.d {
            Some(d) => (d / 1000.0) as f32,
            None => 0.0,
        };
        let theta = match dh_parameter.theta {
            Some(theta) => theta as f32,
            None => 0.0,
        };
        let alpha = match dh_parameter.alpha {
            Some(alpha) => alpha as f32,
            None => 0.0,
        };

        let mut joint_transform = Transform3D::new(
            Basis::from_cols(
                Vector3::from_array([1.0, 0.0, 0.0]),
                Vector3::from_array([0.0, 1.0, 0.0]),
                Vector3::from_array([0.0, 0.0, 1.0]),
            ),
            Vector3::from_array([0.0, 0.0, 0.0]),
        );

        joint_transform = joint_transform.translated_local(Vector3::from_array([0.0, d, 0.0]));
        joint_transform = joint_transform.translated_local(Vector3::from_array([a, 0.0, 0.0]));

        joint_transform =
            joint_transform.rotated_local(Vector3::from_array([1.0, 0.0, 0.0]), alpha);
        joint_transform = joint_transform.rotated_local(
            Vector3::from_array([0.0, 1.0, 0.0]),
            rotation_direction_factor * joint_value + theta,
        );

        joint_node.set_transform(joint_transform);
    }

    fn collect_joints(root_node: &Gd<Node3D>) -> Array<Gd<Node3D>> {
        let mut joint_nodes: Array<Gd<Node3D>> = Array::new();

        // traverse the joint try along all nodes with something like _JXX
        for child in root_node.get_children().iter_shared() {
            let joint_node = match child.try_cast::<Node3D>() {
                Ok(joint_node) => joint_node,
                Err(_) => continue,
            };

            joint_nodes = RobotController::collect_joints(&joint_node);

            let name_parts = joint_node.get_name().split("_");
            if name_parts[name_parts.len() - 1].begins_with("J") {
                joint_nodes.push_front(&joint_node);
            }
        }
        joint_nodes
    }

    fn attach_tool(&self) {
        let tool_scene = match self.tool {
            Some(ref tool_scene) => tool_scene,
            None => return,
        };
        let tool = match tool_scene.instantiate() {
            Some(tool) => tool,
            None => return,
        };

        let mut flange_node = match RobotController::get_flange_node(&self.base()) {
            Some(node) => node,
            None => {
                godot_error!("No FLG node found");
                return;
            }
        };

        flange_node.add_child(&tool);
    }

    fn get_flange_node(root_node: &Gd<Node3D>) -> Option<Gd<Node3D>> {
        for child in root_node.get_children().iter_shared() {
            let joint_node = match child.try_cast::<Node3D>() {
                Ok(joint_node) => joint_node,
                Err(_) => continue,
            };

            let name_parts = joint_node.get_name().split("_");
            if name_parts[name_parts.len() - 1] == GString::from("FLG") {
                return Some(joint_node);
            }
        }
        None
    }

    #[signal]
    fn speed_increased();
}
