struct Foo {
    foo: fn()
}

impl Foo {
    fn foo(&self) {
        print!("X");
    }

    fn bar(&self) {
        print!("Z");
    }
}

// prints XYZ
fn main() {
    let f = || print!("Y");
    let foo = Foo { foo: f };
    foo.foo();    // method: X
    (foo.foo)();  // field:  Y
    foo.bar();    // method: Z
    // (foo.bar)();  -- get error: method, not a field
}
