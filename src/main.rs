mod game;

use std::io::{self, stdout, Read};
use std::time::Instant;
use termion::raw::IntoRawMode;

fn main() {
    let now = Instant::now();
    let mut level = 1;
    let mut game = game::Game::create_map_player(level);

    'tick: while level < 3 {
        if level > 1 {
            let mut old_player = game.player.clone();
            let old_inventory = game.inventory.clone();
            game = game::Game::create_map_player(level);
            old_player.x = game.player.x;
            old_player.y = game.player.y;
            game.player = old_player;
            game.inventory = old_inventory;
        }

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
                game.turn_actions();
                game.item_interaction();
                game.print();

                if c == 'q' {
                    break 'tick;
                }
                if game.game_status() != 0 {
                    break;
                }
            }
        }
        level += 1;
        // Read in user input without pressing enter each time
        game.print();
        println!("Time {} sec", now.elapsed().as_secs());
    }
}
