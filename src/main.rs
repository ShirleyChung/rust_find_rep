use text_colorizer::*;
use std::env;
use regex::Regex;
use std::fs;

#[derive(Debug)]
struct Arguments {
    target: String,
    replacement: String,
    filename: String,
    output: String,
}

/**
解析參數
*/
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

/**
使用regex替換字串
*/
fn replace(target: &str, replacement: &str, text: &str) -> Result<String, regex::Error> {
    let regex = Regex::new(target)?;
    Ok(regex.replace_all(text, replacement).to_string())
}

fn main() {
    let args = parse_args();

    if let Ok(data) = fs::read_to_string(&args.filename) {
        if let Ok(replaced_data) = replace(&args.target, &args.replacement, &data) {
            if let Err(e) = fs::write(&args.output, &replaced_data) {
                println!("fail to write {}: {:?}", args.output, e);
            } else {
                println!("{} save successfully!", args.output);
            }

        } else {
            println!("error to replace with {}", &args.replacement.red());
        }
    } else {
        println!("error to read {}", &args.filename.yellow());
    }
}
