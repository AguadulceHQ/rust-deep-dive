// this annotation indictes that the macro will be available whenever the crate in which the macro is defined is brought into scope
#[macro_export]
// start actual macro definition
macro_rules! list {
    // body of the macro starts here
    // matching pattern enclosed in () and starts with $ to declare a variable that contains the subsequent Rust code
    // $x:expr matches any expression and gives the name $x to it
    // , states that a comma separator could optionally be there
    // * states that the pattern matches zero or more of whatever precedes *
    ( $( $x:expr ),* ) => // arm with one matching pattern
    {
        let mut temp_list = Vec::new();
        $(
            temp_list.push($x);
        )*
        temp_list
    }
}
// vec![1, 2, 3] matches the above pattern three times with expressions 1, 2 and 3
