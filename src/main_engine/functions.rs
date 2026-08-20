use std::collections::HashMap;
pub mod fn_power;

enum FunctionRef {
    BuiltIn(BuiltInFn),
    UserDefined(String), 
}

enum BuiltInFn {
    
}