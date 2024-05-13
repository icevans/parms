use anyhow::{anyhow, bail, Context, Result};
use clap::{Parser, Subcommand};
use dialoguer::{theme::ColorfulTheme, Editor, FuzzySelect};
use serde_json::Value;

use crate::ssm::Ssm;

mod ssm;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Search in this AWS region
    #[arg(short, long, default_value_t = String::from("us-west-2"))]
    region: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Fetches the value of selected parameter
    Fetch,
    /// Allows to edit the current value of selected parameter
    Edit {
        /// Whether to skip the usual check that the updated value is valid JSON
        #[arg(short, long)]
        skip_json_validation: bool,
    },
}

/// Fetches a parameter and displays the decrypted value
#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    let ssm = Ssm::new(&cli.region).await;
    let parameter_names = ssm.get_parameter_names().await?;

    if parameter_names.is_empty() {
        bail!("no parameters found in region {}", cli.region);
    }

    let selected_index = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select parameter (type for fuzzy search):")
        .max_length(10)
        .items(&parameter_names)
        .interact()
        .unwrap();

    let value = ssm
        .get_parameter_value(&parameter_names[selected_index])
        .await
        .ok_or(anyhow!("oops"))?;

    match &cli.command {
        Commands::Fetch => {
            println!("{value}");
            Ok(())
        }
        Commands::Edit {
            skip_json_validation,
        } => {
            let Some(new_text) = Editor::new().edit(&value).unwrap() else {
                println!("Editing aborted");
                return Ok(());
            };

            if !*skip_json_validation {
                let _: Value = serde_json::from_str(&new_text)
                    .with_context(|| format!("Invalid json in: \r\n{}", new_text))?;
            }

            ssm.update_parameter_value(&parameter_names[selected_index], &new_text)
                .await?;

            println!(
                "Successfully updated `{}`",
                &parameter_names[selected_index]
            );

            Ok(())
        }
    }
}
