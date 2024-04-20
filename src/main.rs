use std::io::stdin;

fn main() {
    let board: [[u8; 3]; 3] = [[0, 0, 0], [0, 0, 0], [0, 0, 0]];

    loop {
        // Draw the board
        for row in board {
            for _ in row {
                print!("*");
            }
            println!();
        }

        let mut key = String::new();
        stdin().read_line(&mut key).unwrap();
    }
}
