use frcrs::input::Joystick;
use crate::constants::inputmap;

pub struct Inputs {
    pub wheel: Joystick,
    pub pedal: Joystick,
}

impl Inputs {
    pub fn new() -> Inputs {
        Inputs {
            wheel: Joystick::new(inputmap::WHEEL_ID),
            pedal: Joystick::new(inputmap::PEDAL_ID)
        }
    }
}