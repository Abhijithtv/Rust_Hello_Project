use crate::helpers::display_helper;

pub fn learn_compound_data_type(){
let context: &str = "COMPOUND DATA TYPE";
    display_helper::start(context);
    learn_array();
    learn_tuple();
    learn_slice();
    learn_mut_string();
    display_helper::end(context);
}

fn learn_mut_string(){
    let mut name = String:: from("GHO");
    let name_copy = &name;
    println!("mut before name - {name}");
    println!("mut copy before name - {name_copy}");
    name.push_str(" lol");
    println!("mut after name - {name}");
}

fn learn_slice()  {
    let num = &[1, 2, 3, 4];
    println!("slice num is {:?}", num);

    let names = &["abc", "lol", "ok"];
    println!("slice &str are {:?}", names);

    let str_names =  &["abc".to_string(), "lol".to_string(), "ok".to_string()];
    println!("slice string are {:?}", str_names);
}

fn learn_tuple()  {
    let tuple = ("abc", 1, 2.0);
    println!("tuple values are {:?}", tuple);
}

fn learn_array(){
    let arr = [1, 23, 4, 5];
    println!("array values are {:?}", arr);
}