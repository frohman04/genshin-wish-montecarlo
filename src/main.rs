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
        .arg(
            Arg::new("bucket-size")
                .short('b')
                .long("bucket-size")
                .num_args(1)
                .value_parser(clap::value_parser!(u8))
                .help("If specified, bucket results into buckets of the specified size"),
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

    if let Some(bucket_size) = matches.get_one::<u8>("bucket-size") {
        let max_pity = sim.get_max_pity();

        let num_buckets = max_pity as f64 / *bucket_size as f64;
        if num_buckets.round() != num_buckets {
            clap::Error::raw(
                ErrorKind::InvalidValue,
                format!("--bucket-size must be an integral divisor of {}", max_pity),
            )
            .exit();
        }

        let mut buckets = vec![0; num_buckets as usize];

        for _run_i in 0..iterations {
            let mut wish_count: u8 = 1;
            while !sim.wish() {
                wish_count += 1;
            }
            buckets[(wish_count as f64 / *bucket_size as f64).floor() as usize] += 1;
        }

        println!("min_wishes,max_wishes,num_limited_wins");
        for (i, count) in buckets.iter().enumerate() {
            println!(
                "{},{},{}",
                i * *bucket_size as usize,
                (i + 1) * *bucket_size as usize,
                count
            );
        }
    } else {
        for _run_i in 0..iterations {
            let mut wish_count: u8 = 1;
            while !sim.wish() {
                wish_count += 1;
            }
            println!("{}", wish_count);
        }
    }
}
