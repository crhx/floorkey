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
pub type Map = Vec<Vec<Tile>>;

/// Map is created with Tile by Tile
/// Each Tile has character to display, blocked, visited to display partial map
#[derive(Clone, Debug)]
pub struct Tile {
    pub print: char,
    pub blocked: bool,
    pub visited: bool,
    pub color: String,
    pub print_colored: ColoredString,
}

/// Constructor for Tile struct
impl Tile {
    /// Creating empty Tile
    pub fn empty() -> Self {
        Tile {
            print: '.',
            blocked: false,
            visited: false,
            color: "green".to_string(),
            print_colored: '.'.to_string().color("green"),
        }
    }

/*
    // dead_code
    /// Creating wall Tile
    pub fn wall() -> Self {
        Tile {
            print: 'W',
            blocked: true,
            visited: false,
            color: "red".to_string(),
            print_colored: 'W'.to_string().color("red"),
        }
    }
*/

    pub fn to_wall(&mut self) {
        self.print = 'W';
        self.blocked = true;
        self.visited = false;
        self.color = "red".to_string();
        self.print_colored = 'W'.to_string().color("red");
    }

/*
    // dead_code
    /// Printing self
    pub fn print(&self) {
        print!("{}", self.print);
    }
*/
}

/*
// dead_code
/// Takes in Height and Width of the map and creates a map then returns it
pub fn make_map(x: u64, y: u64) -> Map {
    let map = vec![vec![Tile::empty(); x as usize]; y as usize];

    // Algorithm to procedurally generate map

    map
}
*/

/*
// dead_code
/// Function that takes in Map type and prints the base map of it
pub fn print_map(map: &Map) {
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
*/

/// Build a terminal printable map from map vector and overlay objects
/// from object vector.
//pub fn build_map(map: Vec<Vec<Tile>>, object: Vec<Object>) -> ColoredString {
// just build from map vector until object vector is finished
pub fn build_map(map: Vec<Vec<Tile>>) -> String {
    #[derive(Clone, Debug)]
    struct ColoredCell {
        print_colored: ColoredString,
    }

    impl ColoredCell {
        fn new() -> Self {
            ColoredCell { print_colored: " ".to_string().color("white") }
        }
    }

    let mut colormap = vec![vec![ColoredCell::new(); 0]; 0];

    // build colormap from map
    for outer in map {
        let mut cells: Vec<ColoredCell> = Vec::new();
        for inner in outer {
            let mut cell = ColoredCell::new();
            cell.print_colored = inner.print_colored;
            cells.push(cell);
        }
        colormap.push(cells);
    }

    // overlay objects
    // todo!();

    // deconstruct colormap and build result
    let mut result: String = "".to_string();
    for outer in colormap {
        for inner in outer {
            result = format!("{}{}", inner.print_colored, result);
        }
        result = format!("{}\r\n", result);
    }

    result
}


/// Function that takes in file name and create map from read in text file
/// Returns the created map
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
       // wall
       if "1-|+┌┘└┐┴┬├┤─│┼".contains(c) {
            map[i][j].to_wall();
            map[i][j].print = c;
            j += 1;
       // water
       } else if "~%".contains(c) {
           map[i][j].to_wall();
           map[i][j].print = c;
           map[i][j].color = "blue".to_string();
           map[i][j].print_colored = c.to_string().color("blue");
       } else if c == '\n' {
           i += 1;
           j = 0;
       } else {
           j += 1;
       }
    }

    map
}

pub fn get_row_col(map: &Map) -> (u64, u64) {
    (map.len() as u64, map[0].len() as u64)
}
