use std::io;

use rand::{rngs::ThreadRng, Rng};

#[derive(Eq, PartialEq, Copy, Clone)]
enum MapCells {
    BALNK,
    X,
    O,
    LOCK,
}

fn initiate_game_map(rng: &mut ThreadRng) -> Vec<MapCells> {
    let mut game_map: Vec<MapCells> = vec![];

    for _ in 0..16 {
        game_map.push(MapCells::BALNK);
    }

    for _ in 0..3 {
        let index = rng.gen_range(0..16);

        game_map[index] = MapCells::LOCK;
    }

    game_map
}

fn print_game_map(game_map: &Vec<MapCells>) {
    for i in 0..16 {
        match game_map[i] {
            MapCells::BALNK => print!("_"),
            MapCells::X => print!("X"),
            MapCells::O => print!("O"),
            MapCells::LOCK => print!("*"),
        }

        if (i + 1) % 4 == 0 {
            println!();
        }
    }
}

fn check_winner_status(game_map: &Vec<MapCells>) -> MapCells {
    let mut sum: i32;
    let mut cell: MapCells;

    for i in 0..4 {
        sum = 0;
        cell = MapCells::BALNK;

        for j in 0..4 {
            let idx = i * 4 + j;
            if cell == game_map[idx] {
                sum += 1;
            } else {
                sum = 1;
                cell = game_map[idx];
            }

            if cell != MapCells::BALNK && sum == 3 {
                return cell;
            }
        }
    }

    for j in 0..4 {
        sum = 0;
        cell = MapCells::BALNK;

        for i in 0..4 {
            let idx = i * 4 + j;
            if cell == game_map[idx] {
                sum += 1;
            } else {
                sum = 1;
                cell = game_map[idx];
            }

            if cell != MapCells::BALNK && sum == 3 {
                return cell;
            }
        }
    }

    for j in 0..4 {
        sum = 0;
        cell = MapCells::BALNK;

        for i in 0..4 {
            if i != j{
                continue;
            }
            let idx = i * 4 + j;

            if cell == game_map[idx] {
                sum += 1;
            } else {
                sum = 1;
                cell = game_map[idx];
            }

            if cell != MapCells::BALNK && sum == 3 {
                return cell;
            }
        }
    }

    for j in 0..4 {
        sum = 0;
        cell = MapCells::BALNK;

        for i in 0..4 {
            if i + j != 3{
                continue;
            }
            let idx = i * 4 + j;

            if cell == game_map[idx] {
                sum += 1;
            } else {
                sum = 1;
                cell = game_map[idx];
            }

            if cell != MapCells::BALNK && sum == 3 {
                return cell;
            }
        }
    }

    return MapCells::BALNK;
}

fn parse_string_to_integer(string: &str) -> i32 {
    let res = string.to_owned().parse::<i32>();
    // println!("[debug]: {}", string);
    match res {
        Ok(num) => num,
        Err(_) => -1,
    }
}

fn read_user_move() -> (i32, i32) {
    let mut user_input = String::new();

    io::stdin()
        .read_line(&mut user_input)
        .expect("Thre was an error reading the input");

    let numbers: Vec<&str> = user_input.trim().split(" ").collect();
    if numbers.len() != 2 {
        (-1, -1)
    } else {
        (
            parse_string_to_integer(numbers[0]),
            parse_string_to_integer(numbers[1]),
        )
    }
}

fn step(game_map: &mut Vec<MapCells>, x: i32, y: i32, turn: MapCells) -> bool {
    let idx = (x * 4 + y) as usize;
    if game_map[idx] == MapCells::BALNK {
        game_map[idx] = turn;
        true
    } else {
        false
    }
}

fn main() {
    let mut rng = rand::thread_rng();
    let mut game_map = initiate_game_map(&mut rng);

    let mut win_status = check_winner_status(&game_map);
    let mut turn = MapCells::X;

    print_game_map(&game_map);
    let mut counter = 0;
    while win_status == MapCells::BALNK || win_status == MapCells::LOCK {
        counter += 1;

        if counter == 16 {
            break;
        }

        println!("Enter your move: ");
        let (x, y) = read_user_move();

        let res = step(&mut game_map, x, y, turn);

        // println!("x = {}, y = {}", x, y);
        turn = if turn == MapCells::X {
            MapCells::O
        } else {
            MapCells::X
        };
        if !res {
            println!("Illegal move!, the winner is: {}", {
                if turn == MapCells::X {
                    "X"
                } else {
                    "O"
                }
            });
            break;
        }

        print_game_map(&game_map);
        win_status = check_winner_status(&game_map);
    }

    println!("The winner is: {}", {
        if win_status == MapCells::X {
            "X"
        } else {
            "O"
        }
    });
}
