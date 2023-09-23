//! 
//! The Control Message is sent to robots over radio to inform them of what actions to take.
//! 

use packed_struct::prelude::*;

use crate::Team;

/// The body{X, Y, W} are multiplied (upon sending) by the VELOCITY_SCALE_FACTOR and devided
/// (upon receiving) to preserve at least 3 decimals of floating point precision.
const VELOCITY_SCALE_FACTOR: f32 = 1000.0;

/// The Trigger Mode Kicking
#[derive(Clone, Copy, Debug)]
pub enum TriggerMode {
    StandDown = 0,
    Immediate = 1,
    OnBreakBeam = 2,
}

impl Into<u8> for TriggerMode {
    fn into(self) -> u8 {
        self as u8
    }
}

/// The Control Message is Sent from the Base Station to the Robots.
/// 
/// Size = 80 Bits = 10 Bytes
#[derive(PackedStruct, Clone, Copy, Debug)]
#[packed_struct(bit_numbering="msb0", endian="msb")]
pub struct ControlMessage {
    // Team of the Robot (0: Blue) (1: Yellow)
    #[packed_field(bits="0")]
    pub team: bool,

    // Id of the Robot
    #[packed_field(bits="1..=4")]
    pub robot_id: Integer<u8, packed_bits::Bits::<4>>,

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

impl ControlMessage {
    pub fn new(
        team: Team,
        robot_id: u8,
        shoot_mode: bool,
        trigger_mode: TriggerMode,
        body_x: f32,
        body_y: f32,
        body_w: f32,
        dribbler_speed: i8,
        kick_strength: u8,
        role: u8,
    ) -> Self {
        Self {
            team: team.into(),
            robot_id: robot_id.into(),
            shoot_mode,
            trigger_mode: trigger_mode.into(),
            body_x: ((body_x * VELOCITY_SCALE_FACTOR) as i16).into(),
            body_y: ((body_y * VELOCITY_SCALE_FACTOR) as i16).into(),
            body_w: ((body_w * VELOCITY_SCALE_FACTOR) as i16).into(),
            dribbler_speed: dribbler_speed.into(),
            kick_strength: kick_strength.into(),
            role: role.into(),
            unused: 0u8.into(),
        }
    }
}

// TODO: Write Tests (I'm still not 100% certain how to write no-std tests)