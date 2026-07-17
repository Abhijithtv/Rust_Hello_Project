use crate::display_helper;

pub fn primary_data_type_learn(){
    let context: &str = "PRIMARY DATA TYPE";
    display_helper::start(context);
    assign_numbers();
    assign_bool();
    assign_char();
    assign_float();
    display_helper::end(context);
}

fn assign_float(){
    let num = 1.23;
    println!("float is-{num}");
}

fn assign_char(){
    let c = 'a';
    println!("char is {c}");
}

fn assign_bool(){
    let status = true;
    println!("boolean status is {status}");
}


fn assign_numbers(){
    let x = 10;
    println!("from assign numbers = {x}");
}