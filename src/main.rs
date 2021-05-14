mod map;

use std::io::{self, stdout, Read};
use termion::raw::IntoRawMode;

fn main() {
    //let created = map::read_in_map("map2.txt");

    //map::print_map(&created);

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
        /*
            if c == 'w' {
              // move player up
            }
            else if c == 's' {
              // move player down
            }
            else if c == 'a' {
              // move player left
            }
            else if c == 'd' {
              // move player right
            }
            else if c == 'i' {
              // access inventory
            }
            */
            println!("{}\r", c); 
            if c == 'q' {
                break;
            }
        }
    }
}
