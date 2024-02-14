//!
//! The addresses of radios on the nRF24L01+ Network
//! 

#[cfg(any(feature = "blue-team", not(feature = "yellow-team")))]
/// Base Station Address (in nRF24L01+ Pipe Addresses) for the Yellow Team
pub const BASE_STATION_ADDRESS: [u8; 5] = [0xE7, 0xE7, 0xE7, 0xE7, 0xE7];

#[cfg(any(feature = "blue-team", not(feature = "yellow-team")))]
/// Robot Addresses (in nRF24L01+ Pipe Addresses) for the Blue Team
pub const ROBOT_RADIO_ADDRESSES: [[u8; 5]; 6] = [
    [0xC3, 0xC3, 0xC3, 0xC3, 0xC1],
    [0xC3, 0xC3, 0xC3, 0xC3, 0xC2],
    [0xC3, 0xC3, 0xC3, 0xC3, 0xC3],
    [0xC3, 0xC3, 0xC3, 0xC3, 0xC4],
    [0xC3, 0xC3, 0xC3, 0xC3, 0xC5],
    [0xC3, 0xC3, 0xC3, 0xC3, 0xC6],
];

#[cfg(feature = "yellow-team")]
/// Base Station Address (as an nRF24L01+)
pub const BASE_STATION_ADDRESS: [u8; 5] = [0xA4, 0xA4, 0xA4, 0xA4, 0xA4];

#[cfg(feature = "yellow-team")]
/// Robot Addresses (in nRF24L01+ Pipe Addresses) for the Blue Team.
pub const ROBOT_RADIO_ADDRESSES: [[u8; 5]; 6] = [
    [0xD5, 0xD5, 0xD5, 0xD5, 0xD1],
    [0xD5, 0xD5, 0xD5, 0xD5, 0xD2],
    [0xD5, 0xD5, 0xD5, 0xD5, 0xD3],
    [0xD5, 0xD5, 0xD5, 0xD5, 0xD4],
    [0xD5, 0xD5, 0xD5, 0xD5, 0xD5],
    [0xD5, 0xD5, 0xD5, 0xD5, 0xD6],
];
