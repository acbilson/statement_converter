/*

INPUT:

2019/07/16,7.72,"5059 Debit Card Purchase Dollarshaveclubus",,"7790345006","DEBIT"

OUTPUT:

10/18 * Brothers K
  Expenses:Food:Coffee                                           $3.50

*/
use std::{env, process};

use statement_parser as sp;

fn main() {
    let args: Vec<String> = env::args().collect();

    let _params = match sp::Params::new(&args) {
        sp::Source::File(p) => p,
        sp::Source::Unknown => {
            println!("Please supply the name of the file to this command.");
            process::exit(1)
        },
    };

    /*
    match sp::convert_to_journal(&params) {
        Ok(_val) => _val,
        Err(e) => {
            println!("{:?}", e);
            process::exit(1)
        }
    };
    */
}