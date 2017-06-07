use std::collections::HashMap;

pub struct SymbolTable {
  map: HashMap<String, u32>,
  variable_offset: u32
}

impl SymbolTable {
  pub fn new() -> SymbolTable {
    let mut map = HashMap::<String, u32>::new();

    map.insert(String::from("SP"), 0);
    map.insert(String::from("LCL"), 1);
    map.insert(String::from("ARG"), 2);
    map.insert(String::from("THIS"), 3);
    map.insert(String::from("THAT"), 4);
    map.insert(String::from("R0"), 0);
    map.insert(String::from("R1"), 1);
    map.insert(String::from("R2"), 2);
    map.insert(String::from("R3"), 3);
    map.insert(String::from("R4"), 4);
    map.insert(String::from("R5"), 5);
    map.insert(String::from("R6"), 6);
    map.insert(String::from("R7"), 7);
    map.insert(String::from("R8"), 8);
    map.insert(String::from("R9"), 9);
    map.insert(String::from("R10"), 10);
    map.insert(String::from("R11"), 11);
    map.insert(String::from("R12"), 12);
    map.insert(String::from("R13"), 13);
    map.insert(String::from("R14"), 14);
    map.insert(String::from("R15"), 15);
    map.insert(String::from("SCREEN"), 0x4000);
    map.insert(String::from("KBD"), 0x6000);

    SymbolTable { map, variable_offset: 16 }
  }

  pub fn insert(&mut self, key: String, value: u32) {
    self.map.insert(key, value);
  }

  pub fn get(&self, key: &String) -> Option<&u32> {
    self.map.get(key)
  }

  pub fn contains_key(&self, key: &String) -> bool {
    self.map.contains_key(key)
  }

  pub fn append(&mut self, key: String) {
    let value = self.variable_offset;
    self.insert(key, value);
    self.variable_offset += 1;
  }
}
