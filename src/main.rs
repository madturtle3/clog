use std::fmt::Display;

#[derive(PartialEq, Debug)]
enum RecordTypes {
    HEADER,
    RECORD,
    COMMAND,
}

impl From<char> for RecordTypes {
    fn from(input:char) -> RecordTypes {
        match input {
            '!' => RecordTypes::HEADER,
            '*' => RecordTypes::RECORD,
            '$' => RecordTypes::COMMAND,
            _ => panic!("Incorrect header type found in log file"),
        }
    }
}
impl Into<char> for RecordTypes {
    fn into(self) -> char {
        match self {
            RecordTypes::HEADER => '!',
            RecordTypes::RECORD => '*',
            RecordTypes::COMMAND => '$',
        }
    }
}
#[derive(Debug)]
struct Record {
    record_type: RecordTypes,
    pub fields: Vec<String>,
}
impl Record {
    fn new(record_type: RecordTypes) -> Record {
        let fields: Vec<String> = Vec::new();
        Record {
            record_type,
            fields,
        }
    }
    fn from_file() -> Log {
        let log_string: String = std::fs::read_to_string("log").expect("Could not read log in");
        let mut log: Log = Log::new();
        for line in log_string.split("\n") {
            let record_type: RecordTypes = RecordTypes::from(line.chars().nth(0).unwrap());
            let mut record: Record = Record::new(record_type);
            for (i,field) in line.split(" ").enumerate() {
                if i > 0 {
                record.fields.push(field.to_string());
                }
            }
            log.push(record);
        }
        log
    }
}
impl Display for Record {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}",self.fields.join("\t"))
    }
}
type Log = Vec<Record>;
fn print_log(log: &Log, record_type: RecordTypes) {
    for record in log {
        println!("{}",record)
    }
}

fn main() {
    let record: Log = Record::from_file();
    print_log(&record, RecordTypes::RECORD);
}
