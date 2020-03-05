/*
struct Transaction {
    date: String,
    status: char,
    subject: String,
    postings: Vec<Posting>,
}

impl Transaction {

    /* TODO: 
    /    - convert date to proper format
    /    - decide how to handle status
    /    - parse subject from trans subject
    /    - create postings by account & amount
    */
    fn new(record: &Record) -> Transaction { 
        Transaction {
            date: record.0.clone(),
            status: '*',
            subject: record.2.clone(),
            postings: Vec::new()
        }
    }

    // TODO: complete fn
    fn to_string(&self) -> String {
        let mut result = String::with_capacity(6);
        result.push_str(&self.date);
        result.push_str(format!(" {} ", &self.status));
        result.push_str(&self.subject);
        result
    }
}

struct Posting {
    account: String,
    amount: f64, // TODO: create currency type? $10.43
}

impl Posting {

    // TODO: complete fn
    // fn new(line: &str) -> Transaction { }

    // TODO: complete fn
    fn to_string(&self) -> String {
        String::from("dummy value")
    }
}

/*
#[cfg(test)]
mod trans_tests {
    use super::*;

    #[test]
    fn converts_record() {
        let record : Record = (
            String::from("2019/07/16"), 
            7.72, 
            String::from("5059 Debit Card Purchase Dollarshaveclubus"),
            String::new(),
            String::from("7790345006"),
            String::from("DEBIT"));

        let actual = convert(&record);

        assert!(actual.is_some())
    }
}
*/
*/