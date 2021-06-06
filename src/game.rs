mod map;
mod object;
mod level;

extern crate colored;

use crate::game::object::Object;
use colored::*;

#[derive(Clone, Debug)]
pub struct Game {
    map: map::Map,
    player: object::Player,
    objects: object::Objects,
    inventory: object::Object,
}

impl Game {
    pub fn create_map_player(map_name: &str) -> Self {
        Game {
            map: map::read_in_map(1),
            player: object::Object::set_full(
                2_u32,
                2_u32,
                '@',
                1_u32,
                0_u32,
                0_u32,
                1_u32,
                String::from("player"),
                false,
                "purple".to_string(),
                "purple".to_string().color("purple"),
            ),
            objects: Vec::new(),
            inventory: Object::empty(),
        }
    }

    /*
    // dead_code
        pub fn print_only_map(&self) {
            map::print_map(&self.map);
        }
    */

    pub fn print(&mut self, status: String) {
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
        for line in map::build_map(&self.map, &self.player, &mut self.objects, status) {
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
        // moved this to local scope to remove warning
        // let mut player_collision = true;
        let mut temp = self.player.clone();

        if dir == 'w' {
            // move player up
            temp.move_up();
            let player_collision = map::is_collision(&self.map, temp.x, temp.y);
            if !player_collision {
                self.player = temp;
            } else {
                println!("Player collided with an object");
            }
        } else if dir == 's' {
            // move player down
            temp.move_down(row);
            let player_collision = map::is_collision(&self.map, temp.x, temp.y);
            if !player_collision {
                self.player = temp;
            } else {
                println!("Player collided with wall");
            }
        } else if dir == 'a' {
            // move player left
            temp.move_left();
            let player_collision = map::is_collision(&self.map, temp.x, temp.y);
            if !player_collision {
                self.player = temp;
            } else {
                println!("Player collided with wall");
            }
        } else if dir == 'd' {
            // move player right
            temp.move_right(col);
            let player_collision = map::is_collision(&self.map, temp.x, temp.y);
            if !player_collision {
                self.player = temp;
            } else {
                println!("Player collided with wall");
            }
        }
    }

    pub fn item_interaction(&mut self) {
        let temp_player = self.player.clone();
        let obj = self.objects.clone();
        for (i, item) in obj.iter().enumerate() {
            if temp_player.x == item.x && temp_player.y == item.y {
                self.inventory = item.clone();
                self.objects.remove(i);
            }
        }
    }
}
