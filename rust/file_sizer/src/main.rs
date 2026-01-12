use std::env;
use std::fmt;

#[warn(dead_code)]
#[derive(Debug)]
struct Sizes {
    bytes: String,
    kilobytes: String,
    megabytes: String,
    gigabytes: String,
}

impl Sizes {
    fn new(size: u64) -> Self {
        Self {
            bytes: format!("{} bytes", size),
            kilobytes: format!("{} kilobytes", size as f64 / 1000.0),
            megabytes: format!("{} megabytes", size as f64 / 1_000_000.0),
            gigabytes: format!("{} gigabytes", size as f64 / 1_000_000_000.0),
        }
    }
}

impl fmt::Display for Sizes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Sizes ({}, {}, {}, {})", self.bytes, self.kilobytes, self.megabytes, self.gigabytes)
    }
}

fn extract_values(args: Vec<String>) -> (f64, String) {
    if args.len() < 2 {
        eprintln!("No file size provided");
        std::process::exit(1);
    } else if args.len() > 2 {
        eprintln!("Too many arguments provided");
        std::process::exit(1);
    }

    let input = args[1].trim();
    let mut parts = input.split_whitespace();
    let size_str = parts.next().unwrap_or("");

    let size: f64 = match size_str.parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Invalid number provided for file size");
            std::process::exit(1);
        }
    };

    let unit: String = match parts.next() {
        Some(u) => u.to_lowercase(),
        _ => {
            eprintln!("Too many arguments provided");
            std::process::exit(1);
        }
    };

    return (size, unit)
}


fn main() {
    let args: Vec<String> = env::args().collect();

    let (size, unit) = extract_values(args);

    let size_in_bytes: u64 = match unit.as_str() {
        "b" => size as u64,
        "kb" => (size * 1000.0) as u64,
        "mb" => (size * 1_000_000.0) as u64,
        "gb" => (size * 1_000_000_000.0) as u64,
        _ => {
            eprintln!("Invalid unit provided. Use b, kb, mb, or gb.");
            std::process::exit(1);
        }
    };

    let sizes = Sizes::new(size_in_bytes);
    println!("{}", sizes);
}