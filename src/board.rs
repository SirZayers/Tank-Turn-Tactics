mod action;
mod player;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::fmt::Write as fmtWrite;
use std::fs::OpenOptions;
use std::io::Write;

pub use crate::board::action::{act, Action, ClientPlayer, Direction, LogEntry, Request, Response};
pub use crate::board::player::Player;

#[derive(Debug, Serialize, Deserialize)]
pub struct Board {
    id: u128,
    width: i64,
    height: i64,
    logfile: String,
    lastlog: bool,
    has_started: bool,
    has_ended: bool,
    winner: Option<Player>,
    pub players: HashMap<u128, Player>,
    pub logs: Vec<LogEntry>,
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match write!(
            f,
            "BOARD\n
        id: {}\n
        height: {}\n
        width: {}\n
        logfile: {}\n
        has_started: {}\n
        has_ended: {}\n
        winner: {:?}\n",
            self.id,
            self.width,
            self.height,
            &self.logfile,
            self.has_started,
            self.has_ended,
            &self.winner
        ) {
            Ok(_) => (),
            Err(e) => return Err(e),
        };

        for (_, player) in &self.players {
            match writeln!(format!("{player}")) {
                Ok(_) => (),
                Err(e) => return Err(e),
            };
        }
        Ok(())
    }
}

pub fn create_board(
    id: u128,
    width: i64,
    height: i64,
    logfilepath: &String,
) -> Result<Board, String> {
    // Create two logfiles named <path>.id.(1/2).log if they don't exist yet

    let mut logfile0 = match OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(format!("{}.{}.0.log", logfilepath, id))
    {
        Err(_) => {
            return Err(format!(
                "Error: Logfile {}.{}.0.log couldn't be created or already exists.",
                logfilepath, id
            ))
        }
        Ok(f) => f,
    };

    let mut logfile1 = match OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(format!("{}.{}.1.log", logfilepath, id))
    {
        Err(_) => {
            return Err(format!(
                "Error: Logfile {}.{}.1.log couldn't be created or already exists.",
                logfilepath, id
            ))
        }
        Ok(f) => f,
    };

    let newboard = Board {
        id,
        width,
        height,
        logfile: logfilepath.clone(),
        lastlog: true,
        has_ended: false,
        has_started: false,
        winner: None,
        players: HashMap::new(),
        logs: Vec::new(),
    };

    // Fill log files with board information

    match logfile0.write_all(serde_json::to_string(&newboard).unwrap().as_bytes()) {
        Err(_) => {
            return Err(format!(
                "Error: write to {}.{}.0.log failed.",
                logfilepath, id
            ))
        }
        Ok(_) => (),
    };

    match logfile1.write_all(serde_json::to_string(&newboard).unwrap().as_bytes()) {
        Err(_) => {
            return Err(format!(
                "Error: write to {}.{}.1.log failed.",
                logfilepath, id
            ))
        }
        Ok(_) => (),
    };

    return Ok(newboard);
}

pub fn create_local_board(id: u128, width: i64, height: i64) -> Board {
    return Board {
        id,
        width,
        height,
        logfile: String::from(""),
        lastlog: true,
        has_ended: false,
        has_started: false,
        winner: None,
        players: HashMap::new(),
        logs: Vec::new(),
    };
}

pub fn update_player(board: &mut Board, player: Player) {
    board.players.insert(player.token, player);
}

pub fn grant_action(board: &mut Board, player_token: u128) {
    let mut sender = match board.players.get_mut(&player_token) {
        None => return,
        Some(sender) => sender,
    };
    sender.action_points += 1;
}
