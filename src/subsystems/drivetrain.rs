use frcrs::ctre::{CanCoder, Talon};
use crate::constants::robotmap::drivetrain;

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
    fn new() -> Self {
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

    pub fn control_drivetrain{}
}