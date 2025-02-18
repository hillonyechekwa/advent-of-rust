//this is a mini log query tool


pub struct LogQuery<'a> {
    logs: &'a Vec<String>,
}

// 1. Finish the implementation of LogQuery
impl<'a> LogQuery<'a> {
// 2. Create a public associated function named `new()` that will take a reference to a vector of strings
    pub fn new(logs: &'a Vec<String>) -> LogQuery {
            LogQuery{logs}
        }
// 3. Create a public method named `search` that accepts a string slice and finds it from the logs and
//    returns a vector of references to those logs.
        //&self must always be the first parameter
        //contains() is used to check if something exists in another
    pub fn search(&self, keyword: &str) -> Vec<&'a String>{
        let result = self.logs
        .iter()
        .filter(|log| log.contains(keyword))
        .collect::<Vec<_>>();

        return result
    }
}