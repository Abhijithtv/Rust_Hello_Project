use crate::helpers::display_helper;

const CONTEXT: &str = "ENUM-TYPE";

#[derive(Debug)]
enum Status{
    Unknown,
    Success,
    Good
}

pub fn learn_enum(){
    display_helper::start(CONTEXT);
    learn_passing_enum();
    learn_mixed_enum();
    display_helper::end(CONTEXT);
}

fn learn_mixed_enum() {
    #[derive(Debug)]
    enum Mixed{
        Age(i32),
        Name(String),
        AddressTuple(String, i32)
    }

    let age =  Mixed::Age(1);
    let name = Mixed::Name("abhi".to_string());
    let address = Mixed::AddressTuple("some place".to_string(), 670307);
    println!("name is {:?}, age is {:?} and address is {:?}", name, age, address);
}

fn learn_passing_enum(){
    let status = Status::Good;
    eval(&status);
}

fn eval(status: &Status){
    println!("Status is {:?}", status);
}

