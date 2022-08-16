use std::fmt;
use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    token: u128,
    nickname: String,
    contact: String,
    position: (u64, u64),
    action_points: u32,
    range: u8,
    alive: bool
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, 
        "token: {}\n
        nickname: {}\n
        contact: {}\n
        position: {:?}\n
        action points: {}\n
        range: {}\n
        alive: {}",
    self.token, &self.nickname, &self.contact, self.position, self.action_points, self.range, self.alive)
    }
}
