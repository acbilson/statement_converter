#[derive(Debug)]
pub enum Source {
    File(Params),
    Unknown
}

#[derive(Debug)]
pub struct Params {
    pub path: String,
}

impl Params {

    pub fn new(args: &[String]) -> Source {
        if args.len() < 2 {
            return Source::Unknown
        }

        Source::File(Params { path: args[1].clone() })
    }
}

#[cfg(test)]
mod params_tests {
    use super::*;

    #[test]
    fn handles_missing_args() {
        let args: Vec<String> = vec![String::from("target/debug/statement_parser")];

        let actual = match Params::new(&args) {
            Source::File(_f) => assert!(false),
            Source::Unknown => assert!(true),
        };
    }
}