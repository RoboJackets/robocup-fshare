//!
//! Messages related to radio benchmarking.
//! 

use ncomm_utils::packing::{Packable, PackingError};

/// The size of a Radio Receive Benchmark Message
pub const RADIO_RECEIVE_BENCHMARK_SIZE: usize = 8;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// Message sent from the robot back to the base station
/// when benchmarking the radio's receiving capabilities
pub struct RadioReceiveBenchmarkMessage {
    /// The amount of time the radio was receiving for (ms)
    pub receive_time_ms: u32,
    /// The total number of packets received during the time
    /// interval
    pub received_packets: u32,
}

impl Packable for RadioReceiveBenchmarkMessage {
    fn len() -> usize {
        RADIO_RECEIVE_BENCHMARK_SIZE
    }

    fn pack(self, buffer: &mut [u8]) -> Result<(), PackingError> {
        if buffer.len() < RADIO_RECEIVE_BENCHMARK_SIZE {
            return Err(PackingError::InvalidBufferSize);
        }

        buffer[0..4].copy_from_slice(&self.receive_time_ms.to_le_bytes());
        buffer[4..8].copy_from_slice(&self.received_packets.to_le_bytes());

        Ok(())
    }

    fn unpack(data: &[u8]) -> Result<Self, PackingError> {
        if data.len() < RADIO_RECEIVE_BENCHMARK_SIZE {
            return Err(PackingError::InvalidBufferSize);
        }

        Ok(Self {
            receive_time_ms: u32::from_le_bytes(data[0..4].try_into().unwrap()),
            received_packets: u32::from_le_bytes(data[4..8].try_into().unwrap()),
        })
    }
}

/// The size of a radio send benchmark message
pub const RADIO_SEND_BENCHMARK_SIZE: usize = 8;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// Message sent from the robot back to the base station when a radio send benchmark
/// test is complete
pub struct RadioSendBenchmarkMessage {
    /// The number of packets successfully sent (and acknowledged) according
    /// to the robot
    pub acknowledged_packets: u32,
    /// The total number of packets sent by the robot
    pub sent_packets: u32,
}

impl Packable for RadioSendBenchmarkMessage {
    fn len() -> usize {
        RADIO_SEND_BENCHMARK_SIZE
    }

    fn pack(self, buffer: &mut [u8]) -> Result<(), PackingError> {
        if buffer.len() < RADIO_SEND_BENCHMARK_SIZE {
            return Err(PackingError::InvalidBufferSize);
        }

        buffer[0..4].copy_from_slice(&self.acknowledged_packets.to_le_bytes());
        buffer[4..8].copy_from_slice(&self.sent_packets.to_le_bytes());

        Ok(())
    }

    fn unpack(data: &[u8]) -> Result<Self, PackingError> {
        if data.len() < RADIO_SEND_BENCHMARK_SIZE {
            return Err(PackingError::InvalidBufferSize);
        }

        Ok(Self {
            acknowledged_packets: u32::from_le_bytes(data[0..4].try_into().unwrap()),
            sent_packets: u32::from_le_bytes(data[4..8].try_into().unwrap()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test that radio receive benchmark messages can be packed and unpacked
    #[test]
    fn test_radio_receive_benchmark_message_pack_and_unpack() {
        let message = RadioReceiveBenchmarkMessage {
            receive_time_ms: 5_000,
            received_packets: 100,
        };

        let mut buffer = [0u8; RADIO_RECEIVE_BENCHMARK_SIZE];
        message.pack(&mut buffer).unwrap();

        let unpacked_message = RadioReceiveBenchmarkMessage::unpack(&buffer).unwrap();

        assert_eq!(
            message,
            unpacked_message,
        );
    }

    /// Test that radio send benchmark messages can be packed and unpacked
    #[test]
    fn test_radio_send_benchmark_message_pack_and_unpack() {
        let message = RadioSendBenchmarkMessage {
            acknowledged_packets: 100,
            sent_packets: 100,
        };

        let mut buffer = [0u8; RADIO_SEND_BENCHMARK_SIZE];
        message.pack(&mut buffer).unwrap();

        let unpacked_message = RadioSendBenchmarkMessage::unpack(&buffer).unwrap();

        assert_eq!(
            message,
            unpacked_message,
        );
    }
}