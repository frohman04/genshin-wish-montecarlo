use clap::{crate_name, crate_version, Arg, Command};

fn main() {
    let matches = Command::new(crate_name!())
        .version(crate_version!())
        .author("Chris Lieb")
        .arg(
            Arg::new("ITERATIONS")
                .required(true)
                .num_args(1)
                .value_parser(clap::value_parser!(usize))
                .help("The number of iterations of MonteCarlo to run"),
        )
        .arg(
            Arg::new("seed")
                .short('s')
                .long("seed")
                .num_args(1)
                .value_parser(clap::value_parser!(u64))
                .help("The RNG seed to use (if unset, will be nondeterministic)"),
        )
        .get_matches();

    match matches.get_one::<u64>("seed") {
        Some(seed) => {
            fastrand::seed(*seed);
            println!("Using seed {}", seed)
        }
        None => println!("Using random seed {}", fastrand::get_seed()),
    }

    let iterations = *matches.get_one::<usize>("ITERATIONS").unwrap();

    for run_i in 0..iterations {
        for wish_count in 1..=90 {
            if is_5s_char_win(wish_count) {
                println!("Run {}: Won on wish {}", run_i + 1, wish_count);
                break;
            }
        }
    }
}

/// Determine if a roll won a 5* character.  This does not mean that the win is for the limited 5*.
fn is_5s_char_win(wish_count: u64) -> bool {
    let pct_win = if wish_count < 74 {
        0.006
    } else {
        // this is slightly >1 for wish 90, but that's ok because the RNG will not generate
        // a value greater than 1
        0.006 + 0.0585 * ((wish_count - 73) as f64)
    };

    fastrand::f64() < pct_win
}
