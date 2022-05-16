use finite_field::DefaultNumber;
use primitive_types::U512;

use crate::finite_field::FiniteFieldElement;

mod finite_field;

impl DefaultNumber for U512 {
    fn zero() -> Self {
        U512::from(0)
    }

    fn one() -> Self {
        U512::from(1)
    }
}

fn main() {
    let value = FiniteFieldElement::new(U512::from(2u8), U512::MAX);
    let value2 = FiniteFieldElement::new(U512::from(3u8), U512::MAX);
    println!("{:?}", value + value2);
    println!("{:?}", value - value2);
    println!("{:?}", value * value2);
}
