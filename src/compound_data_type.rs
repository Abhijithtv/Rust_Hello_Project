use crate::display_helper;

pub fn compound_data_type_learn(){
let context: &str = "COMPOUND DATA TYPE";
    display_helper::start(context);
    assign_array();
    assign_tuple();
    assign_slice();
    assign_mut_string();
    display_helper::end(context);
}

fn assign_mut_string(){
    let mut name = String:: from("GHO");
    let name_copy = &name;
    println!("mut before name - {name}");
    println!("mut copy before name - {name_copy}");
    name.push_str(" lol");
    println!("mut after name - {name}");
}

fn assign_slice()  {
    let num = &[1, 2, 3, 4];
    println!("slice num is {:?}", num);

    let names = &["abc", "lol", "ok"];
    println!("slice &str are {:?}", names);

    let str_names =  &["abc".to_string(), "lol".to_string(), "ok".to_string()];
    println!("slice string are {:?}", str_names);
}

fn assign_tuple()  {
    let tuple = ("abc", 1, 2.0);
    println!("tuple values are {:?}", tuple);
}

fn assign_array(){
    let arr = [1, 23, 4, 5];
    println!("array values are {:?}", arr);
}