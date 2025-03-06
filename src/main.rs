use std::{env, num::NonZero};

fn main() {
    let Some(product_height) = env::args().nth(2) else {
        eprintln!("provide an integer as the first argument ('product_height') to this sample app");
        return;
    };

    let product_height = product_height
        .parse::<u32>()
        .expect("first argument ('product_height') must be an integer");

    // We validate that integers are non-zero as soon as possible and keep them in the
    // NonZero<T> wrapper type after that, to avoid further unnecessary zero-checks.
    let Some(product_height) = NonZero::new(product_height) else {
        eprintln!("first argument ('product_height') must be non-zero");
        return;
    };

    if !item_fits_exactly_in_packaging(product_height) {
        eprintln!("product does not fit in packaging");
        return;
    }

    println!("product fits in packaging");
}

fn item_fits_exactly_in_packaging(height: NonZero<u32>) -> bool {
    // No need to worry about division by zero because we accept NonZero input.
    // This means we can avoid checking the denominator in every call to this function.
    1000 % height.get() == 0
}

#[cfg(test)]
mod without_nonzero {
    use std::num::NonZero;

    fn item_fits_exactly_in_packaging(height: NonZero<u32>) -> bool {
        // No need to worry about division by zero because we accept NonZero input.
        // This means we can avoid checking the denominator in every call to this function.
        1000 % height.get() == 0
    }

    #[test]
    fn item_fits_exactly_in_packaging_if_divides_1000() {
        // The packaging has a height of 1000, so any integer that divides it evenly will fit.
        assert!(item_fits_exactly_in_packaging(NonZero::new(1).unwrap()));
        assert!(!item_fits_exactly_in_packaging(NonZero::new(3).unwrap()));
        assert!(item_fits_exactly_in_packaging(NonZero::new(25).unwrap()));
        assert!(!item_fits_exactly_in_packaging(NonZero::new(999).unwrap()));
        assert!(item_fits_exactly_in_packaging(NonZero::new(1000).unwrap()));
    }
}

#[cfg(test)]
mod with_nonzero_today {
    fn item_fits_exactly_in_packaging(height: u32) -> bool {
        assert_ne!(0, height, "cannot package a product with a height of zero");
        1000 % height == 0
    }

    #[test]
    fn item_fits_exactly_in_packaging_if_divides_1000() {
        // The packaging has a height of 1000, so any integer that divides it evenly will fit.
        assert!(item_fits_exactly_in_packaging(1));
        assert!(!item_fits_exactly_in_packaging(3));
        assert!(item_fits_exactly_in_packaging(25));
        assert!(!item_fits_exactly_in_packaging(999));
        assert!(item_fits_exactly_in_packaging(1000));
    }
}

#[cfg(test)]
mod with_macro {
    macro_rules! nonzero {
        ($x:literal) => {{
            // Compile-time check: if $x is 0, panic to cause a compilation error
            const _: () = if $x == 0 {
                ::std::panic!("zero is not a valid value for a NonZero type")
            };
            // SAFETY: We already validated it is not zero.
            unsafe { ::std::num::NonZero::new_unchecked($x) }
        }};
    }

    use std::num::NonZero;

    fn item_fits_exactly_in_packaging(height: NonZero<u32>) -> bool {
        // No need to worry about division by zero because we accept NonZero input.
        // This means we can avoid checking the denominator in every call to this function.
        1000 % height.get() == 0
    }

    #[test]
    fn item_fits_exactly_in_packaging_if_divides_1000() {
        // The packaging has a height of 1000, so any integer that divides it evenly will fit.
        assert!(item_fits_exactly_in_packaging(nonzero!(1)));
        assert!(!item_fits_exactly_in_packaging(nonzero!(3)));
        assert!(item_fits_exactly_in_packaging(nonzero!(25)));
        assert!(!item_fits_exactly_in_packaging(nonzero!(999)));
        assert!(item_fits_exactly_in_packaging(nonzero!(1000)));
    }
}
