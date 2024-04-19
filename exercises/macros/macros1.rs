// macros1.rs
//
// Execute `rustlings hint macros1` or use the `hint` watch subcommand for a
// hint.



macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

fn main() {
    my_macro!();
}
/*
在Rust中，宏调用需要在宏名称后面加上感叹号!来表示它是一个宏调用。这是为了与普通函数调用进行区分。感叹号告诉编译器这是一个宏调用，而不是普通的函数调用。

在你的代码中，my_macro!();是在调用宏my_macro，因此需要加上感叹号来标识这是一个宏调用。如果省略感叹号，编译器会将其解释为普通的函数调用，而不是宏调用，这样就会导致编译错误。

因此，在Rust中，当你使用宏时，记得在宏名称后面加上感叹号!来表示它是一个宏调用。
*/