//! map.rs
//!
//! This module takes care of the game's map functionality
//! From creating empty map, read in .txt file to create map, to printing

// Crate for printing color in terminal
extern crate colored;

use colored::*;

// File reading
use std::fs;

// Map is made up of Tile x Tile
type Map = Vec<Vec<Tile>>;

/// Map is created with Tile by Tile
/// Each Tile has character to display, blocked, visited to display partial map
#[derive(Clone, Copy, Debug)]
pub struct Tile {
    print: char,
    blocked: bool,
    visited: bool,
}

/// Constructor for Tile struct
impl Tile {
    /// Creating empty Tile
    pub fn empty() -> Self {
        Tile {
            print: '.',
            blocked: false,
            visited: false,
        }
    }

    /// Creating wall Tile
    pub fn wall() -> Self {
        Tile {
            print: 'W',
            blocked: true,
            visited: false,
        }
    }

    /// Printing self
    #[allow(dead_code)]
    pub fn print(&self) {
        print!("{}", self.print);
    }
}

/// Takes in Height and Width of the map and creates a map then returns it
#[allow(dead_code)]
pub fn make_map(x: u64, y: u64) -> Map {
    let map = vec![vec![Tile::empty(); x as usize]; y as usize];

    // Algorithm to procedurally generate map

    map
}

/// Function that takes in Map type and prints the base map of it
#[allow(dead_code)]
pub fn print_map(map: Map) {
    // Iterate through map and print wall with red and empty with green
    for x in map.iter() {
        for y in x.iter() {
            let tile = y.print;

            if tile == 'W' {
                print!("{}", tile.to_string().red());
            } else {
                print!("{}", tile.to_string().green());
            }
        }
        println!();
    }
}

/// Function that takes in file name and create map from read in text file
/// Returns the created map
#[allow(dead_code)]
pub fn read_in_map(name: &str) -> Map {
    // Read in the file in string form
    let maptext = fs::read_to_string(name).expect("Something went wrong reading the file");

    // Count how many newline char and calculate rows and columns
    let row = maptext.matches('\n').count();
    let col = (maptext.len() - row) / row;

    // Create all empty tile map
    let mut map = vec![vec![Tile::empty(); col as usize]; row as usize];

    let mut i = 0;
    let mut j = 0;

    // Iterate through string and modify the all empty map
    for c in maptext.chars() {
        if c == '1' {
            map[i][j] = Tile::wall();
            j += 1;
        } else if c == '\n' {
            i += 1;
            j = 0;
        } else {
            j += 1;
        }
    }

    map
}
