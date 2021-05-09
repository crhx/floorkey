mod map

type Game = [map::Map, item::Iventory, item::Iventory, object::Objects];

pub fn create_game() -> Game {
  let 
}

pub fn creat_game_file(mapName: String, itemName: String, objName: String) -> Game {
  let map = map::read_in_map(mapName);
}
