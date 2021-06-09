mod level;
mod map;
mod object;

extern crate colored;

use crate::game::object::Object;

///
/// Game contains map, player all the objects list, player inventory
/// status of the game and game messages (Only store up to 4 messages)
///
#[derive(Clone, Debug)]
pub struct Game {
    pub map: map::Map,
    pub player: object::Player,
    pub objects: object::Objects,
    pub inventory: object::Object,
    // Game message storing
    pub message: Vec<String>,
    // Game status 0 for continue, 1 for won, 2 for dead
    pub status: usize,
}

///
/// constructor for Game struct
///
impl Game {
    pub fn create_map_player(level_number: usize) -> Self {
        let mut temp_player = Object::empty();
        temp_player.init_player();
        Game {
            map: map::read_in_map(level_number),
            player: temp_player,
            objects: object::read_in_obj(level_number),
            inventory: Object::empty(),
            message: Vec::new(),
            status: 0,
        }
    }

    ///
    /// Stores game console messages to display
    /// @self : game object
    /// @msg : holds message to display
    /// @returns : None
    ///
    pub fn add_msg(&mut self, msg: String) {
        // Only store up to 4 messages at a time for space efficiency
        while self.message.len() > 4 {
            self.message.remove(0);
        }

        self.message.push(msg);
    }

    ///
    /// Prints the map onto console
    /// @self : Game object
    /// status : Game status
    /// @retuns : None
    ///
    pub fn print(&mut self) {
        print!("\x1B[2J\x1B[1;1H");

        // todo: objects vector to pass to build_map
        for line in map::build_map(
            &self.map,
            &self.player,
            &mut self.objects,
            &mut self.inventory,
            &mut self.message,
        ) {
            print!("{}\r\n", line);
        }
    }

    ///
    /// Function for moving player in all different directions
    /// @self : game object
    /// @dir : which direction player wants to move
    /// @return : None
    ///
    pub fn player_movement(&mut self, dir: char) {
        //let (row, col) = map::get_row_col(&self.map);
        let (row, col) = (self.map.len(), self.map[0].len());
        let mut temp = self.player.clone();

        match dir {
            'w' => {
                // move player up
                temp.move_up();
                let player_collision = map::is_collision(&self, temp.x, temp.y);
                if !player_collision {
                    self.player = temp;
                } else {
                    self.add_msg(String::from("=> You cannot move there"));
                }
            }
            's' => {
                // move player down
                temp.move_down(row);
                let player_collision = map::is_collision(&self, temp.x, temp.y);
                if !player_collision {
                    self.player = temp;
                } else {
                    self.add_msg(String::from("=> You cannot move there"));
                }
            }
            'a' => {
                // move player left
                temp.move_left();
                let player_collision = map::is_collision(&self, temp.x, temp.y);
                if !player_collision {
                    self.player = temp;
                } else {
                    self.add_msg(String::from("=> You cannot move there"));
                }
            }
            'd' => {
                // move player right
                temp.move_right(col);
                let player_collision = map::is_collision(&self, temp.x, temp.y);
                if !player_collision {
                    self.player = temp;
                } else {
                    self.add_msg(String::from("=> You cannot move there"));
                }
            }
            'q' => {
                self.add_msg(String::from("Player quit the game"));
                self.status = 1;
            }
            _ => self.add_msg(String::from("Invalid key")),
        }
    }

    ///
    /// Checks the tile where the user moved to and checks what kind of interaction it needs to take
    /// @self : Game object
    /// @retuns : None
    ///
    pub fn item_interaction(&mut self) {
        let temp_player = self.player.clone();
        let obj = self.objects.clone();
        let mut inv = self.inventory.clone();
        if self.player.id < 20 {
            self.player.id -= 1;
        }
        for (i, item) in obj.iter().enumerate() {
            // Making sure that we only pick it up when the object is holdable
            if temp_player.x == item.x && temp_player.y == item.y {
                // If it's an item
                if item.holdable {
                    // If player is holding an item
                    if !inv.descr.is_empty() {
                        self.add_msg("=> You've dropped your ".to_owned() + &inv.clone().descr);

                        // Put the object on map into player's inventory and remove it from the map
                        //self.inventory = item.clone();
                        //self.objects.remove(i);

                        // Reposition the coordinates of player's inventory item to current player's position
                        inv.reposition_item(temp_player.x, temp_player.y);
                        // Drop it onto the map by just adding that Object to the objects
                        self.objects.push(inv.clone());

                        self.add_msg("=> You just picked up a/an ".to_owned() + &item.descr);
                    }
                    // If player is not holding an item then just pick it up
                    else {
                        //self.inventory = item.clone();
                        //self.objects.remove(i);

                        // Print what item we've picked up
                        self.add_msg("=> You just picked up a/an ".to_owned() + &item.descr);
                    }

                    self.inventory = item.clone();
                    self.objects.remove(i);
                    self.player.score += &self.inventory.score;
                }
                // If it's an interactable object
                else {
                    // Print what item we've picked up
                    self.add_msg("=> You just encountered a/an ".to_owned() + &item.descr);

                    // If level exit has been met
                    if item.descr == "Exit" {
                        self.status = 1;

                        // Print win message
                        self.add_msg(String::from("\n\nYOU WON!!!"));
                    } else if item.descr == "Fire" {
                        self.add_msg("=> To live longer encounter 'Water' next ".to_string());
                        self.player.id /= 2
                    } else if item.descr == "Water" {
                        if self.player.id < 20 {
                            self.add_msg("=> You got saved, continue playing...".to_string());
                            self.player.id = 20;
                        }
                        self.add_msg("=> That was a one nice bath".to_string());
                    }
                }
            }
        }
    }

    ///
    /// To returns the game state 0 -> Continue 1 -> Won 2 -> Dead
    /// @self : Game status
    /// @returns : status numbers (0/1/2)
    ///
    pub fn game_status(&mut self) -> usize {
        if self.player.id == 0 {
            self.add_msg("=> You burned to a crisp! RIP!".to_string());
            2
        } else {
            self.status
        }
    }


    pub fn turn_actions(&mut self) {
        let objects_iter = &self.objects.clone();
        for (i, object) in objects_iter.iter().enumerate() {
            if self.player.x == object.x && self.player.y == object.y {
                match object.id {
                    1 | 2 => {
                        if self.inventory.id == 3 {
                            self.objects.swap_remove(i);
                        }
                    }
                    80 => {
                        if self.inventory.id == 3 {
                            self.objects.swap_remove(i);
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn game_player_move() {
        // Read test map
        // "┌───┐",
        // "│*Γ.│",
        // "│.@0│",
        // "└───┘",
        let mut game = Game::create_map_player(100);

        // Moving into boulder so shouldn't change coords
        game.player_movement('d');
        assert_eq!((game.player.x, game.player.y), (2, 2));

        game.player_movement('w');
        assert_eq!((game.player.x, game.player.y), (1, 2));

        // Moving into top wall so shouldn't change coords
        game.player_movement('w');
        assert_eq!((game.player.x, game.player.y), (1, 2));

        game.player_movement('a');
        assert_eq!((game.player.x, game.player.y), (1, 1));

        // Moving into left wall so shouldn't change coord
        game.player_movement('a');
        assert_eq!((game.player.x, game.player.y), (1, 1));

        game.player_movement('s');
        assert_eq!((game.player.x, game.player.y), (2, 1));

        // Moving into bottom wall so shouldn't change coords
        game.player_movement('s');
        assert_eq!((game.player.x, game.player.y), (2, 1));
    }

    #[test]
    fn item_pickup_dropoff(){
        // Read test map
        // "┌───┐",
        // "│*Γ.│",
        // "│.@0│",
        // "└───┘",
        let mut game = Game::create_map_player(100);

        // Moved up picked up Pickaxe
        game.player_movement('w');
        game.item_interaction();

        assert_eq!(game.inventory.descr, "Pickaxe");

        // Moved left picked up Potion and dropped Pickaxe to map
        game.player_movement('a');
        game.item_interaction();

        assert_eq!((game.player.x, game.player.y), (1, 1));
        assert_eq!(game.inventory.descr, "Potion");

        assert_eq!(game.objects[0].descr, "A completely average boulder.");
        assert_eq!(game.objects[1].descr, "Pickaxe");

        // Dropped onto where player is currently
        assert_eq!((game.objects[1].x, game.objects[1].y), (1, 1));

        // Moved up and down again and return to (1,1) to pick up Pickaxe and drop Potion
        game.player_movement('s');
        game.item_interaction();
        game.player_movement('w');
        game.item_interaction();

        assert_eq!((game.player.x, game.player.y), (1, 1));
        assert_eq!(game.inventory.descr, "Pickaxe");

        assert_eq!(game.objects[0].descr, "A completely average boulder.");
        assert_eq!(game.objects[1].descr, "Potion");

        // Dropped onto where player is currently
        assert_eq!((game.objects[1].x, game.objects[1].y), (1, 1));
    }
}