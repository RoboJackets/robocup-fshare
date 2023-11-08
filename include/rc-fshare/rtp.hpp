#pragma once

#include <cstdint>
#include <string>
#include <vector>

namespace rtp {

struct ControlMessage {
    static constexpr float VELOCITY_SCALE_FACTOR = 1000.0f;

    unsigned trigger_mode: 1;
    unsigned shoot_mode: 1;
    unsigned robot_id: 4;
    unsigned team: 1;

    // // Team of the robot (0: Blue) (1: Yellow)
    // unsigned team: 1;
    // // Id of the robot
    // unsigned robot_id : 4;
    // // 0 -> Kick; 1 -> Chip
    // unsigned shoot_mode : 1;
    // // TODO:
    // unsigned trigger_mode : 2;
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
    static constexpr float BATTERY_SCALE_FACTOR = 0.09884f;;

<<<<<<< HEAD
    // Team of the robot (0: Blue) (1: Yellow)
    unsigned team: 1;
    // Id of the robot
    unsigned robot_id : 5;
    // True if the robot has ball sense
=======
    unsigned team: 1;
    unsigned robot_id : 4;
>>>>>>> fe98c66587ef5ba531f37cabaa824ce1906cb754
    unsigned ball_sense_status : 1;
    // is kicking or not
    unsigned kick_status : 1;
    // Kicker is healthy
    unsigned kick_healthy : 1;
    // Battery Voltage
    uint8_t battery_voltage;
    // Error per motor
    unsigned motor_errors : 5;
    // FPGA is working
    unsigned fpga_status : 1;
    // Unused data
    unsigned unused : 2;
    // Encoder deltas
    int16_t encoder_deltas[18];
} __attribute__((packed));

}  // namespace rtp
