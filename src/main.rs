// learn rust in (x+y)^2 minutes

// creates the modules for these files...
mod guess;
mod kio;
mod array;

#[allow(unused_imports)]
use kio::{get_string, get_int, eq_all};
use std::collections::hash_map::Keys;
use std::io;
use std::collections::HashMap;
use guess::start_guess;
use array::array_main;

fn register_commands(map: &mut HashMap<&str, fn()>) {
    map.insert("guess", start_guess);
    map.insert("array", array_main);
    
}

#[allow(dead_code)]
fn list_commands(keys: Keys<&str, fn()>) {
    for key in keys {
        print!("{} - ", key);
    }

}

#[allow(dead_code)]
fn main() {
    // thats mutable because changes the values of inside the map
    // rust's mutable logic is a bit different from of another languages's const thing
    let mut map: HashMap<&str, fn()> = HashMap::new();
    register_commands(&mut map);
    
    println!("Welcome. You can choice a selection for your purpose. \nFor list type list.\nFor quit type quit or exit.");
    let mut entry: String;
    loop {
        entry = String::new(); // classic problem
        io::stdin().read_line(&mut entry).expect("An error occur while read the entry.");
        let entrim = entry.trim();


        // fix eq_all function in kio.rs
        if eq_all(entrim, &["exit", "quit"]) {
            println!("Goodbye ^^");
            break
        }

        if eq_all(entrim, &["help", "list", "commands"]) {
            list_commands(map.keys());
        }
                

        if !map.contains_key(entrim) {
            println!("Command not found dude.");
        } else {
            // map works with Option<>
            let func_option = map.get(entrim);
            match func_option {
                Some(func) => {
                    println!("Calling the {}'s function...", entrim);
                    func();
                } None => {
                    panic!("AaAaAaAAaa function missing... bro wtf is going here");
                }
            };
        }
    }
}