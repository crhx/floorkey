//! map.rs
//!
//! This module takes care of the game's map functionality
//! From creating empty map, read in .txt file to create map, to printing

// Crate for printing color in terminal
extern crate colored;

use crate::game::object::Object;
use crate::game::object::Player;
use colored::*;
use crate::game::level::*;

// File reading
//use std::fs;

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
pub fn build_map(
    map: &Vec<Vec<Tile>>,
    player: &Player,
    objects: &mut Vec<Object>,
    game_status: String,
) -> Vec<String> {
    // just build from map vector until object vector is finished
    //pub fn build_map(map: &Vec<Vec<Tile>>) -> Vec<String> {
    #[derive(Clone, Debug)]
    struct ColoredCell {
        print_colored: ColoredString,
    }

    impl ColoredCell {
        fn new() -> Self {
            ColoredCell {
                print_colored: " ".to_string().color("white"),
            }
        }
    }

    let mut colormap = vec![vec![ColoredCell::new(); 0]; 0];

    // build colormap from map

    for outer in map {
        let mut cells: Vec<ColoredCell> = Vec::new();
        for inner in outer {
            let mut cell = ColoredCell::new();
            cell.print_colored = inner.print_colored.clone();
            cells.push(cell);
        }
        colormap.push(cells);
    }

    // overlay objects
    if game_status == "game_loading" {
        let mut potion: Object = Object::empty();
        potion.to_potion();
        objects.push(potion);
    }
    for amount_of in objects {
        colormap[amount_of.x as usize][amount_of.y as usize].print_colored =
            amount_of.print_colored.clone();
    }

    // todo: add color to objects
    // todo: accept objects vector
    colormap[player.x as usize][player.y as usize].print_colored =
        player.print.to_string().color("purple");

    // deconstruct colormap and build result
    let mut result: Vec<String> = Vec::new();
    for outer in colormap {
        let mut line: String = "".to_string();
        for inner in outer {
            line = format!("{}{}", line, inner.print_colored);
        }
        result.push(line);
    }

    result
}

/// Function that takes in file name and create map from read in text file
/// Returns the created map
//pub fn read_in_map(name: &str) -> Map {
pub fn read_in_map(level_number: u32) -> Map {
    // Read in the file in string form
//    let maptext = fs::read_to_string(name).expect("Something went wrong reading the file");

    /*
        // Count how many newline char and calculate rows and columns
        let row = maptext.matches('\n').count();
        let col = (maptext.len() - row) / row;
    */

    // Create all empty tile map
    let mut map = vec![vec![Tile::empty(); 0 as usize]; 0 as usize];

    /*
        let mut i = 0;
        let mut j = 0;
    */

    // Iterate through string and modify the all empty map
    let colormap = &level(level_number).map_colors;
    let colorkey = &level(level_number).map_color_key;
    let charmap = &level(level_number).map_chars;
    let boolmap = &level(level_number).map_bools;

    for (x, char_line) in charmap.iter().enumerate() {
        let mut line: Vec<Tile> = Vec::new();
        for (y, c) in char_line.chars().enumerate() {
            let mut tile = Tile::empty();
            tile.print = c;
            for &(key, the_color) in colorkey.iter() {
                if key == colormap[x].chars().nth(y).unwrap() {
                    tile.color = the_color.to_string().clone();
                    tile.print_colored = tile.print.to_string().color(tile.color.clone());
                    break;
                }
            }
            match boolmap[x].chars().nth(y).unwrap() {
                '0' => { tile.blocked = false; tile.visited = false },
                '1' => { tile.blocked = true; tile.visited = false },
                '2' => { tile.blocked = false; tile.visited = true },
                '3' => { tile.blocked = true; tile.visited = true },
                _ => panic!("Undefined value in boolmap."),
            }
            line.push(tile);
        }
        map.push(line);
    }
    map
}

/*            if "1-|+┌┘└┐┴┬├┤─│┼".contains(c) {
                tile.to_wall();
                tile.print = c;
                tile.print_colored = c.to_string().color("red");
            }

            if "~%".contains(c) {
                tile.to_wall();
                tile.print = c;
                tile.color = "blue".to_string();
                tile.print_colored = c.to_string().color("blue");
            }
            line.push(tile);
        }
        map.push(line);
    }

    map
}
*/
pub fn get_row_col(map: &Map) -> (u64, u64) {
    (map.len() as u64, map[0].len() as u64)
}

pub fn is_collision(map: &Map, cur_pos_x: u32, cur_pos_y: u32) -> bool {
    let (row, col) = get_row_col(map);

    //if cur_pos_x < 0 || cur_pos_x >= row as u32 || cur_pos_y < 0 || cur_pos_y >= col as u32
    if cur_pos_x >= row as u32 || cur_pos_y >= col as u32 {
        return true;
    }
    /*
            if map[cur_pos_x as usize][cur_pos_y as usize].print != '.'
            {
                return true;
            }
            false
    */
    // use the blocked flag instead of symbol...
    // invisibile walls anyone?
    map[cur_pos_x as usize][cur_pos_y as usize].blocked
}
