/*
pub fn convert_to_journal(params: &Params) -> Result<bool, Box<dyn Error>> {
    /*
    *   1. get file reader & writer
    *   2. deserialize to record
    *   3. convert to transaction
    *   4. write as string
    *   5. return report
    */
    Ok(true)
}
*/
use std::{ 
    path, 
    fs, 
    io, 
    io::prelude::*
};
use super::parser;
use super::transformer;

pub fn convert_to_journal() -> io::Result<()> {

    // Creates journal for writing
    let out_path = path::Path::new("/home/abilson/source/statement_converter/out/journal.txt").to_str().unwrap();
    let mut out_file = fs::File::create(out_path)?;
    //let writer = io::BufWriter::new(out_file);

    // Opens statement for reading
    let in_path = path::Path::new("/home/abilson/source/statement_converter/data/full_no_header.csv").to_str().unwrap();
    let f = fs::File::open(in_path)?;
    let reader = io::BufReader::new(f);

    // reads, transforms, and writes
    for line in reader.lines() {
        let record = parser::parse(&line?);
        let trans = transformer::record_to_trans(&record);
        for line in trans.to_strings() {
            out_file.write(&line.into_bytes())?;
        }
        out_file.flush()?;
    }

    Ok(())
}