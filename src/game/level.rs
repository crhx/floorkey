//! levels.rs
//!
//! This module is a store for level data

extern crate colored;

use crate::game::object::Object;
use colored::*;
//use crate::game::monster::Monster;

///
/// Game contains different levels and each level has it's own map, objects and exit points
/// 
pub struct Level<'a> {
    pub level_name: &'a str,
    pub player_start_x: usize,
    pub player_start_y: usize,
    pub map_chars: Vec<&'a str>,
    pub map_colors: Vec<&'a str>,
    pub map_color_key: Vec<(char, &'a str)>,
    // map_bools is adding bits: 1 = blocked, 2 = visited
    pub map_bools: Vec<&'a str>,
    pub objects: Vec<Object>,
    //    pub monsters: Vec<Monster>,
}

///
/// Function to create maps for each levels on the game
/// @level_number : game level number
/// @returns : level 
/// 
pub fn level(level_number: usize) -> Level<'static> {
    match level_number {
        1 => Level {
            level_name: "Level 1",
            player_start_x: 5,
            player_start_y: 5,
            map_chars: [
                "..................~┌──────",
                ".................~%│......",
                "................~%~│......",
                ".................~%│......",
                "..........................",
                ".................~~│......",
                "................~%~│......",
                ".................~%│......",
                "..................~└──────",
            ]
            .to_vec(),
            map_colors: [
                "                  brrrrrrr",
                "                 bbr      ",
                "                bbbr      ",
                "                 bbr      ",
                "                          ",
                "                 bbr      ",
                "                bbbr      ",
                "                 bbr      ",
                "                  brrrrrrr",
            ]
            .to_vec(),
            map_color_key: [('r', "red"), ('b', "blue"), (' ', "green")].to_vec(),
            map_bools: [
                "22222222222222222233333333",
                "22222222222222222333222222",
                "22222222222222223333222222",
                "22222222222222222333222222",
                "22222222222222222222222222",
                "22222222222222222333222222",
                "22222222222222223333222222",
                "22222222222222222333222222",
                "22222222222222222233333333",
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
                    holdable: false,
                    color: "white".to_string(),
                    print_colored: '0'.to_string().color("white"),
                    paired_item : "".to_string(),
                    score : 1,
										id : 1,
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
                    holdable: false,
                    color: "white".to_string(),
                    print_colored: '0'.to_string().color("white"),
                    paired_item : "".to_string(),
                    score : 0,
										id : 2,
                },
						   Object {
                    x: 6,
                    y: 8,
                    print: '0',
                    attri: 0,
                    mat: 0,
                    status: 0,
                    quantity: 1,
                    descr: "Another completely average boulder.".to_string(),
                    holdable: false,
                    color: "white".to_string(),
                    print_colored: '0'.to_string().color("white"),
                    paired_item : "".to_string(),
                    score : 0,
										id : 2,
                },
						   Object {
                    x: 7,
                    y: 8,
                    print: '0',
                    attri: 0,
                    mat: 0,
                    status: 0,
                    quantity: 1,
                    descr: "Another completely average boulder.".to_string(),
                    holdable: false,
                    color: "white".to_string(),
                    print_colored: '0'.to_string().color("white"),
                    paired_item : "".to_string(),
                    score : 0,
										id : 2,
                },
								Object {
                    x: 7,
                    y: 7,
                    print: '⑈',
                    attri: 0,
                    mat: 0,
                    status: 0,
                    quantity: 1,
                    descr: "Key".to_string(),
                    holdable: true,
                    color: "bright_yellow".to_string(),
                    print_colored: '⑈'.to_string().color("bright_yellow"),
                    paired_item : "".to_string(),
                    score : 3,
										id : 13,
								},

						   Object {
                    x: 8,
                    y: 7,
                    print: '0',
                    attri: 0,
                    mat: 0,
                    status: 0,
                    quantity: 1,
                    descr: "Another completely average boulder.".to_string(),
                    holdable: false,
                    color: "white".to_string(),
                    print_colored: '0'.to_string().color("white"),
                    paired_item : "".to_string(),
                    score : 0,
										id : 2,
                },		
								Object {
                    x: 7,
                    y: 6,
                    print: '0',
                    attri: 0,
                    mat: 0,
                    status: 0,
                    quantity: 1,
                    descr: "Another completely average boulder.".to_string(),
                    holdable: false,
                    color: "white".to_string(),
                    print_colored: '0'.to_string().color("white"),
                    paired_item : "".to_string(),
                    score : 0,
										id : 2,
                },



                Object {
                    x: 4,
                    y: 3,
                    print: 'Γ',
                    attri: 0,
                    mat: 0,
                    status: 0,
                    quantity: 1,
                    descr: "Pickaxe".to_string(),
                    holdable: true,
                    color: "yellow".to_string(),
                    print_colored: 'Γ'.to_string().color("yellow"),
                    paired_item : "".to_string(),
                    score : 3,
										id : 3,
                },
								Object {
                    x: 4,
                    y: 24,
                    print: 'E',
                    attri: 0,
                    mat: 0,
                    status: 0,
                    quantity: 1,
                    descr: "Exit".to_string(),
                    holdable: false,
                    color: "pink".to_string(),
                    print_colored: 'E'.to_string().color("pink"),
                    paired_item : "".to_string(),
                    score : 0,
										id : 99,
                },
															Object {
                    x: 4,
                    y: 19,
                    print: '█',
                    attri: 0,
                    mat: 0,
                    status: 0,
                    quantity: 1,
                    descr: "Door".to_string(),
                    holdable: true,
                    color: "magenta".to_string(),
                    print_colored: '█'.to_string().color("magenta"),
                    paired_item : "".to_string(),
                    score : 0,
										id : 80,
                },

            ]
            .to_vec(),
        },
        2 => Level {
            level_name: "Level 2",
            player_start_x: 4,
            player_start_y: 3,
            map_chars: [
                "┌────────┬──┬────────────┐",
                "│........│..│............│",
                "│........│...............│",
                "│........│..│............│",
                "│........└..┤............│",
                "│...........│............│",
                "│...........│............│",
                "│...........│............│",
                "└───────────┴────────────┘",
            ]
            .to_vec(),
            map_colors: [
                "rrrrrrrrrrrrrrrrrrrrrrrrrr",
                "r                        r",
                "r                        r",
                "r                        r",
                "r                        r",
                "r                        r",
                "r                        r",
                "r                        r",
                "rrrrrrrrrrrrrrrrrrrrrrrrrr",
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
                    x: 2,
                    y: 10,
                    print: '0',
                    attri: 0,
                    mat: 0,
                    status: 0,
                    quantity: 1,
                    descr: "A completely average boulder.".to_string(),
                    holdable: false,
                    color: "white".to_string(),
                    print_colored: '0'.to_string().color("white"),
                    paired_item : "".to_string(),
                    score : 0,
										id : 1,
                },
                Object {
                    x: 2,
                    y: 11,
                    print: '0',
                    attri: 0,
                    mat: 0,
                    status: 0,
                    quantity: 1,
                    descr: "Another completely average boulder.".to_string(),
                    holdable: false,
                    color: "white".to_string(),
                    print_colored: '0'.to_string().color("white"),
                    paired_item : "".to_string(),
                    score : 0,
										id : 2,
                },
                Object {
                    x: 1,
                    y: 1,
                    print: 'ך',
                    attri: 0,
                    mat: 0,
                    status: 0,
                    quantity: 1,
                    descr: "Pickaxe".to_string(),
                    holdable: true,
                    color: "yellow".to_string(),
                    print_colored: 'ך'.to_string().color("yellow"),
                    paired_item : "".to_string(),
                    score : 20,
										id : 3,
                },
                Object {
                    x: 3,
                    y: 3,
                    print: 'ם',
                    attri: 0,
                    mat: 0,
                    status: 0,
                    quantity: 1,
                    descr: "Fire".to_string(),
                    holdable: false,
                    color: "Yellow".to_string(),
                    print_colored: 'ם'.to_string().color("Yellow"),
                    paired_item : "".to_string(),
                    score : 0,
										id : 4,
                },
                Object {
                    x: 6,
                    y: 19,
                    print: 'ם',
                    attri: 0,
                    mat: 0,
                    status: 0,
                    quantity: 1,
                    descr: "Water".to_string(),
                    holdable: false,
                    color: "Yellow".to_string(),
                    print_colored: 'ם'.to_string().color("Yellow"),
                    paired_item : "".to_string(),
                    score : 0,
										id : 5,
                },
            ]
            .to_vec(),
        },
				3 => Level {
            level_name: "Level 3",
            player_start_x: 5,
            player_start_y: 5,
            map_chars: [
            	  "┌────────┬─┬─┬─┬─────────┐",
                "│........│.│.│.│.........│",
                "│........└.┴.┴.┘.........│",
                "│........................│",
                "│........................│",
                "│........................│",
                "│........................│",
                "│........................│",
                "└────────────────────────┘",
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
                    holdable: false,
                    color: "white".to_string(),
                    print_colored: '0'.to_string().color("white"),
                    paired_item : "".to_string(),
                    score : 0,
										id : 1,
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
                    holdable: false,
                    color: "white".to_string(),
                    print_colored: '0'.to_string().color("white"),
                    paired_item : "".to_string(),
                    score : 0,
										id : 2,

                },
            ]
            .to_vec(),
        },
				4 => Level {
            level_name: "Level 4",
            player_start_x: 5,
            player_start_y: 5,
            map_chars: [
             	  "┌────────┬─┬─┬─┬─────────┐",
                "│........│.│.│.│.........│",
                "│........└.┴.┴.┘.........│",
                "│........................│",
                "│........................│",
                "│........................│",
                "│........................│",
                "│........................│",
                "└────────────────────────┘",
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
                    holdable: false,
                    color: "white".to_string(),
                    print_colored: '0'.to_string().color("white"),
                    paired_item : "".to_string(),
                    score : 0,
										id : 1,
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
                    holdable: false,
                    color: "white".to_string(),
                    print_colored: '0'.to_string().color("white"),
                    paired_item : "".to_string(),
                    score : 0,
										id : 2,
                },
            ]
            .to_vec(),
        },
				5 => Level {
            level_name: "Level 5",
            player_start_x: 5,
            player_start_y: 5,
            map_chars: [
             	  "┌────────┬─┬─┬─┬─────────┐",
                "│........│.│.│.│.........│",
                "│........└.┴.┴.┘.........│",
                "│........................│",
                "│........................│",
                "│........................│",
                "│........................│",
                "│........................│",
                "└────────────────────────┘",
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
                    holdable: false,
                    color: "white".to_string(),
                    print_colored: '0'.to_string().color("white"),
                    paired_item : "".to_string(),
                    score : 0,
										id : 1,
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
                    holdable: false,
                    color: "white".to_string(),
                    print_colored: '0'.to_string().color("white"),
                    paired_item : "".to_string(),
                    score : 0,
										id : 2,
                },
            ]
            .to_vec(),
        },



        _ => panic!("call to initialize undefined level number in init_level()"),
    }
}
