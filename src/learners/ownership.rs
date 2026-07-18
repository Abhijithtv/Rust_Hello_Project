use crate::helpers::display_helper;

const CONTEXT:&str = "Ownership";

pub fn learn_ownership(){
    display_helper::start(CONTEXT);   
    learn_pass_reference();
    learn_one_ownership();
    display_helper::end(CONTEXT);   
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
