# Floorkey

[![MIT License](https://img.shields.io/apm/l/atomic-design-ui.svg?)](https://github.com/crhx/floorkey/blob/main/LICENSE)

Portland State University

Spring 2021 CS510 Rust Programming Final Project

## Description

Floorkey is a roguelike game that lets players go through levels of dungeon by clearing up quiz-like interactions.
Players can move through the dungeon to interact with items on the floor or obstacles that might block your path to victory!
 
Dungeon contains items that might help the player through their journey or it could potentially be the player's excruciating end.
Pick up a pickaxe to break the boulder and more! Pick up water to give yourself a cooling salvation from the fire that burns off your skin! Or just watch the exit from the other side of the obstacle. 

Figure out what works and what doesn't through pain!

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

## Lessons Learned

* What didn't work out
    * Structuring of the modules and its functionality
        * Certain functionality bled into one another due to lack of cohesivenss of what certain module is responsible for. Such as item interaction vs object interaction
        * Tossing struct data around into different modules as a pub field
    * Easily structured interaction
        * Was hoping to implement easily checkable item to object interaction
        * Ended up using pattern matching and if statements to do the interaction, which made the code base messy and harder to read
    * Printing in color
        * Tried to print color in terminal without using a crate function thus cutting one less step

* What did work
    * Game printing
        * Combining different components such as map, objects on the map, and player into one vector of String for printing
        * Easily print color through struct's color printing function field instead of checking the element to call specific color function
    * Game level initialization
        * Using ```level.rs``` to initialize all the components of the game level to cut down on repetitive coding within ```game.rs```
    * User input
        * Utilizing ```termion``` to read user input character by character without user manually pressing enter after every turn
    * Item interaction
        * Player's ability to pick up holdable objects from the map or replace the object on the map with what player holds
        * Interaction between the objects on the map with the player's item such as breaking a boulder with a pickaxe

* Improvement for future
    * Structuring the project layout and each module's functionality more deeply before implementation
    * Creating key interactions between objects beforehand so that we don't have to alter functionality or keep on adding if statement to check new interactions
    * Creating only the necessary fields within the struct and variable with intuitive name

  
## Authors

- [@AmulyaH](https://github.com/AmulyaH) => amulya2@pdx.edu
- [@Bittaurus](https://github.com/Bittaurus) => gbarron@pdx.edu
- [@crhx](https://github.com/crhx) => tevren@pdx.edu
- [@Leonlee190](https://github.com/Leonlee190) => seung2@pdx.edu

  
## Contributing

Contributions are always welcome!

Please contact our repo master [@crhx](https://github.com/crhx)!
