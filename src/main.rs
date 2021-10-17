#![allow(unused)]

enum Scale {
    Celsius,
    Fahrenheit,
}

struct Temperature {
    degrees: f32,
    scale: Scale,
}

impl Temperature {
    fn new(degrees: f32) -> Self {
        Temperature {
            degrees,
            scale: Scale::Celsius,
        }
    }

    fn as_celsius(&self) -> f32 {
        todo!();
    }

    fn as_fahrenheit(&self) -> f32 {
        todo!();
    }
}

fn main() {
    let temp = Temperature::new(20.0);

    println!("{}C is {}F", temp.as_celsius(), temp.as_fahrenheit());
}

#[test]
fn one_degree() {
    let cold = Temperature::new(1.0);
    assert!((cold.as_fahrenheit() - 33.8) < 0.01);
    assert!((cold.as_fahrenheit() - 33.8) >= 0.0);
}

#[test]
fn boiling() {
    let hot = Temperature::new(100.0);
    assert!((hot.as_fahrenheit() - 212.0) < 0.01);
    assert!((hot.as_fahrenheit() - 212.0) >= 0.0);
}

