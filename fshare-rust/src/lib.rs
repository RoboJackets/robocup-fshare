//!
//! rtp is a library of the cross platform messages that are sent between the base computer
//! base station, and robots.
//!

#![no_std]
#![deny(missing_docs)]

pub mod control_message;
pub use control_message::{ControlMessage, ControlMessageBuilder, CONTROL_MESSAGE_SIZE};

pub mod robot_status_message;
pub use robot_status_message::{RobotStatusMessage, RobotStatusMessageBuilder, ROBOT_STATUS_SIZE};

pub mod imu_test_message;

pub mod kicker_program_message;

pub mod radio_addresses;
pub use radio_addresses::BASE_STATION_ADDRESSES;
pub use radio_addresses::ROBOT_RADIO_ADDRESSES;

/// Constant used to select the blue team
pub const BLUE_TEAM: usize = 0;
/// Constant used to select the yellow team
pub const YELLOW_TEAM: usize = 1;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// The Team the Robots are on
pub enum Team {
    /// Blue Team
    Blue = 0,
    /// Yellow Team
    Yellow = 1,
}

impl Into<bool> for Team {
    fn into(self) -> bool {
        match self {
            Team::Blue => false,
            Team::Yellow => true,
        }
    }
}
