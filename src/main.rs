use std::env;
use std::process;
use rustymoney;

fn run_three() {
    println!("Run Three");
}

fn main() {
    const CREATE_CATEGORY: &str = "create-category";
    const CREATE_ACCOUNT: &str = "create-account";

    rustymoney::create_files();

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Not enough arguments to start!");
        process::exit(1);
    }

    let argument = args[1].clone();
    let argument: &str = &argument;

    match argument {
        CREATE_CATEGORY => rustymoney::create_category(),
        CREATE_ACCOUNT => rustymoney::create_account(),
        _ => run_three()
    }
}
