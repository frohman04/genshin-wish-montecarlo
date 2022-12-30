mod character;
mod sim;
mod weapon;

use crate::character::CharacterBannerSimParams;
use crate::sim::BannerSim;
use crate::weapon::WeaponBannerSimParams;
use clap::error::ErrorKind;
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
        .arg(
            Arg::new("character")
                .short('c')
                .long("character")
                .num_args(0)
                .conflicts_with("weapon")
                .help("Run simulations on the character banner"),
        )
        .arg(
            Arg::new("weapon")
                .short('w')
                .long("weapon")
                .num_args(0)
                .conflicts_with("character")
                .help("Run simulations on the weapon banner"),
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

    let mut sim: BannerSim = if matches.get_flag("character") {
        CharacterBannerSimParams::new_sim()
    } else if matches.get_flag("weapon") {
        WeaponBannerSimParams::new_sim()
    } else {
        clap::Error::raw(
            ErrorKind::MissingRequiredArgument,
            "Must specify either --character or --weapon",
        )
        .exit();
    };

    for _run_i in 0..iterations {
        let mut wish_count: u8 = 1;
        while !sim.wish() {
            wish_count += 1;
        }
        println!("{}", wish_count);
    }
}
