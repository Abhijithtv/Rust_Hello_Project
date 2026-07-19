use crate::helpers::display_helper;

const CONTEXT: &str = "ERROR-HANDLING";

pub fn learn_error_handling(){
    display_helper::start(CONTEXT);
    learn_option();
    learn_result();
    display_helper::end(CONTEXT);
}

fn learn_result(){

    let inputs = [2, 5];

    for id in inputs{
        match get_balance(id) {
            Ok(val) => println!("Result of val = {val}"),
            Err(val) =>println!("Result Error - {val}")
        }
    }
}

fn get_balance(id: i32)->Result<i32, String>{
    return if id%2==0 {
        Ok(123)
    }
    else {
        Err("Unknown id".to_string())
    }
}

fn learn_option(){
   try_get_age("abh");
   try_get_age("abhi");
}

fn try_get_age(arg: &str) {
     match get_age(arg)  {
        Some(age) => println!("Success and age is {age}"),
        None => println!("not sucess")
    }
}

fn get_age(name: &str)->Option<i32>{
    return if name == "abhi"{
        Some(24)
    }else{
        None
    }
}