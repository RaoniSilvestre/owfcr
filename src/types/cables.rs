//! Cable defitions
//!
//! Each instance of the WFCR will have a set of cables. In the test suite provided, it will always
//! be three. For that reason, there is some assertions through the code and tricks that explore that.
//!
//! Nothing too magic that couldn't be easily reverted to generalize to a generic number of cables.

use color_eyre::eyre::{ContextCompat, Result};
use std::ops::Deref;

use crate::PENALTY;

/// A cable.
///
/// A cable type defines how much turbine energy it can take, and how expensive is to use that cable type.
///
/// The more `power` that the cable holds, more expensive the `price_per_meter` gets.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Cable {
    /// This number is always integer because
    /// the turbine energy is always normalized to 1. And therefore the cable will be able to hold a
    /// integer capacity.
    pub power: i8,
    /// This defines how expensive the use of the cable will be.
    pub price_per_meter: f64,
}

/// A group of cables
///
/// This structure defines the available cables for a instance of WFCR.
#[derive(Debug, Clone)]
pub struct Cables {
    /// Inner part of the `Cables` structure.
    cables: Vec<Cable>,
}

impl Deref for Cables {
    type Target = Vec<Cable>;

    fn deref(&self) -> &Self::Target {
        &self.cables
    }
}

impl TryFrom<String> for Cables {
    type Error = color_eyre::eyre::Error;

    fn try_from(content: String) -> Result<Self, Self::Error> {
        let mut cables = Vec::new();

        for line in content.lines() {
            let mut items = line.split_whitespace();

            let power: i8 = items.next().context("X ausente")?.parse()?;

            let price_per_meter: f64 = items.next().context("Y ausente")?.parse()?;

            assert!(price_per_meter.is_sign_positive());
            assert!(power.is_positive());

            let cable = Cable {
                power,
                price_per_meter,
            };

            cables.push(cable);
        }

        cables.sort_by_key(|a| a.power);

        assert_eq!(cables.len(), 3);

        Ok(Cables { cables })
    }
}

impl Cables {
    /// The name is not the best,
    ///
    /// `greatest_cable_conn` would be better probably.
    ///
    /// Returns the more capable cable available.
    pub fn best_cable_conn(&self) -> i8 {
        assert_eq!(self.len(), 3);
        self.cables[2].power
    }

    /// Given a flow, it returns the best cable that can hold that energy flow.
    ///
    /// # Panics
    ///
    /// If the flow is more than the amount possible for the problem, the function panics.
    #[must_use]
    pub fn get_cable_for_flow(&self, flow: i8) -> &Cable {
        assert_eq!(self.len(), 3);

        self.cables
            .iter()
            .find(|c| flow <= c.power)
            .unwrap_or(&Cable {
                power: i8::MAX,
                price_per_meter: PENALTY,
            })
    }

    /// Given a flow and a distance, returns the cost of using the best cable along the distance.
    #[inline]
    pub fn get_cable_cost(&self, flow: i8, distance: f64) -> f64 {
        assert_eq!(self.len(), 3);
        let price = self
            .cables
            .iter()
            .find(|c| flow <= c.power)
            .map_or(crate::PENALTY, |c| c.price_per_meter);

        price * distance
    }
}

#[test]
fn test_new_cable() {
    let c = "3 450
        1 234
        13 5000"
        .to_string();

    let cable = Cables::try_from(c).unwrap();

    assert_eq!(cable.cables.len(), 3);
    assert_eq!(
        cable.cables.first(),
        Some(&Cable {
            power: 1,
            price_per_meter: 234.
        })
    );
    assert_eq!(
        cable.cables.last(),
        Some(&Cable {
            power: 13,
            price_per_meter: 5000.
        })
    );
}
