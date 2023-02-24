use rand::{rngs::ThreadRng, Rng};

#[derive(Eq, PartialEq)]
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

fn main() {
    let mut rng = rand::thread_rng();
    let mut game_map = initiate_game_map(&mut rng);
    print_game_map(&game_map);
}
