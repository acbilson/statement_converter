/*

INPUT:

2019/07/16,7.72,"5059 Debit Card Purchase Dollarshaveclubus",,"7790345006","DEBIT"

OUTPUT:

10/18 * Brothers K
  Expenses:Food:Coffee                                           $3.50

*/
use std::{ 
    process,
    path, 
    fs, 
};

use std::io::{
    self,
    BufRead,
    BufReader,
    BufWriter,
    Write,
};

// external crates
use env_logger;
use log::{info, error};
use structopt::StructOpt;
use atty::Stream;

#[derive(StructOpt, Debug)]
#[structopt(name = "cli-args")]
struct CliArgs {

    input: Option<String>,
    output: Option<String>,
}

fn main() {
    env_logger::init();
    info!("starting up!");

    let args = CliArgs::from_args();
    info!("{:?}", args);

    match convert_to_journal(&args) {
        Ok(report) => info!("{}", report.to_string()),
        Err(e) => {
            error!("{:?}", e);
            process::exit(1)
        }
    };
}

/*
*   1. get file reader & writer
*   2. deserialize to record
*   3. convert to transaction
*   4. write as string
*   5. return report
*/
fn convert_to_journal(args: &CliArgs) -> io::Result<ConvertReport> {

    let reader = get_reader(&args.input)?;
    let mut writer = get_writer(&args.output)?;

    let mut count = 0;

    // reads, transforms, and writes
    for line in reader.lines() {
        let record = parse(&line?);
        let trans = record_to_trans(&record);
        for line in trans.to_strings() {
            writer.write(&line.into_bytes())?;
        }
        writer.flush()?;
        count += 1;

        // logs every twenty-fifth trans
        if count % 25 == 0 {
            info!("{:?}", trans);
        }
    }

    Ok(ConvertReport { total: count } )
}

struct ConvertReport {
    total: i32,
}

impl ConvertReport {
    fn to_string(&self) -> String {
        format!("Total transactions recorded: {}", &self.total)
    }
}

fn has_term_input() -> bool { atty::isnt(Stream::Stdin) }

fn get_reader(path: &Option<String>) -> io::Result<Box<dyn BufRead>> {

    return match &path {
        None => { 
            if has_term_input() {
                Ok(Box::new(BufReader::new(io::stdin())))
            } else {
                Err(io::Error::new(io::ErrorKind::BrokenPipe, "stdin is unavailable."))
            }
        },
        Some(path) => match open_statement(path) {
            Err(e) => Err(e),
            Ok(reader) => Ok(Box::new(reader)),
        },
    };
}

fn get_writer(path: &Option<String>) -> io::Result<Box<dyn Write>> {

    return match &path {
        None => { 
            Ok(Box::new(BufWriter::new(io::stdout())))
        },
        Some(path) => match create_journal(path) {
            Err(e) => Err(e),
            Ok(writer) => Ok(Box::new(writer)),
        },
    };
}

fn create_journal(path: &str) -> io::Result<fs::File> {

    let out_path = path::Path::new(&path).to_str().unwrap();
    let writer = fs::File::create(out_path)?;
    info!("journal created at {}", &out_path);
    Ok(writer)
}

fn open_statement(path: &str) -> io::Result<io::BufReader<fs::File>> {
    let in_path = path::Path::new(&path).to_str().unwrap();
    let in_file = fs::File::open(in_path)?;
    let reader = io::BufReader::new(in_file);
    info!("statement opened at {}", &in_path);
    Ok(reader)
}

fn parse(line: &str) -> Record {

    let values: Vec<&str> = line.split(',').collect();

    Record {
        date: parse_date(values[0]),
        amount: values[1].parse::<f64>().unwrap(),
        subject: rm_quotes(values[2]),
        location: rm_quotes(values[3]),
        point_of_sale: rm_quotes(values[4]),
        debit: rm_quotes(values[5]) == "DEBIT",
    }
}

fn parse_date(value: &str) -> String {
    value[5..].to_string()
}

fn rm_quotes(value: &str) -> String {
    value.trim_matches(
        |c| c == '\\' || c == '"'
    ).to_string()
}

#[derive(Debug)]
struct Record {
    date: String,
    amount: f64,
    subject: String,
    location: String,
    point_of_sale: String,
    debit: bool,
}

#[cfg(test)]
mod parser_tests {
    use super::*;

    #[test]
    fn parses_line() {
        let line = "2019/07/16,7.72,\"5059 Debit Card Purchase Dollarshaveclubus\",,\"7790345006\",\"DEBIT\"";

        let record = parse(&line);

        assert_eq!("07/16".to_string(), record.date);
        assert_eq!(7.72, record.amount);
        assert_eq!("5059 Debit Card Purchase Dollarshaveclubus", record.subject);
        assert_eq!("", record.location);
        assert_eq!("7790345006", record.point_of_sale);
        assert_eq!(true, record.debit);
    }
}

fn record_to_trans(record: &Record) -> Transaction {
    Transaction {
        date: record.date.to_string(),
        status: '*',
        subject: record.subject.to_string(),
        postings: record_to_postings(&record),
    }
}

fn record_to_postings(record: &Record) -> Vec<Posting> {
    vec![
    Posting {
        account: "Expenses:Unknown".to_string(),
        amount: format!("${}",record.amount),
    },]
}

#[derive(Debug)]
struct Transaction {
    date: String,
    status: char,
    subject: String,
    postings: Vec<Posting>,
}

impl Transaction {

    // TODO: complete fn
    fn to_strings(&self) -> Vec<String> {
        let mut first_line = String::with_capacity(100);
        first_line.push_str(&self.date);
        first_line.push_str(&format!(" {} ", &self.status));
        first_line.push_str(&self.subject);
        first_line.push('\n');

        let mut second_line = String::with_capacity(100);
        for post in &self.postings {
            second_line.push_str(&post.to_string());
        }

        vec![first_line, second_line]
    }
}

#[derive(Debug)]
struct Posting {
    account: String,
    amount: String,
}

impl Posting {

    // TODO: complete fn
    fn to_string(&self) -> String {
        let mut result = String::with_capacity(100);
        result.push_str("  ");
        result.push_str(&self.account);
        result.push_str("  ");
        result.push_str(&self.amount.to_string());
        result.push('\n');
        result
    }
}

#[cfg(test)]
mod trans_tests {
    use super::*;

    #[test]
    fn converts_record() {
        let record = Record {
            date: "2019/07/16".to_string(), 
            amount: 7.72, 
            subject: "5059 Debit Card Purchase Dollarshaveclubus".to_string(),
            location: String::new(),
            point_of_sale: "7790345006".to_string(),
            debit: true,
        };

        let actual = record_to_trans(&record);

        assert_eq!(record.date, actual.date);
        assert_eq!("$7.72", actual.postings[0].amount);
        assert_eq!(record.subject, actual.subject);
    }

    #[test]
    fn prints_trans() {
        let trans = Transaction {
            date: "07/16".to_string(), 
            status: '*',
            subject: "5059 Debit Card Purchase Dollarshaveclubus".to_string(),
            postings: vec![
                Posting {
                    account: "Expenses:Unknown".to_string(),
                    amount: "$7.72".to_string(), 
                },
            ]
        };

        let actual = trans.to_strings();

        assert!(actual.len() == 2);
        assert_eq!("07/16 * 5059 Debit Card Purchase Dollarshaveclubus\n".to_string(), actual[0]);
        assert_eq!("  Expenses:Unknown  $7.72\n".to_string(), actual[1]);
    }

}