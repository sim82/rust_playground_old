struct Bla1 {
	i : int
}

impl Drop for Bla1 {
	fn drop( &mut self ) {
		println!{"drop Bla1"};
	}
}

struct Bla2 {
	i : int
}


impl Drop for Bla2 {
	fn drop( &mut self ) {
		println!{"drop Bla2"};
	}
}


struct Blub {
	bla1 : Bla1,
	bla2 : Bla2
}

enum Msg {
    Msg1( f64),
    Msg2( uint )
}


fn main() {
	let blub = Blub{ bla1 : Bla1{ i : 1 }, bla2 : Bla2{ i : 2 } };

	let (tx, rx) = channel();
	spawn(proc() {
	  tx.send(Msg1(1.0));
	  tx.send(Msg2(666));
	});

	for i in range(0,2) {
	//	println!("rx: {}", rx.recv())
		match rx.recv() {
			Msg1(f) => println!( "msg1" ),
			Msg2(i) => println!( "msg2" )
		}
	}
	// assert_eq!(rx.recv(), 10);
	
}