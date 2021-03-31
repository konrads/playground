#[derive(Debug)]
enum List1<T> {
    Cons1(T, Box<List1<T>>),
    Nil,
}

fn ll1() {
    let list: List1<i32> = List1::Cons1(1, Box::new(List1::Cons1(2, Box::new(List1::Nil))));
    println!("{:?}", list);

    let list: List1<i32> = List1::Nil;
    println!("{:?}", list);
}

////////////////
pub struct List2 {
    head: Link2,
}

enum Link2 {
    Empty2,
    More2(Box<Node2>),
}

struct Node2 {
    elem: i32,
    next: Link2,
}

////////////////
pub struct List3<T> {
    head: Link3<T>,
}

type Link3<T> = Option<Box<Node3<T>>>;

struct Node3<T> {
    elem: T,
    next: Link3<T>,
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ll1_test() {
        ll1()
    }
}