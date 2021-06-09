# Floorkey

[![MIT License](https://img.shields.io/apm/l/atomic-design-ui.svg?)](https://github.com/crhx/floorkey/blob/main/LICENSE)

Portland State University

Spring 2021 CS510 Rust Programming Final Project

## Description

Floorkey is a roguelike game that lets players go through levels of dungeon by clearing up quiz-like interactions.
Players can move through the dungeon to interact with items on the floor or obstacles that might block your path to victory!
 
Dungeon contains items that might help the player through their journey or it could potentially be the player's excruciating end.
Pick up a pickaxe to break the boulder and more! Pick up water to give yourself a cooling salvation from the fire that burns off your skin! Or just watch the exit from the other side of the obstacle. 

Try to exit each level of dungeon with the fastest time and win the whole game!
## Installation 

* Required crates
    * termion: 
        * https://docs.rs/termion/1.5.6/termion/
        * https://crates.io/crates/termion
    * colored: 
        * https://docs.rs/colored/2.0.0/colored/ 
        * https://crates.io/crates/colored

Build: 
```bash 
  cargo build
```

Run:
```bash
  cargo run
```
## User Input

- *w* : Move the player up
- *s* : Move the player down
- *a* : Move the player left
- *d* : Move the player right
- *q* : Quit the whole game

  
## File Summary

* ```floorkey/src/main.rs``` : Initializes the game and its levels. Takes in a user input checks for its validity and updates the game accordingly
* ```floorkey/src/game.rs``` : Module that stores the level data including map, player information, objects on the map, object that player is holding, game messages, and game status. This module can initialize game, move player, interact with objects including items, and print the updated map.
* ```floorkey/src/game/map.rs``` : Module for ```game.rs``` which contains map initialization with color, base map collision checking, and building an combined map for printing purpose.
* ```floorkey/src/game/object.rs``` : Module for ```game.rs``` which contains repositioning of objects including player's location.
* ```floorkey/src/game/level.rs``` : Contains level struct that stores all the dungeon level information so that when initializing the ```game.rs``` reads all the dungeon information from here.

  
## Running Tests

To run tests, run the following command

```bash
  cargo test
```

  
## Authors

- [@AmulyaH](https://github.com/AmulyaH) => amulya2@pdx.edu
- [@Bittaurus](https://github.com/Bittaurus) => gbarron@pdx.edu
- [@crhx](https://github.com/crhx) => tevren@pdx.edu
- [@Leonlee190](https://github.com/Leonlee190) => seung2@pdx.edu

  
## Contributing

Contributions are always welcome!

Please contact our repo master [@crhx](https://github.com/crhx)!
