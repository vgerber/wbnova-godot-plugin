use godot::{
    builtin::GString,
    classes::{INode3D, Node3D},
    obj::{Base, WithBaseField},
    prelude::{godot_api, GodotClass},
};

use crate::objects::nova_cell_context::NovaCellContext;

use super::robot_controller::RobotController;

#[derive(GodotClass)]
#[class(init, base=Node3D)]
struct NovaCell {
    #[var]
    #[export]
    #[init(val = GString::from("0.0.0.0"))]
    host: GString,
    #[var]
    #[export]
    #[init(val = GString::from("cell"))]
    cell: GString,
    #[var]
    #[export]
    access_token: GString,
    #[var]
    #[export]
    secure_connection: bool,
    base: Base<Node3D>,
}

#[godot_api]
impl INode3D for NovaCell {
    fn ready(&mut self) {
        let cell_context = NovaCellContext::from(
            self.host.to_string(),
            self.cell.to_string(),
            self.access_token.to_string(),
            self.secure_connection,
        );
        for child in self.base_mut().get_children().iter_shared() {
            if let Ok(mut robot_controller) = child.try_cast::<RobotController>() {
                robot_controller.bind_mut().setup(cell_context.clone());
                continue;
            }
        }
    }
}
