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

    fastrand::seed(400);
    for run in 0..*matches.get_one::<usize>("ITERATIONS").unwrap() {
        for wish_count in 1..=90 {
            let pct_win = if wish_count < 74 {
                0.006
            } else {
                // this is slightly >1 for wish 90, but that's ok because the RNG will not generate
                // a value greater than 1
                0.006 + 0.0585 * ((wish_count - 73) as f64)
            };

            if fastrand::f64() < pct_win {
                println!("Run {}: Won on wish {}", run + 1, wish_count);
                break;
            }
        }
    }
}
