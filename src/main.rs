mod character;
mod sim;
mod weapon;

use crate::character::CharacterBannerSim;
use crate::sim::BannerSim;
use crate::weapon::WeaponBannerSim;
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

    let mut sim = CharacterBannerSim::default();
    for run_i in 0..iterations {
        let mut wish_count: u8 = 1;
        while !sim.wish() {
            wish_count += 1;
        }
        println!(
            "Run {}: Won limited 5* character on wish {}",
            run_i + 1,
            wish_count
        );
    }

    let mut sim = WeaponBannerSim::default();
    for run_i in 0..iterations {
        let mut wish_count: u8 = 1;
        while !sim.wish() {
            wish_count += 1;
        }
        println!(
            "Run {}: Won limited 5* weapon on wish {}",
            run_i + 1,
            wish_count
        );
    }
}
