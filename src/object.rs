//! object.rs
//!
//! This module takes care of the game's object functionality
//! Object can be interactables on the map or player

type Objects = Vec<Object>;
type Player  = Object;

/// Object struct that carries x,y coordinates
/// Has its own print character and specs that could be interacted
#[derive(Clone, Copy, Debug)]
pub struct Object {
  x: u32,
  y: u32,
  print: char,
  attri: u32,
  mat: u32,
  status: u32,
}

/// Constructor and position mover
impl Object {
  /// Set only position
  pub fn set_pos(new_x:u32, new_y:u32, logo:char) -> Self {
    Object {
      x: new_x,
      y: new_y,
      print: logo,
      attri: 0,
      mat: 0,
      status: 0,
    }
  }
 
  /// Set all the necessary components of an object
  pub fn set_full(new_x:u32, new_y:u32, logo:char, new_att:u32, new_mat:u32, new_stat:u32) -> Self{
    Object {
      x: new_x,
      y: new_y,
      print: logo,
      attri: new_att,
      mat: new_mat,
      status: new_stat,
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
    }
    else{
      self.x -= 1;
    }
  }
  
  /// moving down (x + 1) in the map
  pub fn move_down(&mut self, height:u64) {
    if self.x + 1 >= height as u32 {
      println!("Out of bound!");
    }
    else{
      self.x += 1;
    }
  }

  /// moving left (y - 1) in the map
  pub fn move_left(&mut self) {
    if self.y == 0 {
      println!("Out of bound!");
    }
    else{
      self.y -= 1;
    }
  }

  /// moving right (y + 1) in the map
  pub fn move_right(&mut self, width:u64) {
    if self.y + 1 >= width as u32 {
      println!("Out of bound!");
    }
    else{
      self.y += 1;
    }
  }
}

