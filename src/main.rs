use seahorse::{App, Context, Flag, FlagType};
use std::{env, process::exit};

use num_bigint::{BigUint, RandBigInt, ToBigUint};
use num_traits::{One, Zero};

use std::io::{self, Write};

fn main() {
    // cli
    let args: Vec<String> = env::args().collect();

    let app = App::new(env!("CARGO_PKG_NAME"))
        .description(env!("CARGO_PKG_DESCRIPTION"))
        .version(env!("CARGO_PKG_VERSION"))
        .usage(format!("{} [flags] <number>", env!("CARGO_PKG_NAME")))
        .flag(
            Flag::new("random", FlagType::Bool)
                .description("Test a random big number(may be very slow)")
                .alias("r"),
        )
	.flag(
	    Flag::new("random-u8", FlagType::Bool)
		.description("Take a random u8 number")
		.alias("u8")
	)
	.flag(
	    Flag::new("random-u16", FlagType::Bool)
		.description("Take a random u16 number")
		.alias("u16")
	)
        .flag(
	    Flag::new("random-u32", FlagType::Bool)
		.description("Take a random u32 number(extremely big)")
		.alias("u32")
	)
	.flag(
	    Flag::new("random-u64", FlagType::Bool)
		.description("Take a random u64 number(extremely extremely big. you probably will not be able to run with this option!)")
		.alias("u64")
	)
        .flag(
            Flag::new("verbose", FlagType::Bool)
                .description("Log everything that's going on(might cause performance issues)")
                .alias("v"),
        )
        .flag(
            Flag::new("optimised-odd-algorithm", FlagType::Bool)
                .description(
                    "Optimised odd algorithm(instead of '3k+1' use '2k+2'. the end result is the same)",
                )
                .alias("o"),
        )
        .action(action);

    app.run(args);
    exit(0);
}

fn action(c: &Context) {
    let optimised_odd_flag: bool = c.bool_flag("optimised-odd-algorithm");
    let verbose_flag: bool = c.bool_flag("verbose");
    let random_flag: bool = c.bool_flag("random");
    let random_u8_flag: bool = c.bool_flag("random-u8");
    let random_u16_flag: bool = c.bool_flag("random-u16");
    let random_u32_flag: bool = c.bool_flag("random-u32");
    let random_u64_flag: bool = c.bool_flag("random-u64");
    let mut input_number: BigUint = if random_flag {
        let mut rng = rand::thread_rng();
        rng.gen_biguint(if random_u64_flag {
            u64::MAX
        } else if random_u32_flag {
            u32::MAX.into()
        } else if random_u16_flag {
            u16::MAX.into()
        } else if random_u8_flag {
            u8::MAX.into()
        } else {
            u16::MAX.into()
        })
    } else {
        let args: Vec<&str> = c.args.iter().map(|arg| arg.as_str()).collect();
        if let Ok(number) = args.join("").parse() {
            number
        } else {
            eprintln!("This is not a supported operation");
            exit(1);
        }
    };

    let mut tries: BigUint = BigUint::zero();

    // to avoid repeated computations
    let two: BigUint = 2.to_biguint().unwrap();

    let odd_algorithm: fn(&mut BigUint) = if optimised_odd_flag {
        optimised_odd
    } else {
        odd
    };

    let print_algorithm: fn(&BigUint, &BigUint) = if verbose_flag {
        verbose_printing
    } else {
        regular_printing
    };

    while !input_number.is_one() && !input_number.is_zero() {
        if (&input_number % &two).is_zero() {
            // even
            even(&mut input_number)
        } else {
            // odd
            odd_algorithm(&mut input_number)
        };

        tries += BigUint::one();

        print_algorithm(&tries, &input_number);
    }

    if !verbose_flag {
        println!("\nEnd Result: {input_number}");
    }
}

#[inline(always)]
fn even(number: &mut BigUint) {
    *number >>= 1;
}

// original algorithm - 3k+1
#[inline(always)]
fn odd(number: &mut BigUint) {
    *number *= BigUint::from(3u8);
    *number += BigUint::one();
}

// optimised algorithm - 2k+2
#[inline(always)]
fn optimised_odd(number: &mut BigUint) {
    *number <<= 1;
    *number += BigUint::from(2u8);
}

#[inline(always)]
fn verbose_printing(tries: &BigUint, number: &BigUint) {
    println!("{tries}) {number}");
}

#[inline(always)]
fn regular_printing(tries: &BigUint, _number: &BigUint) {
    print!("\rTries: {tries}");
    io::stdout().flush().unwrap();
}
