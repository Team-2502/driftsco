pub mod subsystems;
mod constants;

use crate::subsystems::drivetrain::Drivetrain;
use crate::subsystems::input::Inputs;

pub struct DriftingFerris {
    drivetrain: Drivetrain,
    inputs: Inputs,
}

impl DriftingFerris {
    pub fn new() -> Self {
        Self {
            drivetrain: Drivetrain::new(),
            inputs: Inputs::new(),
        }
    }

}