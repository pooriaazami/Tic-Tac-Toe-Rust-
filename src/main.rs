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
    let mut sum: i32 = 0;
    let mut cell = MapCells::BALNK;

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
        }
    }

    if cell != MapCells::BALNK && sum == 3 {
        return cell;
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
        }
    }

    if cell != MapCells::BALNK && sum == 3 {
        return cell;
    }

    let main_diagonal = vec![0, 5, 10, 15];
    let anti_diagonal = vec![3, 6, 9, 12];

    sum = 0;
    cell = MapCells::BALNK;
    main_diagonal.iter().enumerate().for_each(|(i, _)| {
        if cell == game_map[main_diagonal[i]] {
            sum += 1;
        } else {
            sum = 1;
            cell = game_map[main_diagonal[i]];
        }
    });

    if cell != MapCells::BALNK && sum == 3 {
        return cell;
    }

    sum = 0;
    cell = MapCells::BALNK;
    anti_diagonal.iter().enumerate().for_each(|(i, _)| {
        if cell == game_map[anti_diagonal[i]] {
            sum += 1;
        } else {
            sum = 1;
            cell = game_map[anti_diagonal[i]];
        }
    });

    if cell != MapCells::BALNK && sum == 3 {
        return cell;
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

fn main() {
    let mut rng = rand::thread_rng();
    let mut game_map = initiate_game_map(&mut rng);

    let mut win_status = check_winner_status(&game_map);
    while win_status == MapCells::BALNK || win_status == MapCells::LOCK {
        print_game_map(&game_map);
        println!("Enter your move: ");
        let (x, y) = read_user_move();

        println!("x = {}, y = {}", x, y);
        break;
    }
}
