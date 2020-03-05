use super::parser;

pub fn record_to_trans(record: &parser::Record) -> Transaction {
    Transaction {
        date: record.date.to_string(),
        status: '*',
        subject: record.subject.to_string(),
        postings: record_to_postings(&record),
    }
}

fn record_to_postings(record: &parser::Record) -> Vec<Posting> {
    vec![
    Posting {
        account: "Expenses:Unknown".to_string(),
        amount: record.amount,
    },]
}

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

pub struct Posting {
    pub account: String,
    pub amount: f64, // TODO: create currency type? $10.43
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
        let record = parser::Record {
            date: "2019/07/16".to_string(), 
            amount: 7.72, 
            subject: "5059 Debit Card Purchase Dollarshaveclubus".to_string(),
            location: String::new(),
            point_of_sale: "7790345006".to_string(),
            debit: true,
        };

        let actual = record_to_trans(&record);

        assert_eq!(record.date, actual.date);
        assert_eq!(record.amount, actual.postings[0].amount);
        assert_eq!(record.date, actual.date);
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
                    amount: 7.72, 
                },
            ]
        };

        let actual = trans.to_strings();

        assert!(actual.len() == 2);
        assert_eq!("07/16 * 5059 Debit Card Purchase Dollarshaveclubus\n".to_string(), actual[0]);
        assert_eq!("  Expenses:Unknown  7.72\n".to_string(), actual[1]);
    }

}