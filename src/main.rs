use std::{
    env,
    error::Error,
    fs,
    io::{self, BufReader, Read},
    process,
};

fn main() {
    if let Err(e) = run() {
        eprintln!("{e}");
        process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let path = env::args().nth(1).ok_or("usage: hexi <file>|-")?;

    let reader: Box<dyn Read> = if path == "-" {
        Box::new(io::stdin())
    } else {
        Box::new(fs::File::open(path)?)
    };

    let mut reader = BufReader::with_capacity(4096, reader);

    let mut buf = [0u8; 4096];
    let mut offset: u64 = 0;

    loop {
        let n = reader.read(&mut buf)?;
        if n == 0 {
            break;
        }

        for chunk in buf[..n].chunks(16) {
            print_line(offset, chunk);
            offset += chunk.len() as u64;
        }
    }

    Ok(())
}

fn print_line(offset: u64, bytes: &[u8]) {
    print!("{offset:08x}: ");

    for b in bytes {
        print!("{:02x}", b);
    }

    for _ in 0..(16 - bytes.len()) {
        print!("   ")
    }

    print!(" ");

    for &b in bytes {
        let c = if (0x20..=0x7e).contains(&b) {
            b as char
        } else {
            '.'
        };
        print!("{c}")
    }

    println!()
}
