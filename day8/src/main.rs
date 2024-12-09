use std::fs::File;
use std::io::Write;

pub struct LogQuery<'a> {
    logs: &'a Vec<String>,
}

impl<'a> LogQuery<'a> {
    pub fn new(logs: &'a Vec<String>) -> Self {
        LogQuery { logs }
    }

    pub fn search(&self, keyword: &str) -> Vec<&'a String> {
        self.logs
            .iter()
            .filter(|log| log.contains(keyword))
            .collect()
    }

    pub fn export_to_file(&self, keyword: &str, path: &str) -> std::io::Result<()> {
        let searched_logs = self.search(keyword);
        let mut buffer = File::create(path)?;
        for log in searched_logs.iter() {
            writeln!(buffer, "{}", log)?;
        }
        Ok(())
    }
}

fn main() {
    let logs = vec![String::from("INFO: Hello"), String::from("INFO: world"), String::from("ERROR: some errors")];
    let log_query = LogQuery::new(&logs);
    let _ = log_query.export_to_file("INFO", "test.log");
}
