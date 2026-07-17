use crate::display_helper;

pub fn learn_primary_dataype(){
    let context: &str = "PRIMARY DATA TYPE";
    display_helper::start(context);
    learn_numbers();
    learn_bool();
    learn_char();
    learn_float();
    display_helper::end(context);
}

fn learn_float(){
    let num = 1.23;
    println!("float is-{num}");
}

fn learn_char(){
    let c = 'a';
    println!("char is {c}");
}

fn learn_bool(){
    let status = true;
    println!("boolean status is {status}");
}


fn learn_numbers(){
    let x = 10;
    println!("from learn numbers = {x}");
}