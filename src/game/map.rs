//! map.rs
//!
//! This module takes care of the game's map functionality
//! From creating empty map, read in .txt file to create map, to printing

// Crate for printing color in terminal
extern crate colored;

use crate::game::level::*;
use crate::game::object::Object;
use crate::game::object::Player;
use crate::game::Game;
use colored::*;

// Map is made up of Tile x Tile
pub type Map = Vec<Vec<Tile>>;

///
/// Map is created with Tile by Tile
/// Each Tile has character to display, blocked, visited to display partial map
///
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
}

///
/// Build a terminal printable map from map vector and overlay objects from object vector.
/// @map : map object
/// @player : Player on the map
/// @objects : vector of all the objects on the map
/// @inventory : an object which player is hodling
/// @message : game message to display on screen
/// @game_status : status of the game (0 : continue, 1 : won, 2 : dead)
/// @returns : vector of messages to display
///
pub fn build_map(
    map: &[Vec<Tile>],
    player: &Player,
    objects: &mut Vec<Object>,
    inventory: &mut Object,
    message: &mut Vec<String>,
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

    // Add player Inventory
    result.push(String::from("\n"));
    result.push(String::from("----------- Inventory -----------"));
    result.push(String::from(""));

    // Add object to inventory if its holdable object
    if inventory.holdable {
        result.push(inventory.descr.clone());
    }
    // Add player score to result vec
    result.push("Your Score :".to_owned() + &player.score.to_string());
    result.push(String::from(""));

    // Add game's messages at the very end
    result.push(String::from("----------- Game Message -----------"));
    result.append(&mut message.clone());

    result
}

///
/// Function that takes in file name and create map from read in text file
/// @level_number : player level number in game
/// @Returns : the created map
///
pub fn read_in_map(level_number: usize) -> Map {
    let mut map = vec![vec![Tile::empty(); 0_usize]; 0_usize];

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
            // match with boolmap bits: 0 = not blocked & not visited, 1 = blocked, 2 = visited, 3 = blocked & visited
            match boolmap[x].chars().nth(y).unwrap() {
                '0' => {
                    tile.blocked = false;
                    tile.visited = false
                }
                '1' => {
                    tile.blocked = true;
                    tile.visited = false
                }
                '2' => {
                    tile.blocked = false;
                    tile.visited = true
                }
                '3' => {
                    tile.blocked = true;
                    tile.visited = true
                }
                _ => panic!("Undefined value in boolmap."),
            }
            line.push(tile);
        }
        map.push(line);
    }
    map
}

///
/// Function get the number of rows and number of columns of the map
/// @map : map object
/// @returns : tuple of (no_of_row, no_of_column)
///
pub fn get_row_col(map: &Map) -> (u64, u64) {
    (map.len() as u64, map[0].len() as u64)
}

///
/// Function to find if the player collides with a wall
/// @map : game map for finding walls
/// @cur_pos_x : current position of player wrt x co-ordinate
/// @cur_pos_y : current position of player wrt y co-ordinate
/// @returns : true if player collieds with wall otherwise returns false
///
pub fn is_collision(game: &Game, cur_pos_x: usize, cur_pos_y: usize) -> bool {
    let map = &game.map;
    let objects = &game.objects;
    let inventory = &game.inventory;
    let (row, col) = get_row_col(map);

    //if cur_pos_x < 0 || cur_pos_x >= row as usize || cur_pos_y < 0 || cur_pos_y >= col as usize
    if cur_pos_x >= row as usize || cur_pos_y >= col as usize {
        return true;
    }

    for (_i, object) in objects.iter().enumerate() {
        if cur_pos_x == object.x && cur_pos_y == object.y {
            match object.id {
                1 | 2 => {
                    if inventory.id != 3 {
                        return true;
                    }
                }
                80 => {
                    if inventory.id != 13 && inventory.id != 3 {
                        return true;
                    }
                }
                _ => {}
            }
        }
    }
    // return true if wall
    map[cur_pos_x as usize][cur_pos_y as usize].blocked
}
