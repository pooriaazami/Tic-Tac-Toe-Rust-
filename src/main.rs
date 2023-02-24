use rand::Rng;

#[derive(Eq, PartialEq)]
enum MapCells {
    BALNK,
    X,
    O,
    LOCK,
}

fn initiate_game_map() -> Vec<MapCells> {
    let mut game_map: Vec<MapCells> = vec![];

    for _ in 0..16 {
        game_map.push(MapCells::BALNK);
    }

    game_map
}

fn print_game_map(game_map: &Vec<MapCells>) {
    for i in 0..16 {
        if game_map[i] == MapCells::BALNK {
            print!("_");

            if (i + 1) % 4 == 0 {
                println!();
            }
        }
    }
}

fn main() {
    let mut game_map = initiate_game_map();
    print_game_map(&game_map);

    let mut rng = rand::thread_rng();

}
