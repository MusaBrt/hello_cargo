// learn rust in (x+y)^2 minutes

// creates the modules for these files...
mod guess;
mod kio;
mod array;
mod traitly;

#[allow(unused_imports)]
use kio::*;
use kio::StringUtils as StringUtils; // for eq_any
use std::collections::hash_map::Keys;
use std::io;
use std::env;
use std::collections::HashMap;
use std::io::Write;
use guess::start_guess;
use array::array_main;

fn register_commands(map: &mut HashMap<&str, fn()>) {
    map.insert("guess", start_guess);
    map.insert("array", array_main);
    
}

#[allow(dead_code)]
fn print_info(keys: Keys<&str, fn()>, args: &Vec<String>) {
    let last_index = keys.len()-1;
    let mut output = String::new();
    output.push_str("---- Commands ----\n");
    for (i, key) in keys.enumerate() {
        if i != last_index { 
            output.push_str(&format!("{} - ", key));
        } else { 
            output.push_str(key);
        }
    }
    output.push_str("\n---- Args ----\n");
    print_values(args);
    
    println!("{}", output);
}

#[allow(dead_code)]
fn main() {
    let args: Vec<String> = env::args().collect();

    // thats mutable because changes the values of inside the map
    // rust's mutable logic is a bit different from of another languages's const thing
    let mut map: HashMap<&str, fn()> = HashMap::new();
    register_commands(&mut map);
    
    println!("Welcome. You can choice a selection for your purpose. \nFor list type list.\nFor quit type quit or exit.\n----------------\n");
    let mut entry: String;
    loop {
        print!("> ");
        io::stdout().flush(); // for print >
        entry = String::new(); // clear the buffer

        io::stdin().read_line(&mut entry).expect("An error occur while read the entry.");
        let entrim = entry.trim();

        if entrim.eq_any(&["exit", "quit"]) {
            println!("Goodbye ^^");
            break
        }
        
        if entrim.eq_any(&["help", "list", "commands"]) {
            print_info(map.keys(), &args);
            continue;
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