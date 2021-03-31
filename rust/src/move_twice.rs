use std::error::Error;

#[derive(Debug)]
struct X {
    x: i32,
    y: String
}

fn get_res(x: &X) -> Result<&X, Box<dyn Error>> {
    let res = if x.x > 0 {
        Ok(x)
    } else {
        Err(format!("invalid x: {:?}", x).into())
    };
    log::error!("res = {:?}", res);
    res
}

#[cfg(test)]
mod tests {
    use super::{get_res, X};

    #[test]
    fn move_twice_test() {
        let x = 10;
        let xx = X { x: x, y: format!("x: {:?}", x)};
        let res = get_res(&xx);
        log::error!("xx: {:?}, res: {:?}", xx, res);
        let xx = X { x: -x, y: format!("x: {:?}", x)};
        let res = get_res(&xx);
        log::error!("xx: {:?}, res: {:?}", xx, res);
    }
}