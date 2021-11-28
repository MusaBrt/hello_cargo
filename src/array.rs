#[allow(unused_imports)]
use crate::kio::get_string;

fn change_value(num: &mut i8) {
    // dereference for the get exact value inside of the num
    // because the num is a pointer
    // like C

    // & -> pointer and mut -> literally mutable
    *num = *num + 1;
}

#[allow(dead_code)]
fn reference_things() {

    // like this, if you can pass the parameter by &mut, its mutable pointer
    let mut nonfinal: i8 = 100;
    change_value(&mut nonfinal);


    let x: i8 = 100;
    let mut ref_x = &x;

    // all lines print 100
    println!("{}", *ref_x); // printing a de-referenced value    i8
    println!("{}", ref_x);  // printing a pure reference value   &i8
    println!("{}", &ref_x); // printing a double reference value &&i8
    
    ref_x = &101;

    println!("{}", *ref_x); // printing a de-referenced value    i8
    println!("{}", ref_x);  // printing a pure reference value   &i8
    println!("{}", &ref_x); // printing a double reference value &&i8


}

pub fn array_main() {
    println!("---- Array main ----");
    reference_things();
    println!("---- Array end  ----");
}