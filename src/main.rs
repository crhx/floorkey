mod map;

fn main() {
    let created = map::read_in_map("map2.txt");

    map::print_map(created);
}
