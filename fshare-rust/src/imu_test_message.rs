//!
//! Test message sent during the IMU test
//! 

use ncomm_utils::packing::{Packable, PackingError};

/// The size of an IMU Test Message in Bytes
pub const IMU_MESSAGE_SIZE: usize = 13;

#[derive(Clone, Copy, Debug, PartialEq)]
/// A Message sent back from the robot while it is testing
/// its IMU
pub struct ImuTestMessage {
    /// Is this the first IMU test message
    pub first_message: bool,
    /// Is this the last IMU test message
    pub last_message: bool,
    /// The z-gyro value obtained from the IMU
    pub gyro_z: f32,
    /// the x-accelerometer value obtained from the IMU
    pub accel_x: f32,
    /// the y-accelerometer value obtained from the IMU
    pub accel_y: f32,
}

impl Packable for ImuTestMessage {
    fn len() -> usize {
        IMU_MESSAGE_SIZE 
    }

    fn pack(self, buffer: &mut [u8]) -> Result<(), PackingError> {
        if buffer.len() < IMU_MESSAGE_SIZE {
            return Err(PackingError::InvalidBufferSize);
        }

        buffer[0] = (self.first_message as u8) << 4 | (self.last_message as u8);
        buffer[1..5].copy_from_slice(&self.gyro_z.to_le_bytes());
        buffer[5..9].copy_from_slice(&self.accel_x.to_le_bytes());
        buffer[9..13].copy_from_slice(&self.accel_y.to_le_bytes());

        Ok(())
    }

    fn unpack(data: &[u8]) -> Result<Self, PackingError> {
        if data.len() < IMU_MESSAGE_SIZE {
            return Err(PackingError::InvalidBufferSize);
        }

        Ok(Self {
            first_message: data[0] & 0b1 << 4 != 0,
            last_message: data[0] & 0b1 != 0,
            gyro_z: f32::from_le_bytes(data[1..5].try_into().unwrap()),
            accel_x: f32::from_le_bytes(data[5..9].try_into().unwrap()),
            accel_y: f32::from_le_bytes(data[9..13].try_into().unwrap()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test that imu messages are packed and unpacked correctly
    #[test]
    fn test_imu_test_message_pack_and_unpack() {
        let message = ImuTestMessage {
            first_message: true,
            last_message: true,
            gyro_z: -1.5,
            accel_x: 1.25,
            accel_y: 32.3,
        };

        let mut buffer = [0u8; IMU_MESSAGE_SIZE];
        message.pack(&mut buffer).unwrap();
        assert_eq!(buffer[0], 0b0001_0001);

        let unpacked_message = ImuTestMessage::unpack(&buffer).unwrap();

        assert_eq!(
            message,
            unpacked_message,
        );
    }
}
