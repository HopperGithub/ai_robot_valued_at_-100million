#![warn(missing_docs)]
//! # ai_robot_valued_at_100million

pub mod robot;

fn main() {
    let robot = robot::Robot::new();
    robot.listen()
}
