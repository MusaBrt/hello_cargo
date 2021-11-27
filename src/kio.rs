use std::io;

// &[&str] equals to varargs
// example usages: eq_any("some", &["is_some", "or_me_some?"])
#[allow(dead_code)]
pub fn eq_all(value: &str, args: &[&str]) -> bool {


    // doesnt working, make a clean func like this 

    let mut retvalue = true;
    
    println!("--------\n{}", value);
    for str in args {
        println!("{}", *str);
        if !value.eq_ignore_ascii_case(*str) {
            retvalue=false;
            break;
        }
    }
    println!("--------");
    retvalue
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

use std::ops::{Bound, RangeBounds};

trait StringUtils {
    fn substring(&self, start: usize, len: usize) -> &str;
    fn slice(&self, range: impl RangeBounds<usize>) -> &str;
}

impl StringUtils for str {
    fn substring(&self, start: usize, len: usize) -> &str {
        let mut char_pos = 0;
        let mut byte_start = 0;
        let mut it = self.chars();
        loop {
            if char_pos == start { break; }
            if let Some(c) = it.next() {
                char_pos += 1;
                byte_start += c.len_utf8();
            }
            else { break; }
        }
        char_pos = 0;
        let mut byte_end = byte_start;
        loop {
            if char_pos == len { break; }
            if let Some(c) = it.next() {
                char_pos += 1;
                byte_end += c.len_utf8();
            }
            else { break; }
        }
        &self[byte_start..byte_end]
    }
    fn slice(&self, range: impl RangeBounds<usize>) -> &str {
        let start = match range.start_bound() {
            Bound::Included(bound) | Bound::Excluded(bound) => *bound,
            Bound::Unbounded => 0,
        };
        let len = match range.end_bound() {
            Bound::Included(bound) => *bound + 1,
            Bound::Excluded(bound) => *bound,
            Bound::Unbounded => self.len(),
        } - start;
        self.substring(start, len)
    }
}
