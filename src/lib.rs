pub mod subsystems;
mod constants;

use std::cell::RefCell;
use std::rc::Rc;
use crate::subsystems::drivetrain::Drivetrain;

pub struct DriftingFerris {
    drivetrain: Rc<RefCell<Drivetrain>>,

}