#pragma once

#include <cstdint>
#include <string>
#include <vector>

namespace rtp {

struct ControlMessage {
    static constexpr float VELOCITY_SCALE_FACTOR = 1000.0f;

    unsigned robot_id : 5;
    unsigned shoot_mode : 1;
    unsigned trigger_mode : 2;
    int16_t body_x;
    int16_t body_y;
    int16_t body_w;
    int8_t dribbler_speed;
    uint8_t kick_strength;
    unsigned role : 2;
    unsigned unused : 6;
} __attribute__((packed));

struct RobotStatusMessage {
    static constexpr float BATTERY_SCALE_FACTOR = 0.09884f;;

    unsigned robot_id : 5;
    unsigned ball_sense_status : 1;
    unsigned kick_status : 1;
    unsigned kick_healthy : 1;
    uint8_t battery_voltage;
    unsigned motor_errors : 5;
    unsigned fpga_status : 1;
    unsigned unused : 2;
    int16_t encoder_deltas[18];
} __attribute__((packed));

}  // namespace rtp
