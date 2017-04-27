fn main(){
	println!("Hello, world!");

	let my_enum = MyEnum::Else;

	my_match(my_enum);
	print_int(2);


	enum MyEnum {
		MyPrint,
		Else
	}

	fn my_match(msg:MyEnum){
		match msg{
			MyEnum::MyPrint => println!("In My Print!"),
			_ => println!("In Else")
		}
	}

	// This method receives an integer and print a string representation of it
	fn print_int(my_int: i32) {
		match my_int {
			1 => println!("One"),
			2 => println!("Two"),
			_ => println!("Not one or two")
		}
	}

}