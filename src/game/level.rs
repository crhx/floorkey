//! levels.rs
//!
//! This module is a store for level data

extern crate colored;

use crate::game::object::Object;
use colored::*;
//use crate::game::monster::Monster;

pub struct Level<'a> {
    pub level_name: &'a str,
    pub player_start_x: u32,
    pub player_start_y: u32,
    pub map_chars: Vec<&'a str>,
    pub map_colors: Vec<&'a str>,
    pub map_color_key: Vec<(char, &'a str)>,
    // map_bools is adding bits: 1 = blocked, 2 = visited
    pub map_bools: Vec<&'a str>,
    pub objects: Vec<Object>,
    //    pub monsters: Vec<Monster>,
}

pub fn level(level_number: u32) -> Level<'static> {
    match level_number {
        1 => Level {
            level_name: "Level 1",
            player_start_x: 5,
            player_start_y: 5,
            map_chars: [
                ".......┌─┬─┬─┬─┬─────┐...~",
                ".......│.│.│.│.│.....│..~%",
                ".......└.┴.┴.┴.┤.....│..~%",
                ".....................│.~%~",
                "........┌─────┐......│.~~%",
                "........│.....│......│..~~",
                "........│............│.~~~",
                "........└─────┴──────┤...~",
                "..........................",
            ]
            .to_vec(),
            map_colors: [
                "       rrrrrrrrrrrrrrr   b",
                "       r r r r r     r  bb",
                "       r r r r r     r  bb",
                "                     r bbb",
                "        rrrrrrr      r bbb",
                "        r     r      r  bb",
                "        r            r bbb",
                "        rrrrrrrrrrrrrr   b",
                "                          ",
            ]
            .to_vec(),
            map_color_key: [('r', "red"), ('b', "blue"), (' ', "green")].to_vec(),
            map_bools: [
                "22222223333333333333332223",
                "22222223232323232222232233",
                "22222223232323232222232233",
                "22222222222222222222232333",
                "22222222333333322222232333",
                "22222222322222322222232233",
                "22222222322222222222232333",
                "22222222333333333333332223",
                "22222222222222222222222222",
            ]
            .to_vec(),
            objects: [
                Object {
                    x: 6,
                    y: 6,
                    print: '0',
                    attri: 0,
                    mat: 0,
                    status: 0,
                    quantity: 1,
                    descr: "A completely average boulder.".to_string(),
                    holdable: true,
                    color: "white".to_string(),
                    print_colored: '0'.to_string().color("white"),
                },
                Object {
                    x: 6,
                    y: 7,
                    print: '0',
                    attri: 0,
                    mat: 0,
                    status: 0,
                    quantity: 1,
                    descr: "Another completely average boulder.".to_string(),
                    holdable: true,
                    color: "white".to_string(),
                    print_colored: '0'.to_string().color("white"),
                },
            ]
            .to_vec(),
            //                 [],
        },
        _ => panic!("call to initialize undefined level number in init_level()"),
    }
}
