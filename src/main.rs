/*

INPUT:

2019/07/16,7.72,"5059 Debit Card Purchase Dollarshaveclubus",,"7790345006","DEBIT"

OUTPUT:

10/18 * Brothers K
  Expenses:Food:Coffee                                           $3.50

*/
use std::{env, fs, path, process};

use std::io::{self, BufRead, BufReader, BufWriter, Write};

// external crates
use atty::Stream;
use env_logger::{self, Env};
use log::{debug, error, info, warn};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "cli-args")]
struct CliArgs {
    input: Option<String>,
    output: Option<String>,

    #[structopt(short = "w", long = "journal_width", default_value = "70")]
    journal_width: u32,
}

fn enable_logging() {
    env_logger::from_env(Env::default().default_filter_or("warn")).init();
}

fn main() {
    enable_logging();

    let conf = get_config();
    debug!("{:?}", conf);

    let args = CliArgs::from_args();
    debug!("{:?}", args);

    match convert_to_journal(&args, &conf) {
        Ok(report) => info!("{}", report.to_string()),
        Err(e) => {
            error!("{:?}", e);
            process::exit(1)
        }
    };
}

#[derive(Debug)]
struct ConvertConfig {
    nth_to_log: u32,
}

/// returns a configuration struct with optional values from environment variables
fn get_config() -> ConvertConfig {
    let default_nth_to_log: u32 = 25;

    let nth_to_log = match env::var("NTH_TO_LOG") {
        Ok(w) => match w.parse::<u32>() {
            Ok(i) => i,
            Err(e) => {
                warn!("{:?}", e);
                default_nth_to_log
            }
        },
        Err(_e) => {
            info!("Optional environment variable NTH_TO_LOG is not set. Defaults to logging every 25th record.");
            default_nth_to_log
        }
    };

    ConvertConfig { nth_to_log }
}

/// Converts a bank statement into a ledger-cli compliant journal file
///  ## Steps
///  1. get file reader & writer
///  2. deserialize to record
///  3. convert to transaction
///  4. write as string
///  5. return report
fn convert_to_journal(args: &CliArgs, conf: &ConvertConfig) -> io::Result<ConvertReport> {
    let reader = get_reader(&args.input)?;
    let mut writer = get_writer(&args.output)?;

    let mut count: u32 = 0;

    // reads, transforms, and writes
    for line in reader.lines() {
        let line = line?;
        log_nth(&line, count, conf.nth_to_log);

        let record = parse(&line);
        let trans = record_to_trans(&record);
        for line in trans.to_strings(&args.journal_width) {
            writer.write(&line.into_bytes())?;
        }
        writer.flush()?;
        count += 1;

        log_nth(trans, count, conf.nth_to_log);
    }

    Ok(ConvertReport { total: count })
}

fn log_nth(val: impl std::fmt::Debug, count: u32, n: u32) {
    if count % n == 0 {
        debug!("{:?}", val);
    }
}

struct ConvertReport {
    total: u32,
}

impl ConvertReport {
    fn to_string(&self) -> String {
        format!("Total transactions recorded: {}", &self.total)
    }
}

fn has_term_input() -> bool {
    atty::isnt(Stream::Stdin)
}

/// returns a buffered reader from an input file, if present, or from stdin
fn get_reader(path: &Option<String>) -> io::Result<Box<dyn BufRead>> {
    return match &path {
        None => {
            if has_term_input() {
                info!("input streaming from stdin.");
                Ok(Box::new(BufReader::new(io::stdin())))
            } else {
                Err(io::Error::new(
                    io::ErrorKind::BrokenPipe,
                    "stdin is unavailable.",
                ))
            }
        }
        Some(path) => match open_statement(path) {
            Err(e) => Err(e),
            Ok(reader) => Ok(Box::new(reader)),
        },
    };
}

/// returns a buffered writer to an input file, if present, or to stdout
fn get_writer(path: &Option<String>) -> io::Result<Box<dyn Write>> {
    return match &path {
        None => {
            info!("output streaming to stdout.");
            Ok(Box::new(BufWriter::new(io::stdout())))
        }
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
        amount: parse_amount(values[1]),
        subject: rm_quotes(values[2]),
        location: rm_quotes(values[3]),
        point_of_sale: rm_quotes(values[4]),
        debit: rm_quotes(values[5]) == "DEBIT",
    }
}

fn parse_amount(value: &str) -> f64 {
    value.parse::<f64>().unwrap()
}

fn parse_date(value: &str) -> String {
    value[5..].to_string()
}

fn rm_quotes(value: &str) -> String {
    value.trim_matches(|c| c == '\\' || c == '"').to_string()
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
    vec![Posting {
        account: "Expenses:Unknown".to_string(),
        amount: amount_to_string(&record.amount),
    }]
}

fn amount_to_string(amount: &f64) -> String {
    let amount = format!("${}", amount);

    if !amount.contains('.') {
        return format!("{}.00", amount).to_string();
    } else if decimal_count(&amount) < (2 as usize) {
        return format!("{}0", amount).to_string();
    }
    amount
}

fn decimal_count(val: &str) -> usize {
    val.get(val.find('.').unwrap()..).unwrap().len() - 1
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
    fn to_strings(&self, width: &u32) -> Vec<String> {
        let mut first_line = String::with_capacity(100);
        first_line.push_str(&self.date);
        first_line.push_str(&format!(" {} ", &self.status));
        first_line.push_str(&self.subject);
        first_line.push('\n');

        let mut second_line = String::with_capacity(100);
        for post in &self.postings {
            second_line.push_str(&post.to_string(&width));
        }
        second_line.push('\n');

        vec![first_line, second_line]
    }
}

#[derive(Debug)]
struct Posting {
    account: String,
    amount: String,
}

impl Posting {
    fn to_string(&self, width: &u32) -> String {
        let mut result = String::with_capacity(100);
        result.push_str("  ");
        result.push_str(&self.account);
        result.push_str(&self.width_to_string(&width));
        result.push_str(&self.amount.to_string());
        result.push('\n');
        result
    }

    fn width_to_string(&self, width: &u32) -> String {
        let size = *width as usize - &self.account.len() - &self.amount.len();
        vec![' '; size].into_iter().collect()
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
}
