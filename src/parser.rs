/*
fn parse(params: &Params) -> Result<bool, io::Error> {

    let file = fs::File::open(&params.filename)?;
    println!("{:?}", file);

    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(file);
    
    for result in reader.deserialize() {
        let record: Record = result?;

        println!("{:?}", record);

        let tran = Transaction::new(&record);
        println!("{}", tran.to_string());
    }

    Ok(true)
}

#[derive(Debug, Deserialize)]
struct Record {
    date: String,
    amount: f64,
    subject: String,
    location: String,
    point_of_sale: String,
    type: String,
}


*/