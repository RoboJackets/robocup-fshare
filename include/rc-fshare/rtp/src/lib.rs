//!
//! rtp is a library of the cross platform messages that are sent between the base computer
//! base station, and robots.
//! 

#![cfg_attr(feature = "nostd", no_std)]
#[cfg(feature = "nostd")]
extern crate alloc;

pub mod control_message;
pub mod robot_status_message;

// Team that Robots can be on.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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