//! object.rs
//!
//! This module takes care of the game's object functionality
//! Object can be interactables on the map or player

pub type Objects = Vec<Object>;
pub type Player = Object;
pub type Item = Object;

/// Object struct that carries x,y coordinates
/// Has its own print character and specs that could be interacted
#[derive(Clone, Debug)]
pub struct Object {
    pub x: u32,
    pub y: u32,
    pub print: char,
    pub attri: u32,
    pub mat: u32,
    pub status: u32,
		pub quantity: u32,
		pub descr: String,
}

/// Constructor and position mover
impl Object {

    /// Set all the necessary components of an object
    pub fn set_full(x: u32, 
					y: u32,
					print: char,
					attri: u32, 
					mat: u32, 
					status: u32, 
					quantity: u32,
					descr: String) -> Self {
        Object {
            x,
            y,
            print,
            attri,
            mat,
            status,
			quantity,
			descr,
        	}
    }

    /// Printing object's position
    pub fn print_pos(&self) {
        println!("x: {}, y: {}\r", self.x, self.y);
    }

    /// moving up (x - 1) in the map
    pub fn move_up(&mut self) {
        if self.x == 0 {
            println!("Out of bound!");
        } else {
            self.x -= 1;
        }
    }

    /// moving down (x + 1) in the map
    pub fn move_down(&mut self, height: u64) {
        if self.x + 1 >= height as u32 {
            println!("Out of bound!");
        } else {
            self.x += 1;
        }
    }

    /// moving left (y - 1) in the map
    pub fn move_left(&mut self) {
        if self.y == 0 {
            println!("Out of bound!");
        } else {
            self.y -= 1;
        }
    }

    /// moving right (y + 1) in the map
    pub fn move_right(&mut self, width: u64) {
        if self.y + 1 >= width as u32 {
            println!("Out of bound!");
        } else {
            self.y += 1;
        }
    }
}
