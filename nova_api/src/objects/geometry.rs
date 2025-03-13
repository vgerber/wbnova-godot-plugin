use crate::objects::capsule::Capsule;
use crate::objects::compound::Compound;
use crate::objects::convex_hull::ConvexHull;
use crate::objects::cylinder::Cylinder;
use crate::objects::generic_box::GenericBox;
use crate::objects::plane::Plane;
use crate::objects::planner_pose::PlannerPose;
use crate::objects::rectangle::Rectangle;
use crate::objects::rectangular_capsule::RectangularCapsule;
use crate::objects::sphere::Sphere;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Geometry {
    pub rectangular_capsule: Option<RectangularCapsule>,
    pub compound: Option<Compound>,
    pub plane: Option<Plane>,
    #[serde(alias = "box")]
    pub geometry_box: Option<GenericBox>,
    pub convex_hull: Option<ConvexHull>,
    pub sphere: Option<Sphere>,
    pub id: Option<String>,
    pub cylinder: Option<Cylinder>,
    pub rectangle: Option<Rectangle>,
    pub capsule: Option<Capsule>,
    pub init_pose: PlannerPose,
}
