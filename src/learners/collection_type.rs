use std::collections::HashMap;

use crate::helpers::display_helper;

const CONTEXT:&str = "COLLECTIONS";

pub fn learn_collection(){
    display_helper::start(CONTEXT);
    learn_vector();
    learn_utf8();
    learn_hash_map();
    display_helper::end(CONTEXT);
}

fn learn_hash_map() {
    let mut map: HashMap<i32, &str> = HashMap::new();
    map.insert(1, "one");
    let keys = &[1, 2];
    for key in keys{
        let res = map.get(key).unwrap_or(&"unknown");
        println!("{key} -> {res}");
    }
}

fn learn_utf8(){
    let mut s = String::from("abc");
    s.push('-');
    s.push_str("def");
    println!("string after mutating = {s}")
}

fn learn_vector() {
    //using new
    {
        let mut vec: Vec<i32> =  Vec::new();
        vec.push(2);
        vec.push(13);
        println!("vec using new {:?}", vec);
    }

    //using vec!
    {
        let vec: Vec<i32> = vec![1, 23, 4];
        println!("vec using vec! {:?}", vec);
    }
}