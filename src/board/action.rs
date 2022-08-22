use crate::board::Board;
use crate::board::Player;
use serde::{Deserialize, Serialize};
use std::vec::Vec;

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct Request {
    pub sender_token: u128,
    pub action: Action,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub enum Action {
    Players,
    TankMove { direction: Direction },
    TankShoot { target_token: u128 },
    TankUpgrade,
    Donate { target_token: u128 },
    Log { since: u64 },
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub enum Direction {
    Right,
    Up,
    Left,
    Down,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub enum Response {
    Players { players: Vec<ClientPlayer> },
    Ok,
    SenderNotFound,
    SenderNotAlive,
    TargetNotFound,
    TargetNotAlive,
    TargetIsSelf,
    NoActionPoints,
    MovesOutside,
    Log { logs: Vec<LogEntry> },
    Error,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    pub id: u64,
    pub request: Request,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct ClientPlayer {
    token: u128,
    position: (i64, i64),
    contact: String,
    alive: bool,
}

pub fn move_in(direction: &Direction, position: (i64, i64)) -> (i64, i64) {
    match direction {
        Direction::Right => (position.0 + 1, position.1),
        Direction::Up => (position.0, position.1 + 1),
        Direction::Left => (position.0 - 1, position.1),
        Direction::Down => (position.0, position.1 - 1),
    }
}

pub fn is_outside(board: &Board, position: (i64, i64)) -> bool {
    0 > position.0 || position.0 >= board.width || 0 > position.1 || position.1 >= board.height
}

pub fn is_hit(sender: &Player, target: &Player) -> bool {
    let dx = target.position.0 - sender.position.0;
    let dy = target.position.1 - sender.position.1;
    let distance_squared = dx * dx + dy * dy;
    let range = sender.range as i64;
    let range_squared = range * range;
    distance_squared <= range_squared
}

pub fn as_client_player(player: &Player) -> ClientPlayer {
    ClientPlayer {
        token: player.token,
        position: player.position,
        contact: player.contact.clone(),
        alive: is_alive(player),
    }
}

pub fn is_alive(sender: &Player) -> bool {
    sender.hit_points > 0
}

pub fn act(board: &mut Board, sender_token: &u128, action: &Action) -> Response {
    match action {
        Action::Players => {
            let mut players = Vec::new();
            if let Err(_) = players.try_reserve(board.players.len()) {
                return Response::Error;
            }
            players.extend(board.players.values().map(as_client_player));
            Response::Players { players }
        }
        Action::TankMove { direction } => {
            let mut sender = match board.players.get(&sender_token) {
                None => return Response::SenderNotFound,
                Some(sender) => sender.clone(),
            };
            if !is_alive(&sender) {
                return Response::SenderNotAlive;
            }
            sender.action_points -= 1;
            sender.position = move_in(direction, sender.position);
            if sender.action_points < 0 {
                return Response::NoActionPoints;
            }
            if is_outside(board, sender.position) {
                return Response::MovesOutside;
            }
            crate::board::update_player(board, sender);
            Response::Ok
        }
        Action::TankShoot { target_token } => {
            if sender_token == target_token {
                return Response::TargetIsSelf;
            }
            let mut sender = match board.players.get(&sender_token) {
                None => return Response::SenderNotFound,
                Some(sender) => sender.clone(),
            };
            let mut target = match board.players.get(&target_token) {
                None => return Response::TargetNotFound,
                Some(target) => target.clone(),
            };
            if !is_alive(&sender) {
                return Response::SenderNotAlive;
            }
            sender.action_points -= 1;
            if is_alive(&target) && is_hit(&sender, &target) {
                target.hit_points -= 1;
            }
            if sender.action_points < 0 {
                return Response::NoActionPoints;
            }
            crate::board::update_player(board, sender);
            crate::board::update_player(board, target);
            Response::Ok
        }
        Action::TankUpgrade => {
            let mut sender = match board.players.get(&sender_token) {
                None => return Response::SenderNotFound,
                Some(sender) => sender.clone(),
            };
            sender.action_points -= 1;
            sender.range += 1;
            if sender.action_points < 0 {
                return Response::NoActionPoints;
            }
            crate::board::update_player(board, sender);
            Response::Ok
        }
        Action::Donate { target_token } => {
            if sender_token == target_token {
                return Response::TargetIsSelf;
            }
            let mut sender = match board.players.get(&sender_token) {
                None => return Response::SenderNotFound,
                Some(sender) => sender.clone(),
            };
            let mut target = match board.players.get(&target_token) {
                None => return Response::TargetNotFound,
                Some(target) => target.clone(),
            };
            if !is_alive(&sender) {
                return Response::SenderNotAlive;
            }
            if !is_alive(&target) {
                return Response::TargetNotAlive;
            }
            sender.action_points -= 1;
            target.action_points += 1;
            if sender.action_points < 0 {
                return Response::NoActionPoints;
            }
            crate::board::update_player(board, sender);
            crate::board::update_player(board, target);
            Response::Ok
        }
        Action::Log { since } => Response::Log {
            logs: Vec::new(), /* TODO: filter logs from board */
        },
    }
}
