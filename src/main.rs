use std::{
    env,
    error::Error,
    fs,
    io::{BufRead, BufReader},
    process,
};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1)
    });

    if let Err(e) = run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}

struct Config {
    file_path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("Not enough arguments");
        }

        let file_path = args[1].clone();
        Ok(Config { file_path })
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    const CAP: usize = 256 * 16;

    let file = fs::File::open(config.file_path)?;

    let mut reader = BufReader::with_capacity(CAP, file);

    loop {
        let lenght = {
            let buffer = reader.fill_buf()?;

            let mut i: u32 = 0;

            buffer.iter().for_each(|byte| {
                print!("{byte:02x} ");

                i += 1;
                if i == 16 {
                    i = 0;
                    println!()
                }
            });

            buffer.len()
        };

        if lenght == 0 {
            break;
        }
        reader.consume(lenght);
    }

    println!();

    Ok(())
}
