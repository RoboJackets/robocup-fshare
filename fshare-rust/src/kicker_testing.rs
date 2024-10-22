//!
//! Message containing information about the kicker when
//! running in Kicker Testing Mode
//! 

use ncomm_utils::packing::{Packable, PackingError};

/// The size of a Kicker Testing Message
pub const KICKER_TESTING_SIZE: usize = 2;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// Messages sent from the robot while testing the kicker
pub struct KickerTestingMessage {
    /// Is the kicker healthy
    pub healthy: bool,
    /// Does the kicker have ball sense
    pub ball_sense: bool,
    /// Is the kicker kicking (otherwise it is chipping)
    pub kicking: bool,
    /// Should the kicker activate on ball sense
    pub kick_on_ball_sense: bool,
    /// Should the kicker be kicking immediately
    pub kick_immediately: bool,
    /// The current voltage of the kicker
    pub voltage: u8,
}

impl Packable for KickerTestingMessage {
    fn len() -> usize {
        KICKER_TESTING_SIZE
    }

    fn pack(self, buffer: &mut [u8]) -> Result<(), PackingError> {
        if buffer.len() < KICKER_TESTING_SIZE {
            return Err(PackingError::InvalidBufferSize);
        }

        buffer[0] = (self.healthy as u8) |
            (self.ball_sense as u8) << 1 |
            (self.kicking as u8) << 2 |
            (self.kick_on_ball_sense as u8) << 3 |
            (self.kick_immediately as u8) << 4;
        buffer[1] = self.voltage;

        Ok(())
    }

    fn unpack(data: &[u8]) -> Result<Self, PackingError> {
        if data.len() < KICKER_TESTING_SIZE {
            return Err(PackingError::InvalidBufferSize);
        }

        Ok(Self {
            ball_sense: data[0] & 0b1 != 0,
            kicking: data[0] & 0b10 != 0,
            healthy: data[0] & 0b100 != 0,
            kick_on_ball_sense: data[0] & 0b1000 != 0,
            kick_immediately: data[0] & 0b1_0000 != 0,
            voltage: data[1],
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test that a kicker testing message can be packed and unpacked
    #[test]
    fn test_kicker_testing_message_pack_and_unpack() {
        let message = KickerTestingMessage {
            healthy: true,
            ball_sense: true,
            kicking: true,
            kick_on_ball_sense: true,
            kick_immediately: true,
            voltage: 123
        };

        let mut buffer = [0u8; KICKER_TESTING_SIZE];
        message.pack(&mut buffer).unwrap();
        assert_eq!(buffer[0], 0b0001_1111);
        assert_eq!(buffer[1], 123);

        let unpacked_message = KickerTestingMessage::unpack(&buffer).unwrap();

        assert_eq!(
            message,
            unpacked_message,
        )
    }
}