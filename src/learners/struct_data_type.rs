use crate::helpers::display_helper;

const CONTEXT: &str = "STRUCT-DATATYPE";

pub fn learn_struct_data_type(){
    display_helper::start(CONTEXT);

    //makes all member mutable
    let mut user = BankUser{
        name :String::from("abhijith"),
        money : 1000.3
    };
    user.add_money(100.0);
    user.print_balance();
    display_helper::end(CONTEXT);
}

struct BankUser{
    name: String,
    money: f64 
}

impl BankUser {
    fn add_money(&mut self, amt: f64){
        self.money += amt;
        println!("money added {amt}");
    }

    fn print_balance(&self){
        println!("balance for user {} is {}", self.name, self.money)
    }
}