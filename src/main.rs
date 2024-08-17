use cloglib::{Log, Record};


/* This module is meant to be self contained and usable outside
of this program. items specific to the UI or CLI should be created
outside of this module. */
mod cloglib {
    use std::fs;

    const LOGFILE: &str = "log";
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
        pub fn from_file() -> Log {
            let log_string: String = fs::read_to_string(LOGFILE).expect("Coudl not read file");
            let mut output: Log = Log::new();
            for line in log_string.split("\n") {
                output.records.push(Record::from(line));
            }
            output
        }
    }
}

fn print_log(log: &Log) { // integrate this all fancy like LATER
    for record in &log.records {
        match record {
            Record::CONTACT { fields } => {
                println!("{}",fields.join("\t"));
            }
            Record::HEADER { columns } => {
                println!("{}",columns.join("\t"));
            }
            _ => ()
        }
    }
}
fn main() {
    let log = cloglib::Log::from_file();
    print_log(&log);
}
