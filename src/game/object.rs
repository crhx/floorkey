//! object.rs
//!
//! This module takes care of the game's object functionality
//! Object can be interactables on the map or player

use crate::game::level::*;
extern crate colored;
use colored::*;

pub type Objects = Vec<Object>;
pub type Player = Object;

///
/// Object struct that carries x,y coordinates
/// Has its own print character and specs that could be interacted
///
#[derive(Clone, Debug)]
pub struct Object {
    pub x: usize,
    pub y: usize,
    pub print: char,
    pub attri: usize,
    pub mat: usize,
    pub status: usize,
    pub quantity: usize,
    pub descr: String,
    pub holdable: bool,
    pub color: String,
    pub print_colored: ColoredString,
    pub paired_item: String,
    pub score: usize,
    pub id: usize,
}

///
/// Constructor and position mover
///
impl Object {
    /// Set all the necessary components of an object
    pub fn init_player(&mut self) {
        self.x = 2;
        self.y = 2;
        self.print = '@';
        self.attri = 1;
        self.mat = 0;
        self.status = 0;
        self.quantity = 1;
        self.descr = String::from("player");
        self.holdable = false;
        self.color = "purple".to_string();
        self.print_colored = "purple".to_string().color("purple");
        self.paired_item = "".to_string();
        self.score = 0;
        self.id = 20;
    }
    /// Creates empty object
    pub fn empty() -> Self {
        Object {
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
        }
    }

    ///
    /// Function tp moving up (x - 1) in the map
    ///
    pub fn move_up(&mut self) {
        if self.x == 0 {
            println!("Out of bound!");
        } else {
            self.x -= 1;
        }
    }

    ///
    /// Function to moving down (x + 1) in the map
    ///
    pub fn move_down(&mut self, height: usize) {
        if self.x + 1 >= height as usize {
            println!("Out of bound!");
        } else {
            self.x += 1;
        }
    }

    ///
    /// Function to moving left (y - 1) in the map
    ///
    pub fn move_left(&mut self) {
        if self.y == 0 {
            println!("Out of bound!");
        } else {
            self.y -= 1;
        }
    }

    ///
    /// Function to moving right (y + 1) in the map
    ///
    pub fn move_right(&mut self, width: usize) {
        if self.y + 1 >= width as usize {
            println!("Out of bound!");
        } else {
            self.y += 1;
        }
    }

    ///
    /// Function to repostion the inventory object when player picks another object
    /// @self : object
    /// @new_x : x co-ordinate value to place the object
    /// @new_y : y co-ordinate value to place the object
    ///
    pub fn reposition_item(&mut self, new_x: usize, new_y: usize) {
        self.x = new_x;
        self.y = new_y;
    }
}

///
/// Function to read the object of specific given level
/// @level_number : game level number
/// @retuns : vector of objects from the given game level
///
pub fn read_in_obj(level_number: usize) -> Objects {
    level(level_number).objects
}
