

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
	let my_impl = MyImpl {my_int: 3};
	let impl_result = my_impl.increment(2);

	println!("Impl result: {}", impl_result);

	// Closure
	let my_closure = |x| x + 1;
	// call closure function
	let closure_result = my_closure(3);

	println!("Closure result: {}", closure_result);

	call_complex_closure();

}

enum MyEnum {
	MyPrint,
	Else
}

struct MyImpl {
	my_int: i32,
}

impl MyImpl {
	fn increment(&self, inc : i32) -> i32{
		self.my_int + inc
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

fn call_complex_closure(){
	let mut result;
	// Define closure
	let my_closure = |a, b| {
		let mut closure_result = "f";
		match a {
			"m" => closure_result = a,
			_ => closure_result = "unknown",
		};
		closure_result
	};

	result = my_closure("m", "b");

	println!("call_complex_closure result: {}", result);

	assert_eq!("m", result);

	result = my_closure("p", "b");

	println!("call_complex_closure result: {}", result);

	assert_eq!("unknown", result)

}