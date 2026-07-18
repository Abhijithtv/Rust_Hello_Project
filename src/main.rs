use crate::learners::{compound_data_type, function, ownership, primary_data_type, struct_data_type};

mod learners;
mod helpers;

fn main() {
    primary_data_type::learn_primary_dataype();    
    compound_data_type::learn_compound_data_type();
    function::learn_function();
    ownership::learn_ownership();
    struct_data_type::learn_struct_data_type();
}

