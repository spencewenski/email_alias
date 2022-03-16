use argparse::{ArgumentParser, Store, StoreFalse};
use rand::Rng;

fn main() {
    let args = Arguments::parse_args();

    const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyz0123456789";
    let mut rng = rand::thread_rng();

    let suffix = if args.random_suffix {
        let s: String = (0..args.random_suffix_length)
            .map(|_| {
                let i = rng.gen_range(0, CHARSET.len());
                CHARSET[i] as char
            })
            .collect();
        format!("{}{}", args.separator, s)
    } else {
        "".to_string()
    };

    let alias = format!("{}{}@{}", args.domain, suffix, args.email_domain);
    let alias = alias.to_ascii_lowercase();

    println!("{}", alias);
}

#[derive(Debug)]
pub struct Arguments {
    pub domain: String,
    pub email_domain: String,
    pub random_suffix: bool,
    pub random_suffix_length: u32,
    pub separator: String,
}

impl Arguments {
    fn new() -> Arguments {
        Arguments {
            domain: String::new(),
            email_domain: String::new(),
            random_suffix: true,
            random_suffix_length: 10,
            separator: "_".to_string(),
        }
    }

    fn parse_args() -> Arguments {
        let mut args = Arguments::new();
        {
            let mut ap = ArgumentParser::new();

            ap.set_description("Generate a new email alias for the given domain.");

            ap.refer(&mut args.domain)
                .add_option(
                    &["-d", "--domain"],
                    Store,
                    "Name of the domain to create an email alias for, e.g. mozilla",
                )
                .required();

            ap.refer(&mut args.email_domain)
                .add_option(
                    &["-e", "--email"],
                    Store,
                    "Name of the base email domain, e.g. spencewenski.com",
                )
                .required();

            ap.refer(&mut args.random_suffix).add_option(
                &["-n", "--no-suffix"],
                StoreFalse,
                "Disable the random suffix.",
            );

            ap.refer(&mut args.random_suffix_length).add_option(
                &["-l", "--random-suffix-length"],
                Store,
                "Length of the random suffix. Default is 10.",
            );

            ap.refer(&mut args.separator).add_option(
                &["-s", "--separator"],
                Store,
                "Separator between the domain and the random suffix. Default is '_'",
            );

            ap.parse_args_or_exit();
        }

        args
    }
}
