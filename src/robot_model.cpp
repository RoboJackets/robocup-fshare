#include "rc-fshare/robot_model.hpp"

const RobotModel RobotModelControl = []() {
    RobotModel model;
    model.WheelRadius = 0.02768;

    model.WheelAngles = {
        DegreesToRadians(180 - 30),  // M1
        DegreesToRadians(180 + 39),  // M2
        DegreesToRadians(360 - 39),  // M3
        DegreesToRadians(0 + 30),    // M4
    };

    model.WheelDist = 0.0798576;

    model.recalculateBotToWheel();

    return model;
}();
