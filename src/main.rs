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
    io, 
    io::prelude::*
};

// external crates
use env_logger;
use log::{info, warn, error};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "cli-args")]
struct CliArgs {

    input: String,

    #[structopt(long = "output", short = "o", default_value = "/dev/null")]
    output: String,
}

impl CliArgs {
    // TODO: could get .metadata and post reason why args are not valid
    fn valid(&self) -> bool {
        path::Path::new(&self.input).exists()
    }
}

fn main() {
    env_logger::init();
    info!("starting up!");

    let args = CliArgs::from_args();
    info!("{:?}", args);

    if !args.valid() {
        warn!("cli args were not valid.");
        process::exit(1);
    }

    match convert_to_journal(&args) {
        Ok(count) => info!("total transactions recorded: {}", count),
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
fn convert_to_journal(args: &CliArgs) -> io::Result<i32> {

    // Creates journal for writing
    let out_path = path::Path::new(&args.output).to_str().unwrap();
    let mut out_file = fs::File::create(out_path)?;
    info!("journal created at {}", &out_path);

    // Opens statement for reading
    let in_path = path::Path::new(&args.input).to_str().unwrap();
    let f = fs::File::open(in_path)?;
    info!("statement opened at {}", &in_path);

    let reader = io::BufReader::new(f);

    let mut count = 0;

    // reads, transforms, and writes
    for line in reader.lines() {
        let record = parse(&line?);
        let trans = record_to_trans(&record);
        for line in trans.to_strings() {
            out_file.write(&line.into_bytes())?;
        }
        out_file.flush()?;
        count += 1;

        // logs every tenth trans
        if count % 10 == 0 {
            info!("{:?}", trans);
        }
    }

    Ok(count)
}

pub fn parse(line: &str) -> Record {

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
pub struct Record {
    pub date: String,
    pub amount: f64,
    pub subject: String,
    pub location: String,
    pub point_of_sale: String,
    pub debit: bool,
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

pub fn record_to_trans(record: &Record) -> Transaction {
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
pub struct Transaction {
    pub date: String,
    pub status: char,
    pub subject: String,
    pub postings: Vec<Posting>,
}

impl Transaction {

    // TODO: complete fn
    pub fn to_strings(&self) -> Vec<String> {
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
pub struct Posting {
    pub account: String,
    pub amount: String,
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