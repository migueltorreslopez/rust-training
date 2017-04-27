fn main(){
	println!("Hello, world!");

	let my_enum = MyEnum::Else;

	my_match(my_enum);

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

}