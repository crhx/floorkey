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
    pub paired_item : String,
    pub score : usize,
		pub id : usize
}

///
/// Constructor and position mover
/// 
impl Object {
    /// Set all the necessary components of an object
    pub fn set_full(
        x: usize,
        y: usize,
        print: char,
        attri: usize,
        mat: usize,
        status: usize,
        quantity: usize,
        descr: String,
        holdable: bool,
        color: String,
        print_colored: ColoredString,
        paired_item : String,
        score : usize,
				id : usize,
    ) -> Self {
        Object {
            x,
            y,
            print,
            attri,
            mat,
            status,
            quantity,
            descr,
            holdable,
            color,
            print_colored,
            paired_item,
            score,
						id,
        }
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
            paired_item : "".to_string(),
            score : 0,
						id : 0,
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
    pub fn move_down(&mut self, height: u64) {
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
    pub fn move_right(&mut self, width: u64) {
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
