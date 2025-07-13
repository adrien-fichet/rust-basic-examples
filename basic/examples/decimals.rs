// https://docs.rs/rust_decimal/latest/rust_decimal/

use rust_decimal::prelude::*;
use rust_decimal_macros::dec;

#[derive(Debug)]
enum Currency {
    Euro(Decimal),
}

impl Currency {
    fn value(&self) -> &Decimal {
        match &self {
            Self::Euro(value) => value,
        }
    }

    fn taxed(&self, tax: Decimal) -> Decimal {
        match &self {
            Self::Euro(value) => *value + *value * tax,
        }
    }
}

fn main() {
    let potato = Currency::Euro(dec!(42.58));
    let carrot = Currency::Euro(dec!(57.42));
    println!("{potato:?}, {carrot:?}");
    assert_eq!(potato.value() + carrot.value(), dec!(100.00));

    let tax = dec!(0.1);
    assert_eq!(potato.taxed(tax), dec!(42.58) + dec!(4.258));
}
