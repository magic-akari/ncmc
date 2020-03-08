use glob::glob;
use std::env;
use std::error;
use std::fs::metadata;
use std::path::PathBuf;

use ncmc::convert;

fn main() -> Result<(), Box<dyn error::Error>> {
    let args: Vec<String> = env::args().collect();

    let file_list = match args.len() {
        0 => unimplemented!(),
        1 => glob("**/*.ncm")?.filter_map(Result::ok).collect::<Vec<_>>(),
        2 => {
            let arg = &args[1];
            if metadata(arg)?.is_file() {
                [PathBuf::from(arg)].to_vec()
            } else {
                let path = [arg, "**", "*.ncm"].iter().collect::<PathBuf>();
                glob(path.to_str().unwrap())?
                    .filter_map(Result::ok)
                    .collect::<Vec<_>>()
            }
        }
        _ => args[1..].iter().map(PathBuf::from).collect::<Vec<_>>(),
    };

    println!("total: {}", file_list.len());
    println!("{:-^32}", "");

    let mut success = 0;
    let mut failed = 0;

    for file in file_list {
        println!(
            "{:>4} <-\t{}\n",
            success + failed + 1,
            file.to_str().unwrap()
        );
        match convert(file) {
            Ok(target_path) => {
                success += 1;
                println!("\n{:>4} ->\t{}", "ok", target_path.to_str().unwrap())
            }
            Err(msg) => {
                failed += 1;
                println!("\n{:>4} ->\t{}", "x", msg)
            }
        }
        println!("{:-^32}", "");
    }

    println!("{} success, {} failed.", success, failed);

    Ok(())
}
