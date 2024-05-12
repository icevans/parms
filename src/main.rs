use std::process::exit;

use clap::Parser;
use dialoguer::{theme::ColorfulTheme, FuzzySelect};

use crate::ssm::Ssm;

mod ssm;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Search in this AWS region
    #[arg(short, long, default_value_t = String::from("us-west-2"))]
    region: String,
}

/// Fetches a parameter and displays the decrypted value
#[tokio::main]
async fn main() {
    let args = Args::parse();
    let ssm = Ssm::new(&args.region).await;

    let Some(parameter_names) = ssm.get_parameter_names().await else {
        eprintln!("No parameters exist in this region");
        exit(1);
    };

    let selected_index = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select parameter (type for fuzzy search):")
        .max_length(10)
        .items(&parameter_names)
        .interact()
        .unwrap();

    let value = ssm
        .get_parameter_value(&parameter_names[selected_index])
        .await;

    match value {
        None => println!("oops"),
        Some(value) => println!("{value}"),
    }
}
