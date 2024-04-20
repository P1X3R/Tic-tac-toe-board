use clearscreen;
use std::io::stdin;

fn main() {
    let mut board: [[u8; 3]; 3] = [[0, 0, 0], [0, 0, 0], [0, 0, 0]];
    let mut char: u8 = 1;

    loop {
        clearscreen::clear().unwrap();

        // Draw the board
        for row in board {
            for element in row {
                if element == 1u8 {
                    print!("O");
                } else if element == 2u8 {
                    print!("X");
                } else {
                    print!("*");
                }
            }
            println!(); // Insert end of line
        }

        println!("\nInsert new element [n]");
        println!("Quit [q]");

        println!("\nWhat do you want to do?: ");

        let mut key = String::new();
        stdin().read_line(&mut key).unwrap();

        // Do something on key sended
        match key.trim() {
            "n" => {
                println!("\nWhat is the row what you want to insert?: ");
                let mut row = String::new();
                stdin().read_line(&mut row).unwrap();
                let row: usize = row.trim().parse().unwrap(); // Transform string to int (u8)

                println!("\nWhat is the column what you want to insert?: ");
                let mut column = String::new();
                stdin().read_line(&mut column).unwrap();
                let column: usize = column.trim().parse().unwrap(); // Transform string to int (u8)

                board[row][column] = char;

                // Change the character
                if char == 1 {
                    char = 2;
                } else {
                    char = 1;
                }
            }
            "q" => break,
            _ => println!("Invalid key"),
        }
    }
}
