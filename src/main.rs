

fn main(){
	println!("Hello, world!");

	let my_enum = MyEnum::Else;

	my_match(my_enum);
	print_int(2);
	// Create result
	let res = Ok(10);
	ignore_binding(res);
	{
		let res = Err(11);
		ignore_binding(res);
	}
	ignore_binding(res);

	// Call Impl
	let myImpl = MyImpl {myInt: 3};
	let implResult = myImpl.increment(2);

	println!("Impl result: {}", implResult);
}

enum MyEnum {
	MyPrint,
	Else
}

struct MyImpl {
	myInt: i32,
}

impl MyImpl {
	fn increment(&self, inc : i32) -> i32{
		self.myInt + inc
	}
}

fn my_match(msg: MyEnum){
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

fn ignore_binding(res: Result<i32, i32>) {
	match res{
		Ok(value) => println!("got value: {}", value),
		Err(_) => println!("got error")
	}
}