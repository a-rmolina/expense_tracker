use std::io::{Write, stdin, stdout};
#[derive(Debug)]
enum TransactionType {
    Income,
    Expense,
}

#[allow(dead_code)]
#[derive(Debug)]
struct Transaction {
    kind: TransactionType,
    amount: f32,
    description: String,
    date: String,
    tags: Vec<String>,
}

fn ask_for_field(ask_message: &str) -> String {
    print!("{ask_message}");
    let _ = stdout().flush();
    let mut value = String::new();
    stdin()
        .read_line(&mut value)
        .expect("Did not enter a valid string");
    value.trim().to_string()
}

fn parse_float_form_command_line(input_message: &str) -> f32 {
    loop {
        let input_amount = ask_for_field(input_message);
        match input_amount.parse::<f32>() {
            Ok(number) => {
                return number;
            }
            Err(e) => {
                println!("Invalid amount. Couldn't parse the value due to {}", e);
                continue;
            }
        };
    }
}

fn ask_for_income_fields() -> String {
    let mut income_str = String::new();
    let int_amount =
        parse_float_form_command_line("Please, introduce a valid amount (e.g. 12.56): ");
    income_str.push_str(&int_amount.to_string());
    income_str
}

fn ask_for_transaction_item() -> String {
    let final_message = String::from("You wish! Second chances are not a thing here...");
    let transaction_type = loop {
        let choice = ask_for_field(
            "Select Transaction Type\n\t 1. Income.\n\t 2. Expense.\n >> (type number or name): ",
        );
        match choice.to_lowercase().as_str() {
            "1" | "1." | "income" => break TransactionType::Income,
            "2" | "2." | "expense" => break TransactionType::Expense,
            _ => {
                println!("Invalid option. Please try again.");
                continue;
            }
        }
    };
    let data = ask_for_income_fields();
    println!("For the transaction type {:?}", transaction_type);
    println!("You introduced the following data:\n{}", data);
    final_message
}
fn main() {
    let final_message = ask_for_transaction_item();
    println!("{final_message}");
}
