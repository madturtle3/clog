use std::{io, path::PathBuf};
use clap::{Parser,CommandFactory};
use clap_complete::{aot::Bash, generate};
use cloglib::{Log, Record};
/* This module is meant to be self contained and usable outside
of this program. items specific to the UI or CLI should be created
outside of this module. */
mod cloglib {
    use std::{io::Write, path::Path};
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
    impl ToString for Record {
        fn to_string(&self) -> String {
            match self {
                Record::HEADER { columns } => "! ".to_owned() + &columns.join(" "),
                Record::VARSET { setting, value } => "* ".to_owned() + &setting + &value,
                Record::CONTACT { fields } => "* ".to_owned() + &fields.join(" "),
                Record::COMMENT { comment } => "# ".to_owned() + &comment,
            }
        }
    }
    impl Record {
        pub fn to_char(&self) -> char {
            match self {
                Self::COMMENT { comment: _ } => '#',
                Self::CONTACT { fields: _ } => '*',
                Self::HEADER { columns: _ } => '!',
                Self::VARSET {
                    setting: _,
                    value: _,
                } => '$',
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
        pub fn wrte_log<T: AsRef<Path>>(log: Log,path: T) {
            let mut fileobj = std::fs::File::create(path).expect("Could not open file for writing");
            for record in log.records {
                writeln!(fileobj,"{}\n",record.to_string()).expect("could not write to file!");
            }
        }
    }
}

fn print_log(log: &Log, print_types: &str) {
    let mut colmaxes: Vec<usize> = Vec::new();
    for record in &log.records {
        // so don't have strange margins for non-printed records
        if print_types.contains(record.to_char()) {
            let record_max = match record {
                Record::HEADER { columns } => columns.iter().map(String::len).collect(),
                Record::CONTACT { fields } => fields.iter().map(String::len).collect(),
                Record::COMMENT { comment } => [comment.len()].to_vec(),
                Record::VARSET { setting, value } => [setting.len(), value.len()].to_vec(),
            };
            // update length of colmaxes in case
            // some records have more fields than others
            while colmaxes.len() < record_max.len() {
                colmaxes.push(0);
            }
            // update maximums
            for x in 0..record_max.len() {
                if record_max[x] > colmaxes[x] {
                    colmaxes[x] = record_max[x];
                }
            }
        }
    }
    // actually print out the log here
    for record in &log.records {
        if print_types.contains(record.to_char()) {
            let stringvec = match record {
                Record::CONTACT { fields } => fields,
                Record::HEADER { columns } => columns,
                Record::COMMENT { comment } => &[comment.to_string()].to_vec(),
                Record::VARSET { setting, value } => {
                    &[setting.to_string(), value.to_string()].to_vec()
                }
            };
            for (index, field) in stringvec.iter().enumerate() {
                let padding = colmaxes[index];
                print!(" {:padding$} │", field);
            }
            if stringvec.len() != 0 {
                print!("\n");
            }
        }
    }
}

#[derive(clap::Subcommand)]
enum Commands {
    ///print out the current log
    List,
    /// create a bash completion script
    SetupComplete,
}

#[derive(clap::Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
    ///file to use for the log
    #[arg(short = 'f', long = "file", default_value = "hamlog")]
    file: PathBuf,
}
fn main() {
    let args = Args::parse();
    let log = Log::from_file(args.file);
    match args.command {
        Commands::List => {
            print_log(&log, "!*");
        }
        Commands::SetupComplete => {
            generate(Bash, &mut Args::command(), clap::crate_name!(), &mut io::stdout())
        }
    }
}
