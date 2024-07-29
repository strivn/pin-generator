// ! # PIN Generator

use clap::{Parser, Subcommand};
use sha2::{Digest, Sha256, Sha384, Sha512};
use sha3::{Sha3_256, Sha3_384, Sha3_512};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Generate {
        #[arg(short, long, default_value_t = 6)]
        digits: u8,

        #[arg(short, long)]
        passphrase: String,

        #[arg(short, long, default_value_t = false)]
        show: bool,
    },
}

trait Hasher {
    fn hash(input: &str) -> String;
}

macro_rules! impl_hasher {
    ($type: ty) => {
        impl Hasher for $type {
            fn hash(input: &str) -> String {
                let mut hasher = <$type>::new();
                hasher.update(input);
                let result = hasher.finalize();
                format!("{:x}", result)
            }
        }
    };
}

impl_hasher!(Sha256);
impl_hasher!(Sha384);
impl_hasher!(Sha512);
impl_hasher!(Sha3_256);
impl_hasher!(Sha3_384);
impl_hasher!(Sha3_512);

fn hash_with<T: Hasher>(input: &str) -> String {
    T::hash(input)
}

macro_rules! println_hash {
    ($name:expr, $func:expr, $digits:expr, $show:expr) => {

        let hash_value = &$func;
        match first_n_consecutive_numeric(hash_value, $digits) {
            Some(x) => {
                if $show {
                    println!("The PIN generated value from {} is: {}, original: {}", $name, x, hash_value)
                } else {
                    println!("The PIN generated value from {} is: {}", $name, x)
                }
            },
            None => println!("There is no available PIN from {}", $name),
        }
    };
}

fn first_n_consecutive_numeric(s: &str, n: usize) -> Option<String> {
    s.chars()
        .collect::<Vec<char>>()
        .windows(n)
        .find(|window| window.iter().all(|c| c.is_numeric()))
        .map(|window| window.iter().collect())
}

fn main() {
    let args = Args::parse();

    match &args.command {
        Commands::Generate {
            digits,
            passphrase,
            show,
        } => {
            let digits: usize = digits.clone().into();
            let show = *show;

            // println!("The command is {}!", args.command);
            println!("The input passphrase is {}", passphrase);
            println!("The number of digits is {}", digits);

            println_hash!("SHA256", hash_with::<Sha256>(passphrase), digits, show);
            println_hash!("SHA384", hash_with::<Sha384>(passphrase), digits, show);
            println_hash!("SHA512", hash_with::<Sha512>(passphrase), digits, show);
            println_hash!("SHA3_256", hash_with::<Sha3_256>(passphrase), digits, show);
            println_hash!("SHA3_384", hash_with::<Sha3_384>(passphrase), digits, show);
            println_hash!("SHA3_512", hash_with::<Sha3_512>(passphrase), digits, show);
        }
    }
}
