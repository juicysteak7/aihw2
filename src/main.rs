use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashMap; 
use std::env;

#[derive(Eq,Hash,PartialEq,Ord,PartialOrd)]
struct BoardState(Vec<Vec<char>>);

fn main() {
    // let args: Vec<String> = env::args().collect();
    // if args.len() != 2 {
    //     eprintln!("Please only use one argument of the bugrush board's file name.");
    //     std::process::exit(1);
    // }
    // let file_name: &str = &args[1];

    // // Code to read a game board from a file
    // let mut game_board: Vec<Vec<char>> = Vec::new();
    // match read_board(file_name) {
    //     Ok(result) => {
    //         game_board = result;
    //     }
    //     Err(error) => {
    //         eprintln!("{}", error);
    //     }
    // }

    // if game_board.len() != 4 {
    //     error("Game board not imported correctly");

    // } else if game_board[0].len() != 4 {
    //         error("Game board not imported correctly");
    // }

    // // find whos turn it is
    // let mut xcount = 0;
    // let mut ocount = 0;
    // let player;
    // let previous_player;
    // for r in 0..4 {
    //     for c in 0..4 {
    //         if game_board[r][c] == 'X' {
    //             xcount += 1;
    //         } else if game_board[r][c] == 'O' {
    //             ocount += 1;
    //         }
    //     }
    // }
    // if xcount == ocount {
    //     player = 'X';
    //     previous_player = 'O';
    // } else {
    //     player = 'O';
    //     previous_player = 'X';
    // }
    // // print_board(game_board.clone());
    // let result = recurse_board(game_board.clone(), player, previous_player,4);
    // println!("{} {}",player, result);
    let mut game_board = vec![vec!['.'; 4]; 4];
    let mut transpoition_table: HashMap<BoardState,i32> = HashMap::new();

    // Create a mutable string to store the user's input
    let mut player_result = String::new();

    // Print a prompt to the user
    println!("Do you want to be X or O? (Enter X or O)");
    // Read user input and store it in the 'input' string
    io::stdin().read_line(&mut player_result).expect("Failed to read line");

    let trimmed_player_input = player_result.trim();

    let mut previous_player = 'X';
    let mut player = 'O';
    if let Some(first_char) = trimmed_player_input.chars().next() {
        if first_char == 'O' {
            previous_player = 'O';
            player = 'X';
            game_board[0][0] = 'X';
        } else if first_char != 'X' {
            error("Please select a valid player choice. (X or O)");
        }
    } else {
        error("Please select a valid player choice. (X or O)");
    }

    // Create a mutable string to store the user's input
    let mut difficulty_result = String::new();

    // Print a prompt to the user
    println!("Please enter you desired difficulty (0 - 10) P.S. 10 may run slowly for the first couple moves.");
    // Read user input and store it in the 'input' string
    io::stdin().read_line(&mut difficulty_result).expect("Failed to read line");

    let trimmed_input = difficulty_result.trim();
    
    let difficulty:usize = match trimmed_input.parse() {
        Ok(n) => n,
        Err(e) => {
            eprintln!("{}",e);
            error("Invalid input for difficulty.");
        }
    };

    // let previous_player = 'X';
    // let player = 'O';
    let mut iterative_depth = 0;
    print_board(game_board.clone());
    while !is_complete(game_board.clone()) {
        // Create a mutable string to store the user's input
        let mut input = String::new();

        // Print a prompt to the user
        println!("Please enter your move (Example: 3 2)");

        // Read user input and store it in the 'input' string
        io::stdin().read_line(&mut input).expect("Failed to read line");

        // Split the input into words and collect them into a Vec
        let words: Vec<&str> = input.split_whitespace().collect();

        // Ensure we have exactly two words
        if words.len() != 2 {
            println!("Please enter exactly two numbers separated by a space.");
            continue;
        }

        // Parse the words into usize variables
        let num1: usize = match words[0].parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Invalid input for the first number.");
                continue;
            }
        };

        let num2: usize = match words[1].parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Invalid input for the second number.");
                continue;
            }
        };

        if num1 >= 4 || num2 >= 4 {
            println!("Please use numbers 0-3");
            continue;
        }

        if game_board[num1][num2] == '.'{
            if player == 'O' {
                game_board[num1][num2] = 'X';
            } else {
                game_board[num1][num2] = 'O';
            }
        } else {
            println!("Please only play on empty spaces");
            continue;
        }

        if difficulty <= 4 {
            game_board = take_step(game_board.clone(), player, previous_player, difficulty + iterative_depth, &mut transpoition_table);
        } else if difficulty < 8 {
            game_board = take_step(game_board.clone(), player, previous_player, difficulty - 4 + iterative_depth, &mut transpoition_table);
            iterative_depth += 1;
        } else {
            game_board = take_step(game_board.clone(), player, previous_player, difficulty + 10 + iterative_depth, &mut transpoition_table);
            iterative_depth += 2;
        }

        print_board(game_board.clone());
    }
    if player == 'X' {
        let result = evaluate_board(game_board.clone(), 'O');
        println!("Your result: {}", result);
    } else {
        let result = evaluate_board(game_board.clone(), 'X');
        println!("Your result: {}", result);
    }
}

fn order_moves(board: Vec<Vec<char>>, player: char) -> Vec<Vec<Vec<char>>>{
    let mut ord_moves:Vec<Vec<Vec<char>>> = Vec::new();

    for r in 1..3 {
        for c in 1..3 {
            if board[r][c] == '.' {
                let mut new_board = board.clone();
                new_board[r][c] = player;
                ord_moves.push(new_board);
            }
        }
    }

    for i in 0..4 {
        if board[0][i] == '.' {
            let mut new_board = board.clone();
            new_board[0][i] = player;
            ord_moves.push(new_board);

        }
        if board[3][i] == '.' {
            let mut new_board = board.clone();
            new_board[3][i] = player;
            ord_moves.push(new_board);           
        }
        if board[i][0] == '.' {
            let mut new_board = board.clone();
            new_board[i][0] = player;
            ord_moves.push(new_board);
        }
        if board[i][3] == '.' {
            let mut new_board = board.clone();
            new_board[i][3] = player;
            ord_moves.push(new_board);
        }
    }

    ord_moves
}

// Recurse board using negamax of depth+1, then evaluate next move.
fn take_step(board: Vec<Vec<char>>, player: char, previous_player: char, depth: usize, transposition_table:&mut HashMap<BoardState,i32>) -> Vec<Vec<char>> {
    //let new_board: Vec<Vec<char>> = Vec::new();
    let mut result:i32;
    let mut result1:i32 = i32::MIN;
    let mut r_board:Vec<Vec<char>> = board.clone();
    for r in 0..4 {
        for c in 0..4 {
            // Empty spot, add it to possible moves
            if board[r][c] == '.' {
                let mut new_board = board.clone();
                new_board[r][c] = player;

                // Alpha = lower bound, Beta = Upper bound
                if previous_player == 'X' {
                    result = recurse_board(new_board.clone(), player, 'O', depth, 0, 2, transposition_table);
                } else {
                    result = recurse_board(new_board.clone(), player, 'X', depth, i32::MIN+1, i32::MAX, transposition_table);
                }
                // println!("{}  {}{}",result,r,c);
                if result > result1 {
                    result1 = result;
                    r_board = new_board.clone();
                }
            }
        }
    }
    r_board
}

fn is_complete(board: Vec<Vec<char>>) -> bool {
    for r in 0..4 {
        for c in 0..4 {
            if board[r][c] == '.' {
                return false;
            }
        }
    }
    true
}

fn recurse_board(board: Vec<Vec<char>>, player: char, previous_player: char, depth: usize, mut alpha: i32, mut beta:i32, transposition_table:&mut HashMap<BoardState,i32>) -> i32 {
    // Alpha beta
    let mut result2 = i32::MIN+1;
    //let mut finished = true;
    let mut result1 = 0;

    for mov in order_moves(board.clone(), player) {
        if let Some(score) = transposition_table.get(&BoardState(mov.clone())) {
            result1= *score;
        } else {
            result1 = -recurse_board(
                mov.clone(),
                player,
                if previous_player == 'X' { 'O' } else { 'X' },
                depth - 1,
                -beta,
                -alpha,
                transposition_table
            );
            //transposition_table.insert(BoardState(new_board.clone()),result1);
        }

        result2 = result2.max(result1);
        alpha = alpha.min(result1);
        beta = beta.max(result1);

        if alpha >= beta {
            // println!("pruned: {} {}", alpha, beta);
            break;
        }
    }
    if depth == 0 {
        return evaluate_board(board, player);
    }
    result2

    // if depth != 0 {
    //     for r in 0..4 {
    //         for c in 0..4 {
    //             if board[r][c] == '.' {
    //                 finished = false;
    //                 let mut new_board = board.clone();
    //                 new_board[r][c] = previous_player;

    //                 if let Some(score) = transposition_table.get(&BoardState(new_board.clone())) {
    //                     result1= *score;
    //                 } else {
    //                     result1 = -recurse_board(
    //                         new_board.clone(),
    //                         player,
    //                         if previous_player == 'X' { 'O' } else { 'X' },
    //                         depth - 1,
    //                         -beta,
    //                         -alpha,
    //                         transposition_table
    //                     );
    //                     //transposition_table.insert(BoardState(new_board.clone()),result1);
    //                 }

    //                 result2 = result2.max(result1);
    //                 alpha = alpha.max(result1);
    //                 beta = beta.min(result1);

    //                 if alpha >= beta {
    //                     // println!("pruned: {} {}", alpha, beta);
    //                     break;
    //                 }
    //             }
    //         }
    //     }
    // }
    // if !transposition_table.contains_key(&BoardState(board.clone())) {
    //     transposition_table.insert(BoardState(board.clone()),result2);
    // }
    // if finished || depth == 0 {
    //     return evaluate_board(board, player);
    // }

    // result2
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
            // if r > 2 && c <= 2 {
            //     let mut diagonal1 = Vec::new();
            //     for i in 0..3 {
            //         if c + i < 4 {
            //            diagonal1.push(board[r - i][c + i]);
            //         }
            //     }
            //     if diagonal1.iter().all(|&c| c == 'X') {
            //         xscore += 3;
            //     } else if diagonal1.iter().all(|&c| c == 'O') {
            //         oscore += 3;
            //     }
            // }

            // if r > 2 && c >= 2 {
            //     let mut diagonal2 = Vec::new();
            //     for i in 0..3 {
            //         diagonal2.push(board[r - i][c - i]);
            //     }
            //     if diagonal2.iter().all(|&c| c == 'X') {
            //         xscore += 3;
            //     } else if diagonal2.iter().all(|&c| c == 'O') {
            //         oscore += 3;
            //     }
            // }

            // Check the diagonal from the top-left corner to the bottom-right corner
            if r + 3 < 4 && c + 3 < 4 {
                let diagonal1: Vec<char> = (0..4).map(|i| board[r + i][c + i]).collect();
                if diagonal1.iter().all(|&c| c == 'X') {
                    xscore += 3;
                } else if diagonal1.iter().all(|&c| c == 'O') {
                    oscore += 3;
                }
            }
            
            // Check the diagonal from the top-right corner to the bottom-left corner
            if r + 3 < 4 && c >= 3 {
                let diagonal2: Vec<char> = (0..4).map(|i| board[r + i][c - i]).collect();
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

fn print_board(board: Vec<Vec<char>>){
    for row in board {
        for c in row {
            print!("{}",c);
        }
        println!();
    }
}

// Print a usage error message and exit.
fn error(e: &str) -> ! {
    eprintln!("Error: {}", e);
    std::process::exit(1);
}