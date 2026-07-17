use crate::learners::{compound_data_type, function, primary_data_type};

mod learners;
mod helpers;

fn main() {
    primary_data_type::learn_primary_dataype();    
    compound_data_type::learn_compound_data_type();
    function::learn_function();
}

