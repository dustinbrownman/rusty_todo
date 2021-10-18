use std::collections::HashMap;
use std::env::args;

fn main() {
    let action = args().nth(1).expect("Please specify an action");
    let possible_item = args().nth(2);

    let mut todo = Todo::new().expect("Initialization of db failed");
    if action == "add" {
        let item = possible_item.expect("Please specify an item");
        todo.insert(item);
        match todo.save() {
            Ok(_) => println!("todo saved"),
            Err(why) => println!("An error occurred: {}", why),
        }
    } else if action == "complete" {
        let item = possible_item.expect("Please specify an item");
        match todo.complete(&item) {
            None => println!("{} not found in the list", item),
            Some(_) => match todo.save() {
                Ok(_) => println!("'{}' complete", &item),
                Err(why) => println!("An error occurred: {}", why),
            },
        }
    } else if action == "list" {
        for (action, needs_doing) in todo.map {
            let check = if needs_doing { "[ ]" } else { "[X]" };
            println!("{} {}", check, action);
        }
    }
}

struct Todo {
    map: HashMap<String, bool>,
}

impl Todo {
    fn new() -> Result<Todo, std::io::Error> {
        let file = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open("db.json")?;

        match serde_json::from_reader(file) {
            Ok(map) => Ok(Todo { map }),
            Err(e) if e.is_eof() => Ok(Todo {
                map: HashMap::new(),
            }),
            Err(e) => panic!("An error occurred: {}", e),
        }
    }

    fn insert(&mut self, key: String) {
        self.map.insert(key, true);
    }

    fn complete(&mut self, key: &String) -> Option<()> {
        match self.map.get_mut(key) {
            Some(value) => Some(*value = false),
            None => None,
        }
    }

    fn save(self) -> Result<(), Box<dyn std::error::Error>> {
        let file = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open("db.json")?;

        serde_json::to_writer_pretty(file, &self.map)?;
        Ok(())
    }
}
