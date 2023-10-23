//!
//! The Control Commands are Used Internally by the Rust Programs to Handle Wake
//! Up and Power Down Commands
//! 

#[cfg(feature = "nostd")]
use alloc::format;

use packed_struct::prelude::*;

use crate::{Team, RTPHeader, MessageType};

/// List of Control Commands that can be sent to the robots
pub enum CommandTypes {
    WakeUp = 7,
    PowerDown = 0,
    Unknown = 1,
}

impl Into<u8> for CommandTypes {
    fn into(self) -> u8 {
        self as u8
    }
}

/// Control Commands are very specific commands that have the same format and can
/// be sent to the robots to make them behave in specific ways
#[derive(PackedStruct, Clone, Copy, Debug)]
#[packed_struct(bit_numbering="msb0", endian="msb")]
pub struct ControlCommand {
    // Team of the Robot (0: Blue) (1: Yellow)
    #[packed_field(bits="0")]
    pub team: bool,

    // Id of the Robot
    #[packed_field(bits="1..=4")]
    pub robot_id: Integer<u8, packed_bits::Bits::<4>>,

    // Command sent to the robot
    #[packed_field(bits="5..=7")]
    command: Integer<u8, packed_bits::Bits::<3>>,
}

impl ControlCommand {
    /// Create a wake up control command
    pub fn wake_up(team: Team, robot_id: u8) -> Self {
        Self {
            team: team.into(),
            robot_id: robot_id.into(),
            command: (CommandTypes::WakeUp as u8).into(),
        }
    }
    
    /// Create a shut down control command
    pub fn shut_down(team: Team, robot_id: u8) -> Self {
        Self {
            team: team.into(),
            robot_id: robot_id.into(),
            command: (CommandTypes::PowerDown as u8).into(),
        }
    }

    /// Decipher the Command Type from a ControlCommand
    pub fn command_type(&self) -> CommandTypes {
        match self.command.to_be() {
            0 => CommandTypes::PowerDown,
            7 => CommandTypes::WakeUp,
            _ => CommandTypes::Unknown,
        }
    }
}

impl RTPHeader for ControlCommand {
    fn get_header() -> MessageType { MessageType::ControlCommand }
}