//!
//! The Robot Status Message is sent from the robots to the base station and finally to the computer
//! 

use packed_struct::prelude::*;

/// battery_voltage is a direct reading from the micrcontroller's ADC
/// and must be converted to an actual voltage, which means it should be
/// multiplied by this scale factor (TODO: Debug the scale factor)
const BATTERY_SCALE_FACTOR: f32 = 0.09884;

/// The Robot Status Message is sent back from the robot's whenever they receive communication
/// to let software know that they are doing good.
/// 
/// Size = 21 Bytes
#[derive(PackedStruct, Clone, Copy, Debug)]
#[packed_struct(bit_numbering="msb0", endian="msb")]
pub struct RobotStatusMessage {
    // Id of the Robot where the first bit is the team (0 vs 1) and the remaining
    // 4 bits are the id of the robot (0->15)
    #[packed_field(bits="0..=4")]
    pub robot_id: Integer<u8, packed_bits::Bits::<5>>,

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

impl Default for RobotStatusMessage {
    fn default() -> Self {
        Self {
            robot_id: 0.into(),
            ball_sense_status: false,
            kick_healthy: false,
            kick_status: false,
            battery_voltage: 0.into(),
            motor_errors: 0.into(),
            fpga_status: false,
            unused: 0.into(),
            encoder_deltas: [0u16; 18],
        }
    }
}

// TODO: Write Tests (I'm still not 100% certain how to write no-std tests)