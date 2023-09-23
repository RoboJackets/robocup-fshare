//!
//! rtp is a library of the cross platform messages that are sent between the base computer
//! base station, and robots.
//! 

pub mod control_message;
pub mod robot_status_message;
pub mod control_command;

// Team that Robots can be on.
#[derive(Clone, Copy, Debug)]
pub enum Team {
    // Blue Team
    Blue = 0,
    // Yellow Team
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