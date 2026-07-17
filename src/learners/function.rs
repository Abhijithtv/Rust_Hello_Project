use crate::helpers::display_helper;

const CONTEXT: &str = "FUNCTION";

pub fn learn_function(){
    display_helper::start(CONTEXT);   
    learn_param_passing();
    learn_block_expression();
    learn_return_value();
    display_helper::end(CONTEXT);
}

fn learn_return_value(){
    let a = 2;
    let b= 4;
    println!("Add with return, {a} + {b} = {}", add_with_return(2, 4));
}


fn learn_block_expression() {
    let age:i32 ={
        let born = 2008;
        let now = 2026;
        now - born
    };
    println!("block age {age}");
}

fn learn_param_passing(){
    add(3, 4, "user1");
}


fn add(a:i32, b:i32, user: &str){
    println!("for user {user}, {a} + {b} = {}",a+ b);
}

fn add_with_return(a:i32, b:i32) -> i32 {
    return  a+b;
}