use std::env;
use std::fs::File;
use std::io::Error;
use std::io::Write;
use std::path::Path;

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Invalid number of arguments. Usage: `fizer [filename] [filesize]`. Example: `fizer file.x 72MB`");
        std::process::exit(0);
    }
    let config = Config::init(&args);
    run(config);
    Ok(())
}

struct Config {
    file_name: String,
    file_size: String,
}

impl Config {
    pub fn init(args: &[String]) -> Config {
        let file_name = args[1].to_string();
        let file_size = args[2].to_string();
        Config {
            file_name,
            file_size,
        }
    }
}

fn convert_file_size_to_bytes(file_size: String) -> String {
    match file_size.parse::<u32>() {
        Ok(_) => file_size,
        Err(_) => match file_size {
            _ if file_size[file_size.len() - 2..].to_string() == "GB" => {
                format!("{}{}", &file_size[..file_size.len() - 2], "0".repeat(9))
            }
            _ if file_size[file_size.len() - 2..].to_string() == "MB" => {
                format!("{}{}", &file_size[..file_size.len() - 2], "0".repeat(6))
            }
            _ if file_size[file_size.len() - 2..].to_string() == "KB" => {
                format!("{}{}", &file_size[..file_size.len() - 2], "0".repeat(3))
            }
            _ if file_size[file_size.len() - 1..].to_string() == "B" => {
                file_size[..file_size.len() - 1].to_string()
            }
            _ => {
                eprintln!("Invalid filesize. Accepted args: `B`, `KB`, `MB`, `GB`");
                std::process::exit(0);
            }
        },
    }
}

fn run(config: Config) {
    let file_content_size = "0".repeat(
        convert_file_size_to_bytes(config.file_size)
            .parse()
            .unwrap(),
    );

    let mut file = match File::create(Path::new(&config.file_name)) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Failed to create file: {}", e);
            std::process::exit(0);
        }
    };

    match file.write_all(file_content_size.as_bytes()) {
        Ok(_) => println!("Creating file success!"),
        Err(e) => {
            eprintln!("Failed to write to file: {}", e);
            std::process::exit(0);
        }
    }
}
