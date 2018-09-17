#[macro_use]
extern crate proc_macro_example;
#[macro_use]
extern crate proc_macro_example_level2;

#[derive(MacroExampleLevel1)]
struct Sample;

pub trait MacroExample {
    fn hello_macro();
}

fn main() {
    Sample::hello_macro();
}
