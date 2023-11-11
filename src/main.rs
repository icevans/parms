use std::process::exit;

use clap::Parser;
use dialoguer::Select;

mod ssm;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Search for parameters under this path
    #[arg(short, long)]
    path: String,

    /// Search in this AWS region
    #[arg(short, long, default_value_t = String::from("us-west-2"))]
    region: String,
}

/// Fetches a parameter and displays the decrypted value
#[tokio::main]
async fn main() -> () {
    let args = Args::parse();

    let parameters = ssm::get_parameters(&args.path, &args.region).await;

    if parameters.is_none() {
        eprintln!("No parameters found under that path");
        exit(1);
    }

    let parameters = parameters.unwrap();

    let parameter_names: Vec<&str> = parameters
        .iter()
        .map(|parameter| parameter.name().unwrap_or("unnamed"))
        .collect();

    let selection = Select::new()
        .with_prompt("What do you choose?")
        .items(&parameter_names)
        .interact()
        .unwrap();

    println!("{}", parameters[selection].value().unwrap());
}
