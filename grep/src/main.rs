//迭代器iterator
use std::env;
//处理运行退出程序
use std::process;

use grep::Config;

fn main() {
    //获取参数值并存入Vec<String>
    //let args: Vec<String> = env::args().collect();
    
    //parse_config提取参数
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("搜索关键词：{}", config.query);
    println!("在文件{}中搜索", config.file_path);

    if let Err(e) = grep::run(config) {
        eprintln!("错误：{e}");
        process::exit(1);
    }
}