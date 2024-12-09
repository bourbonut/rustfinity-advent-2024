pub struct LogQuery<'a> {
    logs: &'a Vec<String>,
}

impl<'a> LogQuery<'a> {
    pub fn new(log_inputs: &'a Vec<String>) -> LogQuery {
        LogQuery { logs: log_inputs }
    }

    pub fn search(&self, keyword: &str) -> Vec<&'a String> {
        self.logs.iter().filter(|log| log.contains(keyword)).collect::<Vec<_>>()
    }
}

fn main() {
    let logs = vec![String::from("INFO: Hello"), String::from("INFO: world"), String::from("ERROR: some errors")];
    let log_query = LogQuery::new(&logs);
    assert_eq!(log_query.search("INFO"), vec![&logs[0], &logs[1]]);
}
