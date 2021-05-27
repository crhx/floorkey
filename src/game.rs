mod map;
mod object;

extern crate colored;

use colored::*;

#[derive(Clone, Debug)]
pub struct Game {
    map: map::Map,
    player: object::Player,
}

impl Game {
    pub fn create_map_player(map_name: &str) -> Self {
        Game {
            map: map::read_in_map(map_name),
            player: object::Object::set_pos(2_u32, 2_u32, '@'),
        }
    }

    pub fn print_only_map(&self) {
        map::print_map(&self.map);
    }

    pub fn print(&self) {
        let (row, col) = map::get_row_col(&self.map);

        let mut print_map = vec![vec!['.'; col as usize]; row as usize];

        for x in 0..col {
            for y in 0..row {
                print_map[y as usize][x as usize] = self.map[y as usize][x as usize].print;
            }
        }

        print_map[self.player.x as usize][self.player.y as usize] = '@';

        print!("\x1B[2J\x1B[1;1H");
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
    }

    pub fn player_movement(&mut self, dir: char) {
        let (row, col) = map::get_row_col(&self.map);

        if dir == 'w' {
            // move player up
            self.player.move_up();
        } else if dir == 's' {
            // move player down
            self.player.move_down(col);
        } else if dir == 'a' {
            // move player left
            self.player.move_left();
        } else if dir == 'd' {
            // move player right
            self.player.move_right(row);
        }
    }
}
