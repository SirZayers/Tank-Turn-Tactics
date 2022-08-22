mod board;

use board::{
    act, create_local_board, grant_action, update_player, Action,
    Action::{Donate, Log, Players, TankMove, TankShoot, TankUpgrade},
    Board,
    Direction::{Down, Left, Right, Up},
    Player, Request, Response,
};

use openssl::{
    pkey::PKey,
    sign::{Signer, Verifier},
};

// Create board an save in HashMap by board id

fn serve(board: &mut Board, sender_token: u128, action: Action) -> Response {
    let request = Request {
        sender_token,
        action,
    };
    let response = act(board, &request.sender_token, &request.action);
    let request_json = serde_json::to_string(&request).unwrap();
    let response_json = serde_json::to_string(&response).unwrap();
    println!("{:}", request_json);
    println!("\t-> {:}", response_json);
    println!("{:?}\n", board);
    response
}

fn main() {
    let data = serde_json::to_string(&TankMove { direction: Up }).unwrap();

    let private_key =
        PKey::private_key_from_pem(&std::fs::read("keys/private.pem").unwrap()).unwrap();
    let mut signer = Signer::new_without_digest(&private_key).unwrap();
    let signature = signer.sign_oneshot_to_vec(data.as_bytes()).unwrap();

    let public_key = PKey::public_key_from_pem(&std::fs::read("keys/public.pem").unwrap()).unwrap();
    let mut verifier = Verifier::new_without_digest(&public_key).unwrap();
    let valid = verifier
        .verify_oneshot(&signature, data.as_bytes())
        .unwrap();

    println!("message: {:}", data);
    println!("signature: {:?}", signature);
    assert!(valid);

    let mut board = create_local_board(1, 10, 10);
    update_player(
        &mut board,
        Player {
            token: 1,
            nickname: String::from("p1"),
            contact: String::from("p1@localhost"),
            position: (0, 0),
            action_points: 1,
            hit_points: 3,
            range: 2,
        },
    );
    update_player(
        &mut board,
        Player {
            token: 2,
            nickname: String::from("p2"),
            contact: String::from("p2@localhost"),
            position: (5, 5),
            action_points: 1,
            hit_points: 3,
            range: 2,
        },
    );
    println!("{:?}\n", board);

    grant_action(&mut board, 1);
    grant_action(&mut board, 1);
    grant_action(&mut board, 1);
    grant_action(&mut board, 2);
    grant_action(&mut board, 2);
    grant_action(&mut board, 2);
    println!("{:?}\n", board);

    serve(&mut board, 1, Action::Players);
    assert_eq!(
        serve(&mut board, 1, TankMove { direction: Up }),
        Response::Ok
    );
    assert_eq!(
        serve(&mut board, 1, TankMove { direction: Up }),
        Response::Ok
    );
    assert_eq!(
        serve(&mut board, 1, TankMove { direction: Left }),
        Response::MovesOutside,
    );
    assert_eq!(
        serve(&mut board, 1, TankMove { direction: Right }),
        Response::Ok
    );

    assert_eq!(
        serve(&mut board, 2, TankShoot { target_token: 1 }),
        Response::Ok
    );
}
