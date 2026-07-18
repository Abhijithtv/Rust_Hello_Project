use crate::helpers::display_helper;

const CONTEXT:&str = "Ownership";

pub fn learn_ownership(){
    display_helper::start(CONTEXT);   
    learn_pass_reference();
    learn_one_ownership();
    learn_borrowing();
    learn_mut_reference();
    display_helper::end(CONTEXT);   
}

fn learn_mut_reference() {
    let mut  _x  = 10;
    let _y: &mut i32 = &mut _x;
    *_y = *_y + 1;
    println!("after mut ref of x={_y}");
}

fn learn_borrowing(){
    let x = 12;
    let y: &i32 = &x;
    println!("x is {x}");
    println!("y is {y}");
    println!("*y is {}", *y);
    println!("address of x     = {:p}", &x);
    println!("address in y     = {:p}", y);
    println!("address of y     = {:p}", &y);
}

fn learn_one_ownership(){
    let s = String::from("Hi Value");
    let copy = s;
    printter(&copy);
    //throws error
    //printter(&s);
}

fn learn_pass_reference() {
    let s = String::from("Hi Value");
    printter(&s);
}

fn printter(s: &String ){
    println!("From Printter - {s}");
}
