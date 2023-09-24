//!
//! rtp is a library of the cross platform messages that are sent between the base computer
//! base station, and robots.
//! 

#![cfg_attr(feature = "nostd", no_std)]
#[cfg(feature = "nostd")]
extern crate alloc;

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

/// The Type of Message Being Sent to the Robots
#[derive(Clone, Copy, Debug)]
pub enum MessageType {
    /// Control Message (see control_message.rs)
    ControlMessage = 0,
    /// Control Command (see control_command.rs)
    ControlCommand = 1,
    /// Unknown Command
    Unknown = 255,
}

impl From<u8> for MessageType {
    fn from(value: u8) -> Self {
        match value {
            0 => MessageType::ControlMessage,
            1 => MessageType::ControlCommand,
            _ => MessageType::Unknown,
        }
    }
}

/// The 8 bit header (usually denoting the message type)
pub trait RTPHeader {
    /// Gets the header value corresponding to the specific outgoing message type.
    fn get_header() -> MessageType;
}