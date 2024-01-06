use std::collections::HashMap;
use std::fmt::format;
use std::fs::{File, read};
use std::io::{BufRead, BufReader};

pub const CONFIG_PATH: &str = "config.cfg";

pub struct Parser {
    file_path: String,
    data: HashMap<String, String>
}

impl Parser {

    pub fn new(path: &str) -> Result<Self, String> {

        let file = File::open(&path)
            .map_err(|err| format!("[FILE.ERROR] Can't open file `{}`\n{}", path, err.to_string()))?;
        let reader = BufReader::new(file);

        let mut data = HashMap::new();

        for line in reader.lines() {
            match line {
                Ok(line) => {
                    let record = Parser::parse_line(&line);
                    if record.is_empty()  {
                        continue;
                    }
                    data.insert(
                        record[0].to_string(),
                        record[1].to_string()
                    );
                }
                Err(err) => {
                    return Err(err.to_string());
                }
            };
        }

        Ok(Parser { file_path: path.to_string(), data })
    }

    fn parse_line(line: &String) -> Vec<String> {
        if line.starts_with("#") || line.len() == 0 {
            return Vec::new();
        }
        let parts = line.split_once("=").expect("[FILE.ERROR] Some config record(s) is not in `key=value` format");

        Vec::from([String::from(parts.0.trim()), String::from(parts.1.trim())])
    }

    pub fn get_value(&self, key: &str) -> Result<&String, String> {
        self.data.get(key).ok_or(format!("[FILE.ERROR] No such record with key `{}`", key))
    }
}
