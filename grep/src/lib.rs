use std::error::Error;
//处理文件
use std::fs;
//处理环境变量的函数位于标准库env模块
use std::env;
use regex::Regex;

pub struct Config {
    pub query: String,
    pub file_path: String,
    //pub ignore_case: bool,
    regex: Regex,
}

impl Config {
    pub fn build(
        mut args: impl Iterator<Item = String>,
    ) -> Result<Config, &'static str> {
        args.next();

        // if args.len() < 3{
        //     return Err("没有输入足够的参数");
        // }

        // let query = args[1].clone();

        // let file_path = args[2].clone();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("没有输入搜索字符串"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("没有输入搜索路径"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();
        let regex_pattern = if ignore_case {
            format!("(?i){}", query)
        } else {
            query.clone()
        };

        let regex = Regex::new(&regex_pattern).map_err(|_| "Invalid regex")?;

        Ok(Config { query, file_path, regex, })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    //fs.read_to_string(file_path) 返回包含其内容的 std::io::Result<String>
    let contents = fs::read_to_string(config.file_path)?;
    
    // let results = if config.ignore_case {
    //     search_case_insensitive(&config.query, &contents)
    // } else {
    //     search(&config.query, &contents)
    // };

    let results = search_with_regex(&config.regex, &contents);

    for line in results {
        println!("{line}");
    }

    Ok(())
}


pub fn search<'a>(query: &str, contents:&'a str) -> Vec<&'a str> {
    // let mut results = Vec::new();

    // for line in contents.lines() {
    //     if line.contains(query) {
    //         results.push(line);
    //     }
    // }
    // results
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents:&'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}


fn search_with_regex<'a>(regex: &Regex, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| regex.is_match(line))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    
    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:","Trust me."],
            search_case_insensitive(query, contents)
        );
    }

    #[test]
    fn regex_search() {
        //?i大小写不敏感匹配
        let pattern = r"(?i)\brust\b";
        let regex = Regex::new(pattern).unwrap();
        let contents = "\
Rust:
safe, fast, productive.
Trust me.";

    assert_eq!(
        vec!["Rust:"],
        search_with_regex(&regex, contents)
    );
    }
    
}