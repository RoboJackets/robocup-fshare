//!
//! The control test message takes the data used by the robot
//! for the current control step and sends the raw sensor values
//! back to the base station.
//! 
//! This message is sent when testing the FPGA.
//! 

use ncomm_utils::packing::{Packable, PackingError};

/// The size of a control update test message
pub const CONTROL_TEST_MESSAGE_SIZE: usize = 32;

#[derive(Clone, Copy, Debug, PartialEq)]
/// A message sent back from the robot containing all relevant
/// sensor measurements for making control decisions.
pub struct ControlTestMessage {
    /// The z-gyro value obtained from the IMU
    pub gyro_z: f32,
    /// The x-accelerometer value obtained from the IMU
    pub accel_x: f32,
    /// The y-accelerometer value obtained from the IMU
    pub accel_y: f32,
    /// The encoder velocities obtained from the FPGA
    pub motor_encoders: [f32; 4],
    /// The time from the last control test message to this message (us)
    pub delta: u32,
}

impl Packable for ControlTestMessage {
    fn len() -> usize {
        CONTROL_TEST_MESSAGE_SIZE
    }

    fn pack(self, buffer: &mut [u8]) -> Result<(), PackingError> {
        if buffer.len() < CONTROL_TEST_MESSAGE_SIZE {
            return Err(PackingError::InvalidBufferSize);
        }

        buffer[0..4].copy_from_slice(&self.gyro_z.to_le_bytes());
        buffer[4..8].copy_from_slice(&self.accel_x.to_le_bytes());
        buffer[8..12].copy_from_slice(&self.accel_y.to_le_bytes());
        buffer[12..16].copy_from_slice(&self.motor_encoders[0].to_le_bytes());
        buffer[16..20].copy_from_slice(&self.motor_encoders[1].to_le_bytes());
        buffer[20..24].copy_from_slice(&self.motor_encoders[2].to_le_bytes());
        buffer[24..28].copy_from_slice(&self.motor_encoders[3].to_le_bytes());
        buffer[28..32].copy_from_slice(&self.delta.to_le_bytes());

        Ok(())
    }

    fn unpack(data: &[u8]) -> Result<Self, PackingError> {
        if data.len() < CONTROL_TEST_MESSAGE_SIZE {
            return Err(PackingError::InvalidBufferSize);
        }

        Ok(Self {
            gyro_z: f32::from_le_bytes(data[0..4].try_into().unwrap()),
            accel_x: f32::from_le_bytes(data[4..8].try_into().unwrap()),
            accel_y: f32::from_le_bytes(data[8..12].try_into().unwrap()),
            motor_encoders: [
                f32::from_le_bytes(data[12..16].try_into().unwrap()),
                f32::from_le_bytes(data[16..20].try_into().unwrap()),
                f32::from_le_bytes(data[20..24].try_into().unwrap()),
                f32::from_le_bytes(data[24..28].try_into().unwrap()),
            ],
            delta: u32::from_le_bytes(data[28..32].try_into().unwrap()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test that Control Test Messages are packed and unpacked correctly
    #[test]
    fn test_control_test_message_pack_and_unpack() {
        let message = ControlTestMessage {
            gyro_z: 2.0,
            accel_x: -1.5,
            accel_y: 2.2,
            motor_encoders: [0.7, 0.2, 0.2, 0.7],
            delta: 100,
        };

        let mut buffer = [0u8; CONTROL_TEST_MESSAGE_SIZE];
        message.pack(&mut buffer).unwrap();

        let unpacked_message = ControlTestMessage::unpack(&buffer).unwrap();

        assert_eq!(
            message,
            unpacked_message,
        );
    }
}