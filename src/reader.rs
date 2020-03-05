#[cfg(test)]
mod reader_tests {
    use std::io::prelude::*;
    use std::{ fs, io, env };

    fn get_wd() -> String {
        if let Ok(var) = env::var("TEST_DIR") {
            var
        } else {
            panic!("Please create a TEST_DIR env variable to continue.")
        }
    }

    #[test]
    fn reads_line() -> io::Result<()> {

        let header = "date,amount,subject,location,point_of_sale,type\n";

        let path = format!("{}/data/single.csv", get_wd());
        let f = fs::File::open(&path)?;
        let mut reader = io::BufReader::new(f);
        let mut buffer = String::new();

        // read a line into buffer
        //let len = reader.read_line(&mut buffer)?;
        for line in reader.lines() {
            //assert!(&line?.len() > 0);
            println!("{}", &line?);
        }
        Ok(())
    }

    /*
    #[test]
    fn reads_line() {
        let path = format!("{}/data/single.csv", get_wd());
        let f = fs::File::open(&path).expect("should open file");
        let mut reader = io::BufReader::new(f);

        let mut buf = String::new();
        let len = reader.read_line(&mut buf).expect("should read first line");
        assert!(len > 0);
        println!(buf);
    }
    */

}