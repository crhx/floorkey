mod game;

use std::io::{self, stdout, Read};
use termion::raw::IntoRawMode;


fn main() {
    let mut game = game::Game::create_map_player("map2.txt");
    game.print();

    // Using termion raw mode
    let _stdout = stdout().into_raw_mode().unwrap();

    // Read in user input without pressing enter each time
    for b in io::stdin().bytes() {
        let b = b.unwrap();
        let c = b as char;

        // if the char is unprintable
        if c.is_control() {
            println!("Unreadable Input! Please try again.");
        } else {
            game.player_movement(c);
            
            game.print();

            if c == 'q' {
                break;
            }
        }
    }

    game.print();
}
