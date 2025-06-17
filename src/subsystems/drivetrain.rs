use frcrs::ctre::{CanCoder, ControlMode, Talon};
use crate::constants::{robotmap::drivetrain, general};

//rwd with swerve modules on front for turning
pub struct Drivetrain {
    //swerve modules on front
    sr_drive: Talon,
    sr_turn: Talon,
    sr_encoder: CanCoder,

    sl_drive: Talon,
    sl_turn: Talon,
    sl_encoder: CanCoder,

    //tank on back
    tl_drive: Talon,
    tr_drive: Talon,
}

impl Drivetrain {
    pub fn new() -> Self {
        let sr_drive = Talon::new(drivetrain::SR_DRIVE_ID, None);
        let sr_turn = Talon::new(drivetrain::SR_TURN_ID, None);
        let sr_encoder = CanCoder::new(drivetrain::SR_ENCODER_ID, None);

        let sl_drive = Talon::new(drivetrain::SL_DRIVE_ID, None);
        let sl_turn = Talon::new(drivetrain::SL_TURN_ID, None);
        let sl_encoder = CanCoder::new(drivetrain::SL_ENCODER_ID, None);

        let tl_drive = Talon::new(drivetrain::TL_ID, None);
        let tr_drive = Talon::new(drivetrain::TR_ID, None);

        Self {
            sr_drive,
            sr_turn,
            sr_encoder,

            sl_drive,
            sl_turn,
            sl_encoder,

            tl_drive,
            tr_drive,
        }
    }

    pub fn forward(&self, percent: f64) {
        self.tl_drive.set(ControlMode::Percent, percent);
        self.tr_drive.set(ControlMode::Percent, percent);
    }

    pub fn turn(&self, position: f64) {
        self.sl_turn.set(ControlMode::Position, position * general::SWERVE_TURN_GEAR_RATIO);
        self.sr_turn.set(ControlMode::Position, position * general::SWERVE_TURN_GEAR_RATIO);
    }
}