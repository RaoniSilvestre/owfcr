//! WFCR Graph node module.

/// Wind farm node. Holds the position and how much energy it holds.
#[derive(Debug, Clone, Copy)]
pub struct WindFarmNode {
    /// X axis
    pub x: f64,
    /// Y axis
    pub y: f64,
    /// "watts". 1 for every turbine, -|capacity| for the substation
    pub w: i8,
}

impl WindFarmNode {
    /// Initializes a node.
    #[must_use]
    pub fn new(x: f64, y: f64, w: i8) -> Self {
        WindFarmNode { x, y, w }
    }
}
