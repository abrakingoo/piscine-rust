pub fn sum(a: u8, b: u8) -> u8 {
    let res = a + b;

    if res < a {
        panic!("ERROR: attempt to add with overflow")
    }

    res
}

pub fn diff(a: i16, b: i16) -> i16 {
    let diff = a - b;

    if (b > 0 && a < i16::MIN + b) || (b < 0 && a > i16::MAX + b) {
        panic!("ERROR: attempt to subtract with overflow");
    }

    diff
}

pub fn pro(a: i8, b: i8) -> i8 {
    a * b
}

pub fn quo(a: f32, b: f32) -> f32 {
    if b == 0.0 {
        panic!("ERROR: attempt to divide by zero");
    }

    a / b
}

pub fn rem(a: f32, b: f32) -> f32 {
    if b == 0.0 {
        panic!("ERROR: attempt to divide by zero");
    }

    a % b
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        
        let result = sum(100, 150);
        assert_eq!(result, 250);

        let result = std::panic::catch_unwind(|| {
            sum(1, 255); 
        });

        assert!(result.is_err()); 
    }
}

