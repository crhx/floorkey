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
        let (row, col) = (self.map.len(), self.map[0].len());
        let mut temp = self.player.clone();

        match dir {
            'w' => {
                // move player up
                temp.move_up();
                let player_collision = map::is_collision(self, temp.x, temp.y);
                if !player_collision {
                    self.player = temp;
                } else {
                    self.add_msg(String::from("=> You cannot move there"));
                }
            }
            's' => {
                // move player down
                temp.move_down(row);
                let player_collision = map::is_collision(self, temp.x, temp.y);
                if !player_collision {
                    self.player = temp;
                } else {
                    self.add_msg(String::from("=> You cannot move there"));
                }
            }
            'a' => {
                // move player left
                temp.move_left();
                let player_collision = map::is_collision(self, temp.x, temp.y);
                if !player_collision {
                    self.player = temp;
                } else {
                    self.add_msg(String::from("=> You cannot move there"));
                }
            }
            'd' => {
                // move player right
                temp.move_right(col);
                let player_collision = map::is_collision(self, temp.x, temp.y);
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

                        // Reposition the coordinates of player's inventory item to current player's position
                        inv.reposition_item(temp_player.x, temp_player.y);
                        // Drop it onto the map by just adding that Object to the objects
                        self.objects.push(inv.clone());

                        self.add_msg("=> You just picked up a/an ".to_owned() + &item.descr);
                    }
                    // If player is not holding an item then just pick it up
                    else {
                        // Print what item we've picked up
                        self.add_msg("=> You just picked up a/an ".to_owned() + &item.descr);
                    }

                    // Put the object on map into player's inventory and remove it from the map
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
                    85 => {
                        if self.inventory.id == 86 {
                            self.add_msg("Old man: *gluf gluf*".to_string());
                            self.add_msg("Old man: \"I guess I'll come with you.".to_string());
                            self.inventory = Object::empty();
                        }
                    }
                    87 => {
                        if self.inventory.id == 85 {
                            self.add_msg("Old man: \"Aaaaaaahhhhh.\"".to_string());
                            self.add_msg("Old man: *thud*".to_string());
                            self.add_msg("=> The terrain has been smoothend".to_string());
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
    fn item_pickup_dropoff() {
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

    #[test]
    fn test_message_when_player_collide_with_wall() {
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

        //Moved right towards wall
        game.player_movement('d');
        assert_eq!((game.player.x, game.player.y), (1, 3));

        // Moved right towards wall
        game.player_movement('d');
        assert_eq!((game.player.x, game.player.y), (1, 3));

        assert_eq!(
            (game.message[1]),
            (String::from("=> You cannot move there"))
        );
    }

    #[test]
    fn test_player_encounter_fire_and_burn() {
        // Read test map
        // "┌─────┐",
        // "│*Γ..≌│",
        // "│.@0..│",
        // "└─────┘",
        let mut game = Game::create_map_player(101);
        // Moved up encountered fire
        game.player_movement('d');
        game.item_interaction();

        assert_eq!(
            (game.message[0]),
            (String::from("=> You just encountered a/an Fire"))
        );
        assert_eq!(
            (game.message[1]),
            (String::from("=> To live longer encounter 'Water' next "))
        );

        // Keep encountering fire for 3 times without finding water
        game.player_movement('a');
        game.player_movement('d');
        game.item_interaction();
        game.player_movement('a');
        game.player_movement('d');
        game.item_interaction();
        game.player_movement('a');
        game.player_movement('d');
        game.item_interaction();

        // Moved right towards wall
        game.player_movement('d');
        assert_eq!((game.player.id), (0));

        let status = game.game_status();
        assert_eq!((status), (2));
        assert_eq!(
            (game.message[4]),
            (String::from("=> You burned to a crisp! RIP!"))
        );
    }

    #[test]
    fn test_player_encounter_water_when_burning() {
        // Read test map
        // "┌─────┐",
        // "│*Γ..≌│",
        // "│█@Ж..│",
        // "└─────┘",
        let mut game = Game::create_map_player(101);

        assert_eq!((game.player.x, game.player.y), (2, 2));

        // Moved left encountered fire
        game.player_movement('d');
        game.item_interaction();

        assert_eq!(
            (game.message[0]),
            (String::from("=> You just encountered a/an Fire"))
        );
        assert_eq!(
            (game.message[1]),
            (String::from("=> To live longer encounter 'Water' next "))
        );

        let expected_player_id = 10;
        assert_eq!((game.player.id), (expected_player_id));
        assert_eq!((game.player.x, game.player.y), (2, 3));

        game.player_movement('d');
        game.item_interaction();
        assert_eq!((game.player.x, game.player.y), (2, 4));

        game.player_movement('w');
        game.item_interaction();
        assert_eq!((game.player.x, game.player.y), (1, 4));

        game.player_movement('d');
        game.item_interaction();

        assert_eq!(
            (game.message[2]),
            (String::from("=> You just encountered a/an Water"))
        );
        assert_eq!(
            (game.message[3]),
            (String::from("=> You got saved, continue playing..."))
        );

        let actual_status = game.game_status();
        let expected_status = 0;
        assert_eq!((actual_status), (expected_status));
>>>>>>> f8b045780e4aee315c44ef208e94ac2d9f7b55a4
    }

    #[test]
    fn test_game_status_when_player_continues() {
        // Read test map
        // "┌───┐",
        // "│*Γ.│",
        // "│.@0│",
        // "└───┘",
        let mut game = Game::create_map_player(100);
        // Moved up encountered boulder
        game.player_movement('d');
        game.item_interaction();

        assert_eq!(
            (game.message[0]),
            (String::from("=> You cannot move there"))
        );

        let actual_status = game.game_status();
        let expected_status = 0;
        assert_eq!((actual_status), (expected_status));
    }

    #[test]
    fn test_player_cannot_move_to_boulder() {
        // Read test map
        // "┌───┐",
        // "│*Γ.│",
        // "│.@0│",
        // "└───┘",
        let mut game = Game::create_map_player(100);
        // Moved up encountered boulder
        game.player_movement('d');
        game.item_interaction();

        assert_eq!(
            (game.message[0]),
            (String::from("=> You cannot move there"))
        );
    }

    #[test]
    fn test_player_can_break_boulder_with_pickasa() {
        // Read test map
        // "┌───┐",
        // "│*Γ.│",
        // "│.@0│",
        // "└───┘",
        let mut game = Game::create_map_player(100);
        // Moved up encountered boulder
        game.player_movement('w');
        game.item_interaction();

        assert_eq!(game.inventory.descr, "Pickaxe");

        game.player_movement('d');
        game.player_movement('s');
        game.item_interaction();

        assert_eq!(
            (game.message[0]),
            (String::from("=> You just picked up a/an Pickaxe"))
        );
        assert_eq!(
            (game.message[1]),
            (String::from("=> You just encountered a/an A completely average boulder."))
        );
    }

    #[test]
    fn test_player_cannot_open_dore() {
        // Read test map
        // "┌─────┐",
        // "│*Γ..≌│",
        // "│█@Ж..│",
        // "└─────┘",
        let mut game = Game::create_map_player(101);

        // Moved left to open dore without Pickaxe
        game.player_movement('a');
        game.item_interaction();

        //cant open the dore
        assert_eq!(
            (game.message[0]),
            (String::from("=> You cannot move there"))
        );
    }

    #[test]
    fn test_player_opens_dore_with_pickaxe() {
        // Read test map
        // "┌─────┐",
        // "│*Γ..≌│",
        // "│█@Ж..│",
        // "└─────┘",
        let mut game = Game::create_map_player(101);

        // Moved left to open dore without Pickaxe
        game.player_movement('a');
        game.item_interaction();

        //cant open the dore
        assert_eq!(
            (game.message[0]),
            (String::from("=> You cannot move there"))
        );

        game.player_movement('w');
        game.item_interaction();
        assert_eq!(
            (game.message[1]),
            (String::from("=> You just picked up a/an Pickaxe"))
        );

        // Now player can open the dore since he has Pickaxe
        game.player_movement('s');
        game.player_movement('a');
        game.item_interaction();
        assert_eq!(
            (game.message[3]),
            (String::from("=> You just picked up a/an Door"))
        );
    }

    #[test]
    fn test_player_score_when_enounter_non_objects() {
        // Read test map
        // "┌─────┐",
        // "│*Γ..≌│",
        // "│█@Ж..│",
        // "└─────┘",
        let mut game = Game::create_map_player(101);

        // Moved left to open dore without Pickaxe
        game.player_movement('a');
        game.item_interaction();

        //cant open the dore
        assert_eq!(
            (game.message[0]),
            (String::from("=> You cannot move there"))
        );

        let expected_score = 0;

        assert_eq!((game.player.score), (expected_score));

        // Moved left to open dore without Pickaxe
        game.player_movement('s');
        game.item_interaction();

        //encountered a wall
        assert_eq!(
            (game.message[1]),
            (String::from("=> You cannot move there"))
        );

        assert_eq!((game.player.score), (expected_score));
    }

    #[test]
    fn test_player_score_when_enounter_objects() {
        // Read test map
        // "┌─────┐",
        // "│*Γ..≌│",
        // "│█@Ж..│",
        // "└─────┘",
        let mut game = Game::create_map_player(101);

        // Moved left to open dore without Pickaxe
        game.player_movement('a');
        game.item_interaction();

        //cant open the dore
        assert_eq!(
            (game.message[0]),
            (String::from("=> You cannot move there"))
        );

        let mut expected_score = 0;

        assert_eq!((game.player.score), (expected_score));

        // Moved left to open dore without Pickaxe
        game.player_movement('w');
        game.item_interaction();

        expected_score = 20;

        //encountered a Pickaxe
        assert_eq!(
            (game.message[1]),
            (String::from("=> You just picked up a/an Pickaxe"))
        );

        assert_eq!((game.player.score), (expected_score));
    }

    #[test]
    fn test_player_drops_item_and_pick_another_item() {
        // Read test map
        // "┌─────┐",
        // "│*Γ..≌│",
        // "│█@Ж..│",
        // "└─────┘",
        let mut game = Game::create_map_player(101);

        // Moved up to pick up Pickaxe
        game.player_movement('w');
        game.item_interaction();

        assert_eq!(
            (game.message[0]),
            (String::from("=> You just picked up a/an Pickaxe"))
        );

        game.player_movement('a');
        game.item_interaction();
        assert_eq!(
            (game.message[1]),
            (String::from("=> You've dropped your Pickaxe"))
        );
        assert_eq!(
            (game.message[2]),
            (String::from("=> You just picked up a/an Potion"))
        );
    }

    #[test]
    fn test_add_msg() {
        // Read test map
        // "┌─────┐",
        // "│*Γ..≌│",
        // "│█@Ж..│",
        // "└─────┘",
        let mut game = Game::create_map_player(101);

        // Moved up to pick up Pickaxe
        game.player_movement('w');
        game.item_interaction();

        assert_eq!(
            (game.message[0]),
            (String::from("=> You just picked up a/an Pickaxe"))
        );

        let actual_msg_len = game.message.len();
        let expected_msg_len = 1;

        assert_eq!((actual_msg_len), (expected_msg_len));
    }

    #[test]
    fn test_cannot_add_more_than_5_msg() {
        // Read test map
        // "┌─────┐",
        // "│*Γ..≌│",
        // "│█@Ж..│",
        // "└─────┘",
        let mut game = Game::create_map_player(101);

        game.add_msg(String::from("First message"));
        game.add_msg(String::from("Second message"));
        game.add_msg(String::from("Third message"));
        game.add_msg(String::from("Fourth message"));
        game.add_msg(String::from("Fifth message"));
        game.add_msg(String::from("Sixth message"));

        let expected_msg_at_0 = String::from("Second message");

        assert_eq!((game.message[0]), (expected_msg_at_0));
    }

    #[test]
    fn test_create_empty_object() {
        use crate::game::colored::Colorize;
        let expected_temp_player = Object {
            x: 0,
            y: 0,
            print: '0',
            attri: 0,
            mat: 0,
            status: 0,
            quantity: 0,
            descr: "".to_string(),
            holdable: false,
            color: "green".to_string(),
            print_colored: '0'.to_string().color("green"),
            paired_item: "".to_string(),
            score: 0,
            id: 0,
        };

        assert_eq!((Object::empty()), (expected_temp_player));
    }

    #[test]
    fn test_initialize_player_attributes() {
        use crate::game::colored::Colorize;
        let mut expected_temp_player = Object {
            x: 0,
            y: 0,
            print: '0',
            attri: 0,
            mat: 0,
            status: 0,
            quantity: 0,
            descr: "".to_string(),
            holdable: false,
            color: "green".to_string(),
            print_colored: '0'.to_string().color("green"),
            paired_item: "".to_string(),
            score: 0,
            id: 0,
        };

        expected_temp_player.init_player();

        assert_eq!(expected_temp_player.descr, "player");
    }

    #[test]
    #[should_panic]
    fn test_create_map_panic_screnario() {
        // we dont have level 102, when tried to create map for level 102 cargp panics
        let _game = Game::create_map_player(102);
    }

    #[test]
    fn test_invalid_key_press() {
        // Read test map
        // "┌─────┐",
        // "│*Γ..≌│",
        // "│█@Ж..│",
        // "└─────┘",
        let mut game = Game::create_map_player(101);

        // Moved up to pick up Pickaxe
        game.player_movement('z');
        //game.item_interaction();

        assert_eq!((game.message[0]), (String::from("Invalid key")));
    }

    #[test]
    fn test_game_quit() {
        // Read test map
        // "┌─────┐",
        // "│*Γ..≌│",
        // "│█@Ж..│",
        // "└─────┘",
        let mut game = Game::create_map_player(101);

        // Moved up to Fire
        game.player_movement('d');
        game.item_interaction();

        //Scared and press q to quit the game
        game.player_movement('q');

        assert_eq!((game.message[2]), (String::from("Player quit the game")));
>>>>>>> f8b045780e4aee315c44ef208e94ac2d9f7b55a4
    }
}
