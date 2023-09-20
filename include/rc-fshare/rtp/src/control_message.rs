//! 
//! The Control Message is sent to robots over radio to inform them of what actions to take.
//! 

use packed_struct::prelude::*;

/// The body{X, Y, W} are multiplied (upon sending) by the VELOCITY_SCALE_FACTOR and devided
/// (upon receiving) to preserve at least 3 decimals of floating point precision.
const VELOCITY_SCALE_FACTOR: f32 = 1000.0;

/// The Control Message is Sent from the Base Station to the Robots.
/// 
/// Size = 80 Bits = 10 Bytes
#[derive(PackedStruct, Clone, Copy, Debug)]
#[packed_struct(bit_numbering="msb0", endian="msb")]
pub struct ControlMessage {
    // Id of the Robot where first bit is the team (0 vs 1) and the remaining
    // 4 bits are the id of the robot (0->15)
    #[packed_field(bits="0..=4")]
    pub robot_id: Integer<u8, packed_bits::Bits::<5>>,

    // Mode of kicking for the robot
    // 0 -> Kick
    // 1 -> Chip
    #[packed_field(bits="5")]
    pub shoot_mode: bool,

    // Trigger Mode for the Robot (TODO: Finish Docs)
    #[packed_field(bits="6..=7")]
    pub trigger_mode: Integer<u8, packed_bits::Bits::<2>>,

    // X Coordinate of the Robot's Body Frame (multiplied by VELOCITY_SCALE_FACTOR
    // and truncated)
    #[packed_field(bits="8..=23")]
    pub body_x: Integer<i16, packed_bits::Bits::<16>>,

    // Y Coordinate of the Robot's Body Frame (multiplied by VELOCITY_SCALE_FACTOR
    // and truncated)
    #[packed_field(bits="24..=39")]
    pub body_y: Integer<i16, packed_bits::Bits::<16>>,

    // W Coordinate of the Robot's Body Frame (multiplied by VELOCITY_SCALE_FACTOR
    // and truncated))
    #[packed_field(bits="40..=55")]
    pub body_w: Integer<i16, packed_bits::Bits::<16>>,

    // Speed of the dribbler (TODO: Determine Units)
    #[packed_field(bits="56..=63")]
    pub dribbler_speed: Integer<i8, packed_bits::Bits::<8>>,

    // Strength of the kicker on kick (TODO: Determine Units)
    #[packed_field(bits="64..=71")]
    pub kick_strength: Integer<u8, packed_bits::Bits::<8>>,

    // Role of This Robot (TODO: Finish Docs)
    #[packed_field(bits="72..=73")]
    pub role: Integer<u8, packed_bits::Bits<2>>,

    // Unused Bits to make this struct an even byte length
    #[packed_field(bits="74..=79")]
    pub unused: Integer<u8, packed_bits::Bits::<6>>,
}

// TODO: Write Tests (I'm still not 100% certain how to write no-std tests)