trait OrTrait {
    fn foo(self);
}

struct Check;

impl OrTrait for &Check {
    fn foo(self) {
        print!("X");
    }
}

impl OrTrait for &&&Check {
    fn foo(self) {
        print!("Y");
    }
}

// prints XXYYY
fn main() {
    let zero = Check;
    let one = &Check;
    let two = &&Check;
    let three = &&&Check;
    let four = &&&&Check;
    zero.foo();   // X
    one.foo();    // X
    two.foo();    // Y
    three.foo();  // Y
    four.foo();   // Y
}