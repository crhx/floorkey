mod map;
mod object;

extern crate colored;

use colored::*;
use crate::game::object::Object;

#[derive(Clone, Debug)]
pub struct Game {
    map: map::Map,
    player: object::Player,
}

impl Game {
    pub fn create_map_player(map_name: &str) -> Self {
        Game {
            map: map::read_in_map(map_name),
            player: object::Object::set_full(
                2_u32,
                2_u32,
                '@',
                1_u32,
                0_u32,
                0_u32,
                1_u32,
                String::from("player"),
            ),
        }
    }

    /*
    // dead_code
        pub fn print_only_map(&self) {
            map::print_map(&self.map);
        }
    */

    pub fn print(&self) {
        /*
                let (row, col) = map::get_row_col(&self.map);

                let mut print_map = vec![vec!['.'; col as usize]; row as usize];

                for x in 0..col {
                    for y in 0..row {
                        print_map[y as usize][x as usize] = self.map[y as usize][x as usize].print;
                    }
                }

                print_map[self.player.x as usize][self.player.y as usize] = '@';
        */
        print!("\x1B[2J\x1B[1;1H");
 
        // todo: objects vector to pass to build_map 
        for line in map::build_map(&self.map, &self.player) {
            print!("{}\r\n", line);
        }
        /*
                for x in print_map.iter() {
                    print!("\r\n");
                    for y in x.iter() {
                        if y == &'W' {
                            print!("{}", y.to_string().red());
                        } else if y == &'.' {
                            print!("{}", y.to_string().green());
                        } else if y == &'@' {
                            print!("{}", y.to_string().purple());
                        } else {
                            print!("{}", y.to_string().white());
                        }
                    }
                }
        */
    }

    pub fn player_movement(&mut self, dir: char) {
        let (row, col) = map::get_row_col(&self.map);
        let mut player_collision = true;
        let mut temp = self.player.clone();

        if dir == 'w' {
            // move player up
            temp.move_up();
            player_collision = map::isCollision(&self.map, temp.x, temp.y);
            if !player_collision
            {
                self.player = temp;
            }
            else{  
                println!("Player collided with an object");
            }
            
        } else if dir == 's' {
            // move player down
            temp.move_down(row);
            player_collision = map::isCollision(&self.map, temp.x, temp.y);
            if !player_collision
            {
                self.player = temp;
            }
            else{  
                println!("Player collided with an object");
            }
            
        } else if dir == 'a' {
            // move player left
            temp.move_left();
            player_collision = map::isCollision(&self.map, temp.x, temp.y);
            if !player_collision
            {
                self.player = temp;
            }
            else{  
                println!("Player collided with an object");
            }
            
        } else if dir == 'd' {
            // move player right
            temp.move_right(col);
            player_collision = map::isCollision(&self.map, temp.x, temp.y);
            if !player_collision
            {
                self.player = temp;
            }
            else{  
                println!("Player collided with an object");
            }
            
        }
    }
}
