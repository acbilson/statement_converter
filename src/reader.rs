use std::{ fs, io };

pub fn get_reader(path: &String) -> Result<io::BufReader<fs::File>, io::Error> {
    let f = fs::File::open(&path)?;
    Ok(io::BufReader::new(f))

    /*
    let mut line = String::new();
    let len = reader.read_line(&mut line)?;
    println!("First line is {} bytes long", len);
    println!("{}", &line);
    */
}

#[cfg(test)]
mod reader_tests {
    use super::*;
    const WD: &str = "/home/abilson/source/statement_converter";

    #[test]
    fn gets_reader() {
        let path = format!("{}/data/single.csv", WD);
        println!("{}", path);

        match get_reader(&path) {
            Ok(_r) => assert!(true),
            Err(e) => {
                println!("{:?}", e);
                assert!(false)
            }, 
        };
    }
}