mod level;
mod map;
mod object;

extern crate colored;

use crate::game::object::Object;
use colored::*;


///
/// Game contains map, player all the objects list, player inventory 
/// status of the game and game messages (Only store up to 4 messages)
///
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

///
/// constructor for Game struct
/// 
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
                "".to_string(),
                0,
            ),
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

    ///
    /// Function for moving player in all different directions 
    /// @self : game object
    /// @dir : which direction player wants to move
    /// @return : None
    ///
    pub fn player_movement(&mut self, dir: char) {
        let (row, col) = map::get_row_col(&self.map);
        let mut temp = self.player.clone();

        match dir { 
            'w' => {
                // move player up
                temp.move_up();
                let player_collision = map::is_collision(&self.map, temp.x, temp.y);
                if !player_collision {
                    self.player = temp;
                } else {
                    self.add_msg(String::from("=> Player collided with a wall"));
                }
            }
            's' => {
                // move player down
                temp.move_down(row);
                let player_collision = map::is_collision(&self.map, temp.x, temp.y);
                if !player_collision {
                    self.player = temp;
                } else {
                    self.add_msg(String::from("=> Player collided with a wall"));
                }
            }
            'a' => {
                // move player left
                temp.move_left();
                let player_collision = map::is_collision(&self.map, temp.x, temp.y);
                if !player_collision {
                    self.player = temp;
                } else {
                    self.add_msg(String::from("=> Player collided with a wall"));
                }
            }
            'd' => {
                // move player right
                temp.move_right(col);
                let player_collision = map::is_collision(&self.map, temp.x, temp.y);
                if !player_collision {
                    self.player = temp;
                } else {
                    self.add_msg(String::from("=> Player collided with a wall"));
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

        for (i, item) in obj.iter().enumerate() {
            // Making sure that we only pick it up when the object is holdable
            if temp_player.x == item.x && temp_player.y == item.y {
                // If it's an item
                if item.holdable {
                    // If player is holding an item
                    if !inv.descr.is_empty() {
                        if inv.descr == "Fire" {
                            self.add_msg("=> Player did not find water and died".to_string());
                            self.status = 2;
                        }
                        else{
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
                        if inv.descr == "Fire"{
                            self.add_msg("=> You can not exit yet ! look for water".to_string());
                        }
                        else{
                            self.status = 1;

                        // Print win message
                        self.add_msg(String::from("\n\nYOU WON!!!"));
                        }
                    }
                    else if item.descr == "Fire" {
                        self.add_msg("=>To live longer encounter 'Water' next ".to_string());
                        self.inventory = item.clone();
                        self.status = 0;
                    }
                    else if item.descr == "Water"{
                        if inv.descr == "Fire"{
                            self.add_msg("=> You got saved, continue playing...".to_string());
                        }
                        else {
                            self.add_msg("=> That was a one nice bath".to_string());
                        }
                        self.status = 0;
                        self.inventory = object::Object::empty();
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
    pub fn game_status(&self) -> u8 {
        self.status
    }
}
