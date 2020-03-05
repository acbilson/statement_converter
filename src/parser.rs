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