mod level;
mod map;
mod object;

extern crate colored;

use crate::game::object::Object;
use colored::*;

#[derive(Clone, Debug)]
pub struct Game {
    map: map::Map,
    player: object::Player,
    objects: object::Objects,
    inventory: object::Object,
    // Game message storing
    message: Vec<String>,
    // Game status 0 for continue, 1 for won, 2 for dead
    status: u8,
}

impl Game {
    pub fn create_map_player(level_number: u32) -> Self {
        Game {
            map: map::read_in_map(level_number),
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
            objects: object::read_in_obj(level_number),
            inventory: Object::empty(),
            message: Vec::new(),
            status: 0,
        }
    }

    /// Stores game console messages to display
    pub fn add_msg(&mut self, msg: String) {
        // Only store up to 4 messages at a time for space efficiency
        while self.message.len() > 4 {
            self.message.remove(0);
        }

        self.message.push(msg);
    }

    pub fn print(&mut self, status: String) {
        print!("\x1B[2J\x1B[1;1H");

        // todo: objects vector to pass to build_map
        for line in map::build_map(
            &self.map,
            &self.player,
            &mut self.objects,
            &mut self.inventory,
            &mut self.message,
            status,
        ) {
            print!("{}\r\n", line);
        }
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
                self.add_msg(String::from("=> Player collided with a wall"));
            }
        } else if dir == 's' {
            // move player down
            temp.move_down(row);
            let player_collision = map::is_collision(&self.map, temp.x, temp.y);
            if !player_collision {
                self.player = temp;
            } else {
                self.add_msg(String::from("=> Player collided with a wall"));
            }
        } else if dir == 'a' {
            // move player left
            temp.move_left();
            let player_collision = map::is_collision(&self.map, temp.x, temp.y);
            if !player_collision {
                self.player = temp;
            } else {
                self.add_msg(String::from("=> Player collided with a wall"));
            }
        } else if dir == 'd' {
            // move player right
            temp.move_right(col);
            let player_collision = map::is_collision(&self.map, temp.x, temp.y);
            if !player_collision {
                self.player = temp;
            } else {
                self.add_msg(String::from("=> Player collided with a wall"));
            }
        }
    }

    /// Checks the tile where the user moved to and checks what kind of interaction it needs to take
    pub fn item_interaction(&mut self) {
        let temp_player = self.player.clone();
        let obj = self.objects.clone();
        for (i, item) in obj.iter().enumerate() {
            // Making sure that we only pick it up when the object is holdable
            if temp_player.x == item.x && temp_player.y == item.y {
                if item.holdable == true {
                    self.inventory = item.clone();
                    self.objects.remove(i);

                    // Print what item we've picked up
                    let msg = "=> You just picked up a/an ".to_owned() + &item.descr;
                    self.add_msg(String::from(msg));
                } else {
                    // Print what item we've picked up
                    let msg = "=> You just encountered a/an ".to_owned() + &item.descr;
                    self.add_msg(String::from(msg));

                    // If level exit has been met
                    if item.descr == "Exit" {
                        self.status = 1;

                        // Print win message
                        self.add_msg(String::from("\n\nYOU WON!!!"));
                    }
                }
            }
        }
    }

    /// Returns the game state 0 -> Continue 1 -> Won 2 -> Dead
    pub fn game_status(&self) -> u8 {
        self.status
    }
}
