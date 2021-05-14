mod map;
mod object;

use std::io::{self, stdout, Read};
use termion::raw::IntoRawMode;

fn main() {
    //let created = map::read_in_map("map2.txt");

    //map::print_map(&created);
    let width:u64 = 80;
    let height:u64 = 50;

    let mut player = object::Object::set_pos(2,2,'@');

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
            if c == 'w' {
              // move player up
              player.move_up();
            }
            else if c == 's' {
              // move player down
              player.move_down(height);
            }
            else if c == 'a' {
              // move player left
              player.move_left();
            }
            else if c == 'd' {
              // move player right
              player.move_right(width);
            }
            /*
            else if c == 'i' {
              // access inventory
            }
            */
            
            println!("{}\r", c);
            player.print_pos();

            if c == 'q' {
                break;
            }
        }
    }
}
