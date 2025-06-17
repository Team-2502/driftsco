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

    pub fn control_robot(&self) {
        let forward_amount = self.inputs.pedal.get_y();
        let turn_amount = self.inputs.wheel.get_x();

        self.drivetrain.forward(forward_amount);
        self.drivetrain.turn(turn_amount);
    }
}