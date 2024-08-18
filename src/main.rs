use std::path::PathBuf;

use clap::Parser;
use cloglib::{Log, Record};
/* This module is meant to be self contained and usable outside
of this program. items specific to the UI or CLI should be created
outside of this module. */
mod cloglib {
    use std::path::Path;
    #[derive(Debug)]
    pub enum Record {
        HEADER { columns: Vec<String> },
        CONTACT { fields: Vec<String> },
        VARSET { setting: String, value: String },
        COMMENT { comment: String },
    }
    impl From<&str> for Record {
        fn from(line: &str) -> Record {
            let record_indicator = line.chars().nth(0).unwrap();
            let fields: Vec<String> = line.split(" ").map(String::from).collect();
            let fields = fields[1..fields.len()].to_owned();
            match record_indicator {
                '!' => Record::HEADER { columns: fields },
                '*' => Record::CONTACT { fields: fields },
                '$' => {
                    if fields.len() == 2 {
                        Record::VARSET {
                            setting: fields[0].clone(),
                            value: fields[1].clone(),
                        }
                    } else {
                        panic!("Updater record only can take 2 fields, 3 given.");
                    }
                }
                '#' => Record::COMMENT {
                    comment: fields.join(" ").to_string(),
                }, // RESERVED FOR COMMENTS
                _ => panic!("Incorrect record indicator {}", record_indicator),
            }
        }
    }
    impl Into<String> for Record {
        fn into(self: Record) -> String {
            match self {
                Record::HEADER { columns } => "! ".to_owned() + &columns.join(" "),
                Record::VARSET { setting, value } => "* ".to_owned() + &setting + &value,
                Record::CONTACT { fields } => "* ".to_owned() + &fields.join(" "),
                Record::COMMENT { comment } => "# ".to_owned() + &comment,
            }
        }
    }
    pub struct Log {
        pub records: Vec<Record>,
    }

    impl Log {
        fn new() -> Log {
            Log {
                records: Vec::new(),
            }
        }
        pub fn from_file<T: AsRef<Path>>(path: T) -> Log {
            let log_string: String = std::fs::read_to_string(path).expect("Coudl not read file");
            let mut output: Log = Log::new();
            for line in log_string.split("\n") {
                output.records.push(Record::from(line));
            }
            output
        }
    }
}

fn print_log(log: &Log) {
    let mut colmaxes: Vec<usize> = Vec::new();
    for record in &log.records {
        // this is cursed and I really don't care
        let record_max = match record {
            Record::HEADER { columns } => columns.iter().map(String::len).collect(),
            Record::CONTACT { fields } => fields.iter().map(String::len).collect(),
            Record::COMMENT { comment: _ } => Vec::new(),
            Record::VARSET {
                setting: _,
                value: _,
            } => Vec::new(),
        };
        while colmaxes.len() < record_max.len() {
            colmaxes.push(0);
        }
        for x in 0..record_max.len() {
            if record_max[x] > colmaxes[x] {
                colmaxes[x] = record_max[x];
            }
        }
    }
    for record in &log.records {
        let stringvec = match record {
            Record::CONTACT { fields } => Some(fields),
            Record::HEADER { columns } => Some(columns),
            _ => None,
        };

        if let Some(x) = stringvec {
            for (index, field) in x.iter().enumerate() {
                let padding = colmaxes[index];
                print!(" {:padding$} │", field);
            }
            if x.len() != 0 {
                print!("\n");
            }
        }
    }
}

#[derive(clap::Subcommand)]
enum Commands {
    ///print out the current log
    List,
}

#[derive(clap::Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
    ///file to use for the log
    #[arg(short = 'o', long = "file", default_value = "hamlog")]
    file: PathBuf,
}
fn main() {
    let args = Args::parse();
    let log = Log::from_file(args.file);
    match args.command {
        Commands::List => {
            print_log(&log);
        }
    }
}
