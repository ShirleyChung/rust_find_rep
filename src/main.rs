use text_colorizer::*;
use std::env;

#[derive(Debug)]
struct Arguments {
    target: String,
    replacement: String,
    filename: String,
    output: String,
}

fn parse_args() -> Arguments {
    let args: Vec<String> = env::args().skip(1).collect(); // 略掉第一個參數即本程式檔名

    if args.len() < 4 {
        println!("{} 參數不足, 至少需要4個參數", "錯誤".red());
        std::process::exit(1);
    }

    Arguments { target: args[0].clone(), 
                replacement: args[1].clone(), 
                filename: args[2].clone(), 
                output: args[3].clone() }
}

fn main() {
    let args = parse_args();
    println!("Hello, world! {} {} {} {}", 
            args.target.red(), 
            args.replacement.yellow(), 
            args.filename.green(), 
            args.output.purple());
}
