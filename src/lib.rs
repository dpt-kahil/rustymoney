use chrono::NaiveDate;
use chrono::Datelike;
use std::io::Read;
use std::path::Path;
use std::io;
use std::fs::File;
use std::io::Write;
use std::fs::OpenOptions;

pub struct Account {
    id: u32,
    name: String,
    opening_balance: i64,
    current_balance: i64
}

impl Account {
    pub fn new(name: String, opening_balance: f64) -> Account {
        let id: u32 = get_last_id("accounts");

        let actual_opening_balance = (opening_balance * 100 as f64) as i64;
        return Account {
            id: id,
            name: name,
            opening_balance: actual_opening_balance,
            current_balance: actual_opening_balance
        }
    }

    pub fn write_to_file(&self) {
        let mut accounts_file = OpenOptions::new()
            .append(true)
            .open("D:\\rustymoney\\data\\accounts.csv")
            .expect("Could not open `accounts.csv`");

        let new_line = format!("{},{},{},{}\n", self.id, self.name, self.opening_balance, self.current_balance);

        accounts_file.write(new_line.as_bytes())
        .expect("Could not write new row to `accounts.csv`.");
    }
    
    pub fn display(&self) {
        println!("Account ID: {}", self.id);
        println!("Name: {}", self.name);
        println!("\t\tBalance");
        let fmt_opening_balance = (self.opening_balance as f64 / 100 as f64) as f64;
        let fmt_current_balance = (self.current_balance as f64 / 100 as f64) as f64;
        println!("Opening: {} | Current: {}", fmt_opening_balance, fmt_current_balance);
    }
}

pub struct Category {
    id: u32,
    name: String,
    description: String,
    date_created: NaiveDate
}

impl Category {
    pub fn new(name: String, description: String) -> Category {
        let id: u32 = get_last_id("categories");
        
        let current_date = chrono::Local::now().date_naive();
        let date_created: NaiveDate = NaiveDate::from_ymd_opt(current_date.year(), current_date.month(), current_date.day()).expect("Could not read the current date.");

        let new_description = String::from("\"") + &description + "\"";
        
        return Category {
            id: id,
            name: name,
            description: new_description,
            date_created: date_created
        }
    }

    pub fn write_to_file(&self) {
        let mut categories_file = OpenOptions::new()
            .append(true)
            .open("D:\\rustymoney\\data\\categories.csv")
            .expect("Could not open `categories.csv`.");

        let new_line = format!("{},{},{},{}\n", self.id, self.name, self.description, self.date_created);
        
        categories_file.write(new_line.as_bytes())
        .expect("Could not write new row to `categories.csv`.");
    }
    
    pub fn display(&self) {
        println!("Category ID: {}", self.id);
        println!("Name: {}", self.name);
        println!("Description: {}", self.description);
        println!("Created on: {:?}", self.date_created);
    }
}

pub fn create_category() {
    println!("CREATE NEW CATEGORY");
    println!("===================");
    println!("Enter category name:");

    let mut category_name = String::new();
    let mut category_desc = String::new();

    io::stdin()
    .read_line(&mut category_name).expect("Failed to read user input.");

    let category_name = category_name.trim_end();

    println!("Great!\nEnter the category description now:");

    io::stdin()
    .read_line(&mut category_desc).expect("Failed to read user input.");

    let category_desc = category_desc.trim_end();

    let new_category = Category::new(category_name.to_string(), category_desc.to_string());

    new_category.write_to_file();

    let success_message = format!("New category '{}' created!", category_name).to_string();
    let success_message_length = success_message.len();

    println!("{}", success_message);
    for _i in 0..success_message_length {
        print!("=");
    }
    println!("");

    new_category.display();
}

pub fn create_account() {
    println!("CREATE NEW ACCOUNT");
    println!("==================");
    println!("Enter account name:");

    let mut account_name = String::new();
    let mut opening_balance = String::new();

    io::stdin()
    .read_line(&mut account_name).expect("Failed to read user input.");

    let account_name = account_name.trim_end();

    println!("Great!\nEnter the current balance of the account:");

    io::stdin()
    .read_line(&mut opening_balance).expect("Failed to read user input.");

    let opening_balance = opening_balance.trim_end().parse::<f64>().expect("Please enter a numeric value!");

    let new_account = Account::new(account_name.to_string(), opening_balance);

    new_account.write_to_file();

    let success_message = format!("New account '{}' created!", account_name).to_string();
    let success_message_length = success_message.len();

    println!("{}", success_message);
    for _i in 0..success_message_length {
        print!("=");
    }
    println!("");

    new_account.display();
}

pub fn get_last_id(table: &str) -> u32 {
    let mut num_lines: u32= 0;
    let mut file_path = "D:\\rustymoney\\data\\".to_string();
    match table {
        "categories" => file_path.push_str("categories.csv"),
        "accounts" => file_path.push_str("accounts.csv"),
        "trades" => file_path.push_str("trades.csv"),
        _ => panic!("Invalid argument provided to get_last_id().")
    }

    let file_open = File::open(file_path);

    let mut file_data = match file_open {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error)
    };

    let mut file_contents = String::new();
    file_data.read_to_string(&mut file_contents).expect("get_last_id(): Could not read contents of file.");

    for _ in file_contents.lines() {
        num_lines += 1;
    }

    return num_lines;
}

pub fn create_files() {
    // Create Categories.csv
    if !Path::new("D:\\rustymoney\\data\\categories.csv").exists() {
        let mut categories_file = File::create("D:\\rustymoney\\data\\categories.csv").expect("Creation of `categories.csv` failed");

        categories_file.write("ID,Name,Description,Date Created\n".as_bytes()).expect("Could not write to `categories.csv`.");
    }

    if !Path::new("D:\\rustymoney\\data\\accounts.csv").exists() {
        let mut accounts_file = File::create("D:\\rustymoney\\data\\accounts.csv").expect("Creation of `accounts.csv` failed");

        accounts_file.write("ID,Name,Opening Balance,Current Balance\n".as_bytes()).expect("Could not write to `accounts.csv`.");
    }

    if !Path::new("D:\\rustymoney\\data\\trades.csv").exists() {
        let mut trades_file = File::create("D:\\rustymoney\\data\\trades.csv").expect("Creation of `trades.csv` failed");

        trades_file.write("ID,Date,Account,Category,Amount,Repayable,Description\n".as_bytes()).expect("Could not write to `trades.csv`.");
    }
}