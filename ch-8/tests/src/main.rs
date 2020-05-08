fn main() {
    println!("Hello, world!");
}

#[test]
fn math_works() {
    let x: i32 = 1;
    assert!(x.is_positive());
    assert_eq!(x + 1, 2);
}

/// This test passes only if division by zero causes a panic,
/// as we claimed in the previous chapter.
#[test]
#[should_panic(expected="divide by zero")]
#[allow(const_err)]
fn test_divide_by_zero_error() {
    let _ = 1 / 0; // should panic!
}

#[cfg(test)]
mod tests {
    fn roughly_equals(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-6
    }

    #[test]
    fn trig_works() {
        use std::f64::consts::PI;
        assert!(roughly_equals(PI.sin(), 0.0));
    }
}
