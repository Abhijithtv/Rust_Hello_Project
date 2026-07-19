use crate::learners::{compound_data_type, control_flow, enum_type, error_handling, function, ownership, primary_data_type, shadowing, struct_data_type};

mod learners;
mod helpers;

fn main() {
    primary_data_type::learn_primary_dataype();    
    compound_data_type::learn_compound_data_type();
    function::learn_function();
    ownership::learn_ownership();
    struct_data_type::learn_struct_data_type();
    shadowing::learn_shadowing();
    control_flow::learn_control_flow();
    enum_type::learn_enum();
    error_handling::learn_error_handling();
}

