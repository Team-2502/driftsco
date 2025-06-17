pub mod general {
    pub const SWERVE_TURN_GEAR_RATIO: f64 = 12.8;
}

pub mod robotmap {
    pub mod drivetrain {
        pub const SR_DRIVE_ID: i32 = 1;
        pub const SR_TURN_ID: i32 = 1;
        pub const SR_ENCODER_ID: i32 = 1;

        pub const SL_DRIVE_ID: i32 = 1;
        pub const SL_TURN_ID: i32 = 1;
        pub const SL_ENCODER_ID: i32 = 1;

        pub const TL_ID: i32 = 1;
        pub const TR_ID: i32 = 1;
    }
}

pub mod inputmap {
    pub const WHEEL_ID: i32 = 1;
    pub const PEDAL_ID: i32 = 2;
}