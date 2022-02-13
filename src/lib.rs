use clap::{App, Arg};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::io::{BufReader, BufWriter, Read, Write};
use std::time::SystemTime;
use std::{error::Error, fs::OpenOptions};

#[derive(Debug)]
pub enum Config {
    All,
    Debug,
    Pop,
    Top { count: u64 },
    Add { task: String },
}

pub fn get_args() -> Result<Config, Box<dyn Error>> {
    let matches = App::new("mindstack")
        .version("0.1.0")
        .author("Minhajuddin Khaja <minhajuddin.k@gmail.com>")
        .about("A simple CLI tool to maintain your mind's stack as you work through tasks in your day.")
        .subcommand(
            App::new("add")
                .about("Add a task to your mind's stack")
                .arg(
                    Arg::new("task")
                        .help("The task to add")
                        .min_values(1)
                   ))
        .subcommand(
            App::new("top")
                .about("List the top tasks on your mind's stack")
                .arg(
                    Arg::new("count")
                        .help("The number of tasks to list")
                        .short('c')
                        .long("count")
                        .default_value("2")
                   ))
        .subcommand(
            App::new("all")
                .about("List the all the tasks on your mind's stack")
                )
        .subcommand(
            App::new("pop")
                .about("Pops the topmost task from your mind's stack")
                )
        .subcommand(
            App::new("debug")
                .about("Shows the full log")
                )
        .get_matches();

    match matches.subcommand() {
        Some(("add", sub_matches)) => {
            let task = sub_matches
                .values_of("task")
                .unwrap()
                .collect::<Vec<&str>>()
                .join(" ");
            Ok(Config::Add { task })
        }
        Some(("top", sub_matches)) => {
            let count = sub_matches.value_of("count").unwrap().to_string().parse()?;
            Ok(Config::Top { count })
        }
        Some(("all", _sub_matches)) => Ok(Config::All),
        Some(("pop", _sub_matches)) => Ok(Config::Pop),
        Some(("debug", _sub_matches)) => Ok(Config::Debug),
        _ => Ok(Config::All),
    }
}

const STATE_FILE: &str = "/home/minhajuddin/mindstack.log";

#[derive(Debug, PartialEq, Eq, Hash)]
struct Item {
    timestamp: u64,
    data: String,
}

#[derive(Serialize, Deserialize, Debug)]
enum LogCommand {
    Add,
    Pop,
}

#[derive(Serialize, Deserialize, Debug)]
struct Log {
    timestamp: u64,
    command: LogCommand,
    data: String,
}

impl Log {
    fn new(timestamp: Option<u64>, command: LogCommand, data: String) -> Log {
        let timestamp = match timestamp {
            Some(t) => t,
            None => SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        };

        Log {
            timestamp,
            command,
            data,
        }
    }

    // serializes data and writes to a structured log using bincode for serialization
    // the format for serialization is as follows:
    // [4-byte-length-of-log][log][magic-number]
    fn save(log: Log) -> Result<(), Box<dyn Error>> {
        let mut log_writer = BufWriter::new(
            OpenOptions::new()
                .create(true)
                .append(true)
                .open(STATE_FILE)?,
        );

        let encoded: Vec<u8> = bincode::serialize(&log)?;
        let encoded_len = (encoded.len() as u32).to_be_bytes();

        log_writer.write_all(&encoded_len)?;
        log_writer.write_all(&encoded)?;

        Ok(())
    }

    fn load_logs() -> Result<Vec<Log>, Box<dyn Error>> {
        let mut log_reader = BufReader::new(OpenOptions::new().read(true).open(STATE_FILE)?);
        let mut logs: Vec<Log> = Vec::new();
        let mut log_len = [0_u8; 4];
        loop {
            // if we can't read 4 bytes, we're done
            if let Err(_) = log_reader.read_exact(&mut log_len) {
                return Ok(logs);
            }
            let log_len = u32::from_be_bytes(log_len);
            let mut log_data = vec![0_u8; log_len as usize];
            log_reader.read_exact(&mut log_data)?;
            let log: Log = bincode::deserialize(&log_data)?;
            logs.push(log);
        }
    }

    fn load() -> Result<Vec<Item>, Box<dyn Error>> {
        let mut log_reader = BufReader::new(OpenOptions::new().read(true).open(STATE_FILE)?);
        let mut items: Vec<Item> = Vec::new();
        let mut items_to_pop: HashSet<Item> = HashSet::new();
        let mut log_len = [0_u8; 4];
        loop {
            // if we can't read 4 bytes, we're done
            if let Err(_) = log_reader.read_exact(&mut log_len) {
                break;
            }
            let log_len = u32::from_be_bytes(log_len);
            let mut log_data = vec![0_u8; log_len as usize];
            log_reader.read_exact(&mut log_data)?;
            let log: Log = bincode::deserialize(&log_data)?;
            match log.command {
                LogCommand::Add => {
                    items.push(Item {
                        timestamp: log.timestamp,
                        data: log.data.clone(),
                    });
                }
                LogCommand::Pop => {
                    items_to_pop.insert(Item {
                        timestamp: log.timestamp,
                        data: log.data.clone(),
                    });
                }
            }
        }

        let mut final_items: Vec<Item> = Vec::new();

        for item in items {
            if !items_to_pop.contains(&item) {
                final_items.push(item);
            }
        }

        final_items.reverse();
        return Ok(final_items);
    }
}

fn print_item(item: Item) {
    println!("{}: {}", item.timestamp, item.data);
}
fn print_all_items(items: Vec<Item>) {
    for item in items {
        print_item(item);
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    match config {
        Config::Add { task } => {
            Log::save(Log::new(None, LogCommand::Add, task))?;
        }
        Config::Debug => {
            let logs = Log::load_logs()?;
            for log in logs {
                println!("{:?}", log);
            }
        }
        Config::Top { count } => {
            let items = Log::load()?
                .into_iter()
                .take(count as usize)
                .collect::<Vec<Item>>();
            print_all_items(items);
        }
        Config::All => {
            let items = Log::load()?;
            print_all_items(items);
        }
        Config::Pop => {
            let items = Log::load()?;
            match items.into_iter().nth(0) {
                Some(item) => {
                    Log::save(Log::new(
                        Some(item.timestamp),
                        LogCommand::Pop,
                        item.data.clone(),
                    ))?;
                    print_item(item);
                }
                None => println!("No items to pop"),
            }
        }
    }
    Ok(())
}
