mod game;

use std::io::{self, stdout, Read};
use termion::raw::IntoRawMode;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let mut game = game::Game::create_map_player(1);
    game.print();
    // Using termion raw mode
    let _stdout = stdout().into_raw_mode().unwrap();

    for b in io::stdin().bytes() {
        let b = b.unwrap();
        let c = b as char;

        // if the char is unprintable
        if c.is_control() {
            println!("Unreadable Input! Please try again.");
        } else {
            game.player_movement(c);
            game.item_interaction();
            game.print();

            if c == 'q' || game.game_status() != 0 {
                break;
            }
        }
    }
    // Read in user input without pressing enter each time
    game.print();
    println!("Time {} sec", now.elapsed().as_secs());
}
