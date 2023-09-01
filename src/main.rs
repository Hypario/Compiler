use std::env;
use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Incorrect usage. Correct usage is...");
        println!("compiler <input.co>");
        
        return ExitCode::FAILURE;
    }

    dbg!(args);
    
    return ExitCode::SUCCESS;
}
