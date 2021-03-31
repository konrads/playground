// traits
trait Animal {
    fn snuggle(&self);
    fn eat(&mut self);
}

trait Cat: Animal {
    fn meow(&self);
}

trait Dog: Animal {
    fn bark(&self);
}

// structs (impls)
struct CatImpl {}
struct DogImpl {}

impl Animal for CatImpl {
    fn snuggle(&self) {
        println!("Cat snuggled!")
    }

    fn eat(&mut self) {
        println!("Cat snuggled!")
    }
}

impl Cat for CatImpl {
    fn meow(&self) {
        println!("Dog meowed!")
    }
}

impl Dog for DogImpl {
    fn bark(&self) {
        println!("Dog barked!")
    }
}

impl Animal for DogImpl {
    fn snuggle(&self) {
        println!("Dog snuggled!")
    }

    fn eat(&mut self) {
        println!("Dog ate!")
    }
}

// fns
fn love<T: Animal>(pet: T) {
    pet.snuggle();
}

fn love2(pet: Box<dyn Animal>) {
    pet.snuggle();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn variance_test() {
        println!("variance test");
        let dog = DogImpl {};
        let cat = CatImpl {};
        love(dog);
        love(cat);
    }

    #[test]
    fn variance_2_test() {
        println!("variance test 2");
        let dog = DogImpl {};
        let cat = CatImpl {};
        love2(Box::new(dog));
        love2(Box::new(cat));
    }

    #[test]
    fn heterogeneous_collection_test() {
        println!("heterogeneous collection test");
        let mut v: Vec<&dyn Animal> = Vec::new();
        let mut v2: Vec<Box<dyn Animal>> = Vec::new();
        // let mut v3: Vec<dyn Animal> = Vec::new();
        let dog = DogImpl {};
        let cat = CatImpl {};
        v.push(&dog);
        v.push(&cat);
        v.push(&dog);
        v.push(&cat);
        v2.push(Box::new(dog));
        v2.push(Box::new(cat));
        // v2.push(Box::new(dog));
        // v2.push(Box::new(cat));
        // v3.push(dog);
        // v3.push(cat);
        // println!("v: {:?}", v);
        // println!("v: {:?}", v2);
    }
}
