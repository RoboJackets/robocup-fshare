//!
//! Message containing information about the current kicker
//! programmer operation
//! 

use ncomm_utils::packing::{Packable, PackingError};

/// The size of a Kicker Program Message
pub const KICKER_PROGRAM_MESSAGE: usize = 5;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// Messages sent from the robot when the kicker is being programmed
pub struct KickerProgramMessage {
    /// Is the kicker being programmed with kick on breakbeam
    pub kick_on_breakbeam: bool,
    /// Is the kicker finished programming
    pub finished: bool, 
    /// The current page being programmed
    pub page: u32,
}

impl Packable for KickerProgramMessage {
    fn len() -> usize {
        KICKER_PROGRAM_MESSAGE
    }

    fn pack(self, buffer: &mut [u8]) -> Result<(), PackingError> {
        if buffer.len() < KICKER_PROGRAM_MESSAGE {
            return Err(PackingError::InvalidBufferSize);
        }

        buffer[0] = (self.kick_on_breakbeam as u8) << 4 | (self.finished as u8);
        buffer[1..5].copy_from_slice(&self.page.to_le_bytes());

        Ok(())
    }

    fn unpack(data: &[u8]) -> Result<Self, PackingError> {
        if data.len() < KICKER_PROGRAM_MESSAGE {
            return Err(PackingError::InvalidBufferSize);
        }

        Ok(Self {
            kick_on_breakbeam: data[0] & 0b1 << 4 != 0,
            finished: data[0] & 0b1 != 0,
            page: u32::from_le_bytes(data[1..5].try_into().unwrap()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test that a kicker program message can be packed and unpacked
    #[test]
    fn test_kicker_program_message_pack_and_unpack() {
        let message = KickerProgramMessage {
            kick_on_breakbeam: true,
            finished: true,
            page: 3
        };

        let mut buffer = [0u8; KICKER_PROGRAM_MESSAGE];
        message.pack(&mut buffer).unwrap();
        assert_eq!(buffer[0], 0b0001_0001);

        let unpacked_message = KickerProgramMessage::unpack(&buffer).unwrap();

        assert_eq!(
            message,
            unpacked_message,
        )
    }
}
