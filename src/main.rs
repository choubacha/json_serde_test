#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

use std::io::{self, Read};

#[derive(Serialize, Deserialize, Debug)]
struct Thing {
    name: String,
    age: i32,
}

fn main() {
    let thing = Thing { name: "Kevin".to_string(), age: 33 };
    println!("the basic form: {}", serde_json::to_string(&thing).unwrap());
    println!("now you try (just press enter to quit):");

    loop {
        let mut buf = String::new();
        match io::stdin().read_line(&mut buf) {
            Ok(n) if n > 1 => {
                match serde_json::from_str::<Thing>(&buf) {
                    Ok(thing) => println!("Your name is {} and your age is {}", thing.name, thing.age),
                    Err(err) => println!("Did not recognize json:\n{}", err),
                }
            },
            Ok(_) => break,
            Err(err) => println!("error: {}", err),
        }
    }
}
