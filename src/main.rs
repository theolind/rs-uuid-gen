use core::fmt::Write;
use rand;
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(name = "uuid-gen")]
struct Args {
    /// Number of UUIDs to generate
    #[clap(short, long, default_value="1")]
    number: u32,

    /// UUID version to generate.
    /// Supported versions: 4, nil
    #[clap(short, long, default_value="4")]
    version: String,
}

fn nil_bytes() -> [u8; 16] {
    return [0; 16];
}

fn v4_bytes() -> [u8; 16] {
    let mut bytes = rand::random::<[u8; 16]>();
    bytes[6] = (bytes[6] & 0x0f) | 0x40;
    bytes[8] = (bytes[8] & 0x3f) | 0x80;
    return bytes;
}

fn bytes_to_uuid(bytes: [u8; 16]) -> String {
    let mut string = String::with_capacity(36);
    for byte in &bytes {
        match write!(string, "{:02X}", byte) {
            Err(e) => println!("{:?}", e),
            _ => ()
        }
    }
    string.insert(20, '-');
    string.insert(16, '-');
    string.insert(12, '-');
    string.insert(8, '-');
    return string.to_lowercase();
}

fn main() {
    let args = Args::parse();
    
    for _ in 0..args.number {
        match args.version.as_str() {
            "1" => {
                println!("UUID v1 not supported");
                break
            }
            "2" => {
                println!("UUID v2 not supported");
                break
            }
            "3" => {
                println!("UUID v3 not supported");
                break
            }
            "4" => println!("{}", bytes_to_uuid(v4_bytes())),
            "5" => {
                println!("UUID v5 not supported");
                break
            }
            "nil" => println!("{}", bytes_to_uuid(nil_bytes())),
            _ => {
                println!("Unknown UUID version {} supplied", args.version);
                break
            }
        }
    }
}
