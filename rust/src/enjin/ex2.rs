trait Parent {
    fn call(&self) {
        print!("P");
    }
}

trait Child: Parent {
    fn call(&self) {
        print!("C");
    }
}

struct Mixed;
impl Parent for Mixed {}
impl Child for Mixed {}

fn dynamic_dispatch(p: &dyn Parent) {
    p.call();
}

fn static_dispatch<T: Parent>(p: &T) {
    p.call();
}

// won't compile due to: multiple `call` found
// fn dynamic_dispatch2(p: &dyn Child) {
//     p.call();
// }

// won't compile due to: multiple `call` found
// fn static_dispatch2<T: Child>(p: &T) {
//     p.call();
// }

// prints PP
fn main() {
    dynamic_dispatch(&Mixed);   // P
    static_dispatch(&Mixed);    // P
}
