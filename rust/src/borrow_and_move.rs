struct X(u32);

impl X {
    #[allow(dead_code)]
    fn moved(self) {
        println!("move: {}", self.0)
    }

    #[allow(dead_code)]
    fn borrowed(&self) {
        println!("borrow: {}", self.0)
    }

    #[allow(dead_code)]
    fn borrowed_mut(&mut self) {
        println!("borrow_mut: {}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut x = X(100);
        x.borrowed_mut();
        x.borrowed();
        x.borrowed();
    }
}