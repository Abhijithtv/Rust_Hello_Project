use crate::helpers::display_helper;

const CONTEXT:&str = "SHADOWING";

pub fn learn_shadowing(){
    display_helper::start(CONTEXT);
    learn();
    display_helper::end(CONTEXT);
}

fn learn(){
    //use -  transform non mutable variable without a new name
    let url = "abc.com/";
    let url = url.trim_end_matches("/");
    println!("transformed url is {url}");
}