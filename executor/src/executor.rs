//#[derive(Debug, Copy, Clone, PartialEq)]
// pub struct Pose {
//     pub x: i32,
//     pub y: i32,
//     pub heading: char,
// }

// impl Pose {
//     pub fn new(x: i32, y: i32, heading: char) -> Self {
//         Pose { x, y, heading }
//     }
// }

// impl Default for Pose {
//     fn default() -> Self {
//         Pose {
//             x: 0,
//             y: 0,
//             heading: 'N',
//         }
//     }
// }
use crate::pose::Pose;
use super::state::State;
pub struct Executor {
    pose: Pose,
    state: State,
}

impl Executor {
    pub fn with_pose(pose: Pose) -> Self {
        Executor { 
            pose,
            state:State::default(), 
        }
    }

    pub fn execute(&mut self, cmds: &str) {
        for cmd in cmds.chars() {
            match cmd {
                'B' => self.state.be_reverse(),
                'M' => {
                    if self.state.is_reverse{
                        self.pose.forward(-1)
                    } else {
                        self.pose.forward(1)
                    }
                },
                'L' => {
                    if self.state.is_reverse{
                        self.pose.turn_right()
                    } else {
                        self.pose.turn_left()
                    }
                },
                'R' => {
                    if self.state.is_reverse{
                        self.pose.turn_left()
                    } else {
                        self.pose.turn_right()
                    }
                },
                _ => (),
            }
        }
    }

    pub fn query(&self) -> Pose {
        self.pose
    }
}
