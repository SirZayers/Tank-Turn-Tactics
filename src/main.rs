mod board;


// Create board an save in HashMap by board id

fn main() {
    let board = board::create_board(1, 10, 10, &String::from("./log")).unwrap();

    println!("Test board:\n{:?}", board);
}
