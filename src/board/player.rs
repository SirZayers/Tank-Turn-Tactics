use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    pub token: u128,
    pub nickname: String,
    pub contact: String,
    pub position: (i64, i64),
    pub action_points: i32,
    pub hit_points: i32,
    pub range: u64,
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(
            f,
            "token: {}\n
        nickname: {}\n
        contact: {}\n
        position: {:?}\n
        action points: {}\n
        hit points: {}\n
        range: {}",
            self.token,
            &self.nickname,
            &self.contact,
            self.position,
            self.action_points,
            self.hit_points,
            self.range,
        )
    }
}
