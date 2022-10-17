use std::io::BufRead;
use crate::models::Log;

pub struct Logs {
    records: Vec<Log>,
}

const FILE_ERROR: &str = "Failed to open the file!";
const READ_ERROR: &str = "Failed to read the file!";

impl Logs {
    pub fn load_file(path: &str) -> Result<Logs, String> {
        let mut records = Vec::new();
        let file = std::fs::File::open(path).expect(FILE_ERROR);
        let reader = std::io::BufReader::new(file);
        for line in reader.lines() {
            let line = line.expect(READ_ERROR);
            let ip = Self::get_ip(&line);
            let timestamp = Self::get_timestamp(&line);
            let method = Self::get_method(&line);
            let path = Self::get_path(&line);
            let status = Self::get_status(&line);
            let referer = Self::get_referer(&line);
            let user_agent = Self::get_user_agent(&line);
            records.push(Log::new(
                ip?,
                timestamp?,
                method?,
                path?,
                status?,
                referer?,
                user_agent?,
            ));
        }
        Ok(Logs {
            records
        })
    }

    pub fn records(&self) -> &Vec<Log> {
        &self.records
    }

    fn get_ip(line: &str) -> Result<String, &str> {
        let parts = line.split(" - - ").collect::<Vec<&str>>();
        if parts.len() > 0 {
            return Ok(String::from(parts[0]));
        }
        Err(FILE_ERROR)
    }

    fn get_timestamp(line: &str) -> Result<String, &str> {
        let parts = line.split("[").collect::<Vec<&str>>();
        if parts.len() > 1 {
            let parts = parts[1].split("]").collect::<Vec<&str>>();
            if parts.len() > 0 {
                return Ok(String::from(parts[0]));
            }
        }
        Err(FILE_ERROR)
    }

    fn get_method(line: &str) -> Result<String, &str> {
        let parts = line.split("\"").collect::<Vec<&str>>();
        if parts.len() > 1 {
            let parts = parts[1].split(" ").collect::<Vec<&str>>();
            if parts.len() > 0 {
                return Ok(String::from(parts[0]));
            }
        }
        Err(FILE_ERROR)
    }

    fn get_path(line: &str) -> Result<String, &str> {
        let parts = line.split(" /").collect::<Vec<&str>>();
        if parts.len() > 1 {
            let parts = parts[1].split(" ").collect::<Vec<&str>>();
            if parts.len() > 0 {
                let mut result = String::from("/");
                result.push_str(parts[0]);
                return Ok(result);
            }
        }
        Err(FILE_ERROR)
    }

    fn get_status(line: &str) -> Result<String, &str> {
        let parts = line.split("\" ").collect::<Vec<&str>>();
        if parts.len() > 1 {
            let parts = parts[1].split(" ").collect::<Vec<&str>>();
            if parts.len() > 0 {
                return Ok(String::from(parts[0]));
            }
        }
        Err(FILE_ERROR)
    }

    fn get_referer(line: &str) -> Result<String, &str> {
        let parts = line.split(" \"").collect::<Vec<&str>>();
        if parts.len() > 1 {
            let parts = parts[2].split("\"").collect::<Vec<&str>>();
            if parts.len() > 0 {
                return Ok(String::from(parts[0]));
            }
        }
        Err(FILE_ERROR)
    }

    fn get_user_agent(line: &str) -> Result<String, &str> {
        let parts = line.split("\" \"").collect::<Vec<&str>>();
        if parts.len() > 1 {
            let parts = parts[1].split("\"").collect::<Vec<&str>>();
            if parts.len() > 0 {
                return Ok(String::from(parts[0]));
            }
        }
        Err(FILE_ERROR)
    }
}