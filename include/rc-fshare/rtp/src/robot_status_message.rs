//!
//! The Robot Status Message is sent from the robots to the base station and finally to the computer
//! 

#[cfg(feature = "nostd")]
use alloc::format;

use packed_struct::prelude::*;

use crate::{Team, RTPHeader, MessageType};

/// battery_voltage is a direct reading from the micrcontroller's ADC
/// and must be converted to an actual voltage, which means it should be
/// multiplied by this scale factor (TODO: Debug the scale factor)
#[allow(unused)]
const BATTERY_SCALE_FACTOR: f32 = 0.09884;

/// The Robot Status Message is sent back from the robot's whenever they receive communication
/// to let software know that they are doing good.
/// 
/// Size = 21 Bytes
#[derive(PackedStruct, Clone, Copy, Debug)]
#[packed_struct(bit_numbering="msb0", endian="lsb")]
pub struct RobotStatusMessage {
    // Team of the RObot (0: Blue) (1: Yellow)
    #[packed_field(bits="0")]
    pub team: bool,

    // Id of the Robot
    #[packed_field(bits="1..=4")]
    pub robot_id: Integer<u8, packed_bits::Bits::<4>>,

    // True if the robot currently has ball sense
    #[packed_field(bits="5")]
    pub ball_sense_status: bool,

    // Status of the kicker (TODO: Confirm this)
    #[packed_field(bits="6")]
    pub kick_status: bool,

    // Health of the kicker
    #[packed_field(bits="7")]
    pub kick_healthy: bool,

    // Voltage measured by the ADC of the Microcontroller
    #[packed_field(bits="8..=15")]
    pub battery_voltage: Integer<u8, packed_bits::Bits::<8>>,

    // Erros experienced by the motor (TODO: Doc this better)
    #[packed_field(bits="16..=20")]
    pub motor_errors: Integer<u8, packed_bits::Bits::<5>>,

    // Status of the FPGA
    #[packed_field(bits="21")]
    pub fpga_status: bool,

    // Unusued bits so the encoded deltas line up against a byte boundary
    #[packed_field(bits="22.=23")]
    unused: Integer<u8, packed_bits::Bits::<2>>,

    // The deltas from each encoder on the Robot.
    #[packed_field]
    pub encoder_deltas: [u16; 18],
}

impl RobotStatusMessage {
    pub fn new(
        team: Team,
        robot_id: u8,
        ball_sense_status: bool,
        kick_status: bool,
        kick_healthy: bool,
        battery_voltage: u8,
        motor_errors: u8,
        fpga_status: bool,
        encoder_deltas: [u16; 18],
    ) -> Self {
        Self {
            team: team.into(),
            robot_id: robot_id.into(),
            ball_sense_status,
            kick_status,
            kick_healthy,
            battery_voltage: battery_voltage.into(),
            motor_errors: motor_errors.into(),
            fpga_status,
            unused: 0u8.into(),
            encoder_deltas,
        }
    }

    pub fn empty(team: Team, robot_id: u8) -> Self {
        Self {
            team: team.into(),
            robot_id: robot_id.into(),
            ball_sense_status: false,
            kick_status: false,
            kick_healthy: true,
            battery_voltage: 0u8.into(),
            motor_errors: 0u8.into(),
            fpga_status: true,
            unused: 0u8.into(),
            encoder_deltas: [0u16; 18],
        }
    }
}

impl RTPHeader for RobotStatusMessage {
    fn get_header() -> MessageType { MessageType::RobotStatusMessage }
}

// TODO: Write Tests (I'm still not 100% certain how to write no-std tests)