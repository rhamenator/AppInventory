use app_inventory::{decode, parse, to_csv};
use std::{env, fs};

fn main() {
    let args: Vec<_> = env::args().skip(1).collect();
    if args.len() != 2 {
        eprintln!("usage: app-inventory INPUT.txt OUTPUT.csv");
        std::process::exit(2);
    }
    let result = fs::read(&args[0])
        .map_err(|error| error.to_string())
        .and_then(|bytes| decode(&bytes))
        .and_then(|input| {
            let apps = parse(&input);
            if apps.is_empty() {
                return Err("no software entries found".into());
            }
            fs::write(&args[1], to_csv(&apps)).map_err(|error| error.to_string())?;
            Ok(apps.len())
        });
    match result {
        Ok(count) => println!("wrote {count} applications to {}", args[1]),
        Err(error) => {
            eprintln!("error: {error}");
            std::process::exit(1);
        }
    }
}
