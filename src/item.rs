type Inventory = Vec<Item>;

#[derive(Clone, Copy, Debug)]
pub struct Item {
  quantity: u32,
  x: u32,
  y: u32,
  descr: String,
}

pub fn print_inven(inven: Inventory) {
  for x in 0..inven.len() {
    println!("{}", inven[x].descr);
  }
}

