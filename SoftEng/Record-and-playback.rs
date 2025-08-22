// To run this code:
// 1. Create a Rust binary project with any name (`cargo new guess`).
// 2. Set this file as the contents of its file `src/main.rs`.
// 3. Set as contents of its file `Cargo.toml` the following lines:
//   [package]
//   name = "guess"
//   version = "0.1.0"
//   edition = "2024"
//   
//   [dependencies]
//   lazy_static = "1.5.0"
//   rand = "0.9.2"
//   serde = "1.0.219"
//   ron = "0.10.1"
//   
//   [features]
//   recording = []
//   playback = []
// 4. The command `cargo run` is quite similar to the example program described here: [https://doc.rust-lang.org/stable/book/ch02-00-guessing-game-tutorial.html].
// 5. The command `cargo run --features "recording"` has apparently a similar behavior, but actually it also creates a text file named `log.txt`. You can look at its contents.
// 6. The command `cargo run --features "playback"` reads the file `log.txt`, and checks that the behavior of the application is exactly described by that file. It also prints a letter for every record read from that file.

use rnp_interface::{console_output, gen_random_number, get_user_input};
use rnp_lib::init_log_file;
use std::cmp::Ordering;

fn main() {
    init_log_file();
    console_output("Guess the number!\n");

    let secret_number = gen_random_number();

    loop {
        console_output("Please input your guess.\n");

        let guess = get_user_input();

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        console_output(&format!("You guessed: {guess}\n"));

        match guess.cmp(&secret_number) {
            Ordering::Less => console_output("Too small!\n"),
            Ordering::Greater => console_output("Too big!\n"),
            Ordering::Equal => {
                console_output("You win!\n");
                break;
            }
        }
    }
}

mod rnp_interface {
    #[cfg(all(feature = "playback", not(feature = "recording")))]
    use crate::rnp_lib::read_event;

    #[cfg(all(feature = "recording", not(feature = "playback")))]
    use crate::rnp_lib::write_event;

    #[cfg(not(feature = "playback"))]
    use std::io::stdin;

    #[cfg(not(feature = "playback"))]
    use rand::Rng;

    // gen_random_number

    #[cfg(all(feature = "recording", not(feature = "playback")))]
    pub fn gen_random_number() -> u32 {
        let n = rand::rng().random_range(1..=100);
        write_event("random", n);
        n
    }

    #[cfg(all(feature = "playback", not(feature = "recording")))]
    pub fn gen_random_number() -> u32 {
        read_event("random")
    }

    #[cfg(not(any(feature = "recording", feature = "playback")))]
    pub fn gen_random_number() -> u32 {
        rand::rng().random_range(1..=100)
    }

    #[cfg(all(feature = "recording", feature = "playback"))]
    pub fn gen_random_number() -> u32 {
        unreachable!();
    }

    // get_user_input

    #[cfg(all(feature = "recording", not(feature = "playback")))]
    pub fn get_user_input() -> String {
        let mut input = String::new();
        stdin().read_line(&mut input).expect("failed to read line");
        input = input.trim().to_string();
        write_event("input", &input);
        input
    }

    #[cfg(all(feature = "playback", not(feature = "recording")))]
    pub fn get_user_input() -> String {
        read_event::<String>("input")
    }

    #[cfg(not(any(feature = "recording", feature = "playback")))]
    pub fn get_user_input() -> String {
        let mut input = String::new();
        stdin().read_line(&mut input).expect("failed to read line");
        input.trim().to_string()
    }

    #[cfg(all(feature = "recording", feature = "playback"))]
    pub fn get_user_input() -> String {
        unreachable!(
            "feature \"recording\" and feature \"playback\" cannot be enabled at the same time"
        );
    }

    // console_output

    #[cfg(all(feature = "recording", not(feature = "playback")))]
    pub fn console_output(text: &str) {
        print!("{text}");
        write_event("output", text);
    }

    #[cfg(all(feature = "playback", not(feature = "recording")))]
    pub fn console_output(text: &str) {
        assert_eq!(
            text,
            read_event::<String>("output"),
            "Console output does not match the expected text"
        );
    }
    #[cfg(not(any(feature = "recording", feature = "playback")))]
    pub fn console_output(text: &str) {
        print!("{text}");
    }

    #[cfg(all(feature = "recording", feature = "playback"))]
    pub fn console_output(_text: &str) {
        unreachable!();
    }
}

mod rnp_lib {
    use serde::Serialize;

    use serde::de::DeserializeOwned;

    use std::error::Error;

    use std::fs::File;

    #[cfg(any(feature = "recording", feature = "playback"))]
    use std::sync::Mutex;

    #[cfg(all(feature = "playback", not(feature = "recording")))]
    use std::io::BufRead;

    #[cfg(all(feature = "playback", not(feature = "recording")))]
    use std::io::Read;

    use std::io::BufReader;

    #[cfg(all(feature = "recording", not(feature = "playback")))]
    use std::time::Instant;

    #[cfg(all(feature = "recording", not(feature = "playback")))]
    use std::io::{BufWriter, Write};

    use std::fmt::Debug;

    #[cfg(all(feature = "recording", not(feature = "playback")))]
    lazy_static::lazy_static! {
        static ref START_TIME: Instant = Instant::now();
        static ref LOG_FILE_WRITER: Mutex<BufWriter<File>> = {
            Mutex::new(BufWriter::new(
                File::create("log.txt").expect("Unable to create log file"),
            ))
        };
    }

    #[cfg(feature = "playback")]
    lazy_static::lazy_static! {
        static ref LOG_FILE_READER: Mutex<BufReader<File>> = {
            Mutex::new(BufReader::new(
                File::open("log.txt").expect("Unable to open log file"),
            ))
        };
    }

    #[cfg(all(feature = "recording", not(feature = "playback")))]
    pub fn write_event<T: Debug + Serialize>(interface: &str, data: T) {
        let mut writer = LOG_FILE_WRITER.lock().unwrap();
        writeln!(
            writer,
            "{} {} {}",
            START_TIME.elapsed().as_millis(),
            interface,
            serialize_object(data).unwrap(),
        )
        .unwrap();
        writer.flush().expect("Failed to flush log file");
    }

    #[cfg(all(feature = "playback", not(feature = "recording")))]
    pub fn read_event<T: DeserializeOwned>(expected_interface: &str) -> T {
        // Any event contains: elapsed_time interface data_length ron_serialized_data
        eprint!("{}", expected_interface.chars().next().unwrap());

        let mut reader = LOG_FILE_READER.lock().unwrap();

        // Read the timestamp.
        let mut timestamp_string = vec![];
        reader.read_until(b' ', &mut timestamp_string).unwrap();
        if timestamp_string.is_empty() {
            panic!("No more events to playback.");
        }
        let _timestamp: usize = str::from_utf8(&timestamp_string)
            .unwrap()
            .trim()
            .parse()
            .unwrap();

        // Read the interface.
        let mut interface = vec![];
        reader.read_until(b' ', &mut interface).unwrap();
        let actual_interface = str::from_utf8(&interface).unwrap().trim();
        assert_eq!(
            actual_interface, expected_interface,
            "Read interface does not match the expected interface. Expected: {expected_interface}, got: {actual_interface}",
        );

        // Read the data_length and the data.
        deserialize_object(&mut reader).unwrap()
    }

    #[cfg(all(feature = "recording", not(feature = "playback")))]
    pub fn init_log_file() {
        eprintln!("Running in Recording mode, log file will be created.");
        write_event("---", "Log initialized");
    }

    #[cfg(all(feature = "playback", not(feature = "recording")))]
    pub fn init_log_file() {
        eprintln!("Running in Playback mode, log file will be read.");
        assert_eq!(
            read_event::<String>("---"),
            "Log initialized",
            "Log file is not initialized correctly"
        );
    }

    #[cfg(not(any(feature = "recording", feature = "playback")))]
    pub fn init_log_file() {
        eprintln!(
            "Running in Normal mode, no log file will be created nor read."
        );
    }

    #[cfg(all(feature = "recording", feature = "playback"))]
    pub fn init_log_file() {
        unreachable!();
    }

    #[cfg(all(feature = "recording", not(feature = "playback")))]
    fn serialize_object(
        object: impl Debug + Serialize,
    ) -> Result<String, Box<dyn Error>> {
        let serialized_object = ron::ser::to_string(&object)?;
        let len = serialized_object.len();
        Ok(format!("{len} {serialized_object}"))
    }
    #[cfg(any(not(feature = "recording"), feature = "playback"))]
    #[allow(dead_code)]
    fn serialize_object(
        _object: impl Debug + Serialize,
    ) -> Result<String, Box<dyn Error>> {
        unreachable!();
    }

    #[cfg(all(feature = "playback", not(feature = "recording")))]
    fn deserialize_object<T: DeserializeOwned>(
        reader: &mut BufReader<File>,
    ) -> Result<T, Box<dyn Error>> {
        let mut len_string = vec![];
        reader.read_until(b' ', &mut len_string)?;
        let len_string = String::from_utf8(len_string)?;
        let len: usize = len_string.trim().parse()?;
        let mut object_string = vec![0; len];
        reader.read_exact(&mut object_string)?;
        ron::de::from_bytes(&object_string).map_err(|e| e.into())
    }
    #[cfg(any(not(feature = "playback"), feature = "recording"))]
    #[allow(dead_code)]
    fn deserialize_object<T: DeserializeOwned>(
        _reader: &mut BufReader<File>,
    ) -> Result<T, Box<dyn Error>> {
        unreachable!();
    }
}
