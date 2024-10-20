#pragma once

#include <cstdint>
#include <string>
#include <vector>

namespace rtp {

/**
* Special enum to dictate the special mode the robot should be in.
*
* In general, software should never be using anything except Default unless
* firmware creates special states for software, however, I included this in
* the C++ section to make sure the commands in software are still parallel
* to the commands in firmware.
*/
enum ControlMode {
    DEFAULT = 0,
    IMU_TEST = 1,
    RECEIVE_BENCHMARK = 2,
    SEND_BENCHMARK = 3,
    PROGRAM_KICK_ON_BREAKBEAM = 4,
    PROGRAM_KICKER = 5,
    KICKER_TEST = 6,
    FPGA_TEST = 7,
}

struct ControlMessage {
    static constexpr float VELOCITY_SCALE_FACTOR = 1000.0f;

    // TODO: Determine the mapping for this
    unsigned trigger_mode: 2;
    // 0 -> Kick; 1 -> Chip
    unsigned shoot_mode: 1;
    // Id of the robot
    unsigned robot_id: 4;
    // Team of the robot (0: Blue) (1: Yellow)
    unsigned team: 1;
    // x velocity (body frame)
    int16_t body_x;
    // y velocity (body frame)
    int16_t body_y;
    // w velocity (body frame)
    int16_t body_w;
    // speed of the dribbler
    int8_t dribbler_speed;
    // strenght of the kicker
    uint8_t kick_strength;
    // Unused bytes
    unsigned unused : 6;
    // Robot role
    unsigned role : 2;
} __attribute__((packed));

struct RobotStatusMessage {
    static constexpr float BATTERY_SCALE_FACTOR = 0.09884f;

    // True if the kicker is healthy
    unsigned kick_healthy: 1;
    // True if is kicking
    unsigned kick_status: 1;
    // True if the robot has ball sense
    unsigned ball_sense_status: 1;
    // Id of the robot
    unsigned robot_id: 4;
    // Team of the robot (0: Blue) (1: Yellow)
    unsigned team: 1;
    // Battery Voltage
    uint8_t battery_voltage;
    // Unused data
    unsigned unused: 2;
    // FPGA is working
    unsigned fpga_status: 1;
    // Error per motor
    unsigned motor_errors: 5;
} __attribute__((packed));

}  // namespace rtp
