use crate::objects::geometry::Geometry;
use crate::objects::planning_limits::PlanningLimits;
use crate::objects::robot_link_geometry::RobotLinkGeometry;
use crate::objects::safety_zone::SafetyZone;
use crate::objects::safety_zone_limits::SafetyZoneLimits;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SafetyConfiguration {
    pub safety_zone_limits: Option<Vec<SafetyZoneLimits>>,
    pub tcp_geometries: Option<Vec<Geometry>>,
    pub safety_zones: Option<Vec<SafetyZone>>,
    pub global_limits: PlanningLimits,
    pub robot_model_geometries: Option<Vec<RobotLinkGeometry>>,
}
