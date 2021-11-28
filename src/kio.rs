use std::io;

// like interface thing
pub trait StringUtils {
    fn eq_any(&self, args: &[&str]) -> bool;
}

// interface implements -for- something
// very cool thing :D
impl StringUtils for &str {
    // &[&str] equals to varargs
    // example usages: String::from("zink").eq_any(&["is_some", "or_me_some?"])
    fn eq_any(&self, args: &[&str]) -> bool {
        let mut retvalue = false;

        for str in args {
            if self.eq_ignore_ascii_case(*str) {
                retvalue=true;
                break;
            }
        }

        retvalue
    }
}

#[allow(dead_code)]
pub fn get_string() ->  String {
    let mut entry = String::new();
    io::stdin().read_line(&mut entry).expect("An error occur.");
    entry        
}

#[allow(dead_code)]
pub fn get_int() -> i32 {
    let mut str = String::new();
    loop {
        io::stdin().read_line(&mut str).expect("An error occur.");
        match str.trim().parse() {
            Ok(num) => {
                return num;
            }
            Err(_) => {
                // println!("Entered value is {}\nErr string: {}", str, err.to_string());
                println!("Only number mate... Only numbers.");
                // there's a problem while assigning the new value from stdin
                // idk why. todo solve it
                str = String::new(); 
                continue;
            }
        };
    }
}

pub fn print_values(arr: &Vec<String>) {
    let mut buf = String::new();
    let last_index = arr.len()-1;
    
    for i in 0..=last_index {
        if i != last_index {
            buf.push_str(&format!("{} - ", arr[i]));
        } else {
            buf.push_str(&arr[i]);
        }
    }
    println!("{}", buf);
}

