use std::fs::File;
use std::io::{self, BufRead};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Please only use one argument of the bugrush board's file name.");
        std::process::exit(1);
    }
    let file_name: &str = &args[1];

    // Code to read a game board from a file
    let mut game_board: Vec<Vec<char>> = Vec::new();
    match read_board(file_name) {
        Ok(result) => {
            game_board = result;
        }
        Err(error) => {
            eprintln!("{}", error);
        }
    }

    if game_board.len() != 4 {
        error("Game board not imported correctly");

    } else if game_board[0].len() != 4 {
            error("Game board not imported correctly");
    }

    // find whos turn it is
    let mut xcount = 0;
    let mut ocount = 0;
    let player;
    let previous_player;
    for r in 0..4 {
        for c in 0..4 {
            if game_board[r][c] == 'X' {
                xcount += 1;
            } else if game_board[r][c] == 'O' {
                ocount += 1;
            }
        }
    }
    if xcount == ocount {
        player = 'X';
        previous_player = 'O';
    } else {
        player = 'O';
        previous_player = 'X';
    }
    // print_board(game_board.clone());
    let result = recurse_board(game_board.clone(), player, previous_player);
    println!("{} {}",player, result);
}

fn recurse_board(board: Vec<Vec<char>>, player: char, previous_player: char) -> i32 {
    let mut result1:i32;
    let mut result3 = i32::MIN;
    let mut finished = true;
    for r in 0..4 {
        for c in 0..4 {
            // Empty spot, add it to possible moves
            if board[r][c] == '.' {
                finished = false;
                let mut new_board = board.clone();
                new_board[r][c] = previous_player;
                //print_board(new_board.clone());
                if previous_player == 'X' {
                    result1 = recurse_board(new_board, player, 'O');
                } else {
                    result1 = recurse_board(new_board, player, 'X');
                }
                if result1 > result3 {
                    result3 = result1;
                }
            }
        }
    }
    if finished {
        return evaluate_board(board.clone(),player);
    }
    result3
}

fn read_board(filename: &str) -> io::Result<Vec<Vec<char>>> {
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);

    // Initialize the 2D vector to store the board
    let mut board: Vec<Vec<char>> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let row: Vec<char> = line.chars().collect();
        board.push(row);
    }

    Ok(board)
}

fn evaluate_board(board: Vec<Vec<char>>, player: char) -> i32 {
    let mut xscore:i32 = 0;
    let mut oscore:i32 = 0;
    for r in 0..4 {
        for c in 0..4 {
            // Check diagonals
            if r > 2 && c <= 2 {
                let mut diagonal1 = Vec::new();
                for i in 0..3 {
                    if c + i < 4 {
                       diagonal1.push(board[r - i][c + i]);
                    }
                }
                if diagonal1.iter().all(|&c| c == 'X') {
                    xscore += 3;
                } else if diagonal1.iter().all(|&c| c == 'O') {
                    oscore += 3;
                }
            }

            if r > 2 && c >= 2 {
                let mut diagonal2 = Vec::new();
                for i in 0..3 {
                    diagonal2.push(board[r - i][c - i]);
                }
                if diagonal2.iter().all(|&c| c == 'X') {
                    xscore += 3;
                } else if diagonal2.iter().all(|&c| c == 'O') {
                    oscore += 3;
                }
            }

            // 3 in a row
            if c > 0 && c < 3 && board[r][c-1] == board[r][c] && board[r][c] == board[r][c+1] {
                if board[r][c] == 'X' {
                    xscore += 3;
                } else {
                    oscore += 3;
                }
            }
            // 3 in a collom
            if r > 0 && r < 3 && board[r-1][c] == board[r][c] && board[r+1][c] == board[r][c] {
                if board[r][c] == 'X' {
                    xscore += 3;
                } else {
                    oscore += 3;
                }
            }
            // Award bonus points
            if r == 0 { // Top row
                if board[r][c] == 'X' {
                    xscore += 1;
                } else {
                    oscore += 1;
                }
            } else if r == 3 { // Bottom row
                if board[r][c] == 'X' {
                    xscore += 1;
                } else {
                    oscore += 1;
                }
            } else if c == 0 { // Left side
                if board[r][c] == 'X' {
                    xscore += 1;
                } else {
                    oscore += 1;
                }
            } else if c == 3 { // Right side
                if board[r][c] == 'X' {
                    xscore += 1;
                } else {
                    oscore += 1;
                }
            }
        }
    }
    if player == 'X' {
        return xscore - oscore;
    } else {
        return oscore - xscore;
    }
}

// fn print_board(board: Vec<Vec<char>>){
//     for row in board {
//         for c in row {
//             print!("{}",c);
//         }
//         println!();
//     }
// }

// Print a usage error message and exit.
fn error(e: &str) -> ! {
    eprintln!("Error: {}", e);
    std::process::exit(1);
}