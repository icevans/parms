use crate::command_helpers::select_param_value;
use crate::param::Param;
use crate::ssm::Ssm;
use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use dialoguer::{Confirm, Editor};
use serde_json::Value;

mod command_helpers;
mod param;
mod ssm;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Search in this AWS region
    #[arg(short, long)]
    region: Option<String>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Creates a new parameter
    Create {
        /// The name of the parameter to create
        #[arg(short, long)]
        name: String,
        /// Whether to skip the usual check that the supplied value is valid JSON
        #[arg(short, long)]
        skip_json_validation: bool,
    },
    /// Fetches the value of selected parameter
    Fetch,
    /// Allows to edit the current value of selected parameter
    Edit {
        /// Whether to skip the usual check that the updated value is valid JSON
        #[arg(short, long)]
        skip_json_validation: bool,
    },
    /// Delete a parameter
    Delete,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    let ssm = Ssm::new(cli.region).await;

    match cli.command {
        Commands::Create {
            name,
            skip_json_validation,
        } => {
            let Some(new_text) = Editor::new().edit(&"")? else {
                println!("Creation aborted");
                return Ok(());
            };

            if !skip_json_validation {
                let _: Value = serde_json::from_str(&new_text)
                    .with_context(|| format!("Invalid json in: \r\n{}", new_text))?;
            }

            let param = Param::new(name, new_text);
            ssm.create_parameter(&param).await?;

            println!("Successfully created `{}`", param.name);

            Ok(())
        }
        Commands::Fetch => {
            let param = select_param_value(&ssm).await?;
            println!("{}", param.value);
            Ok(())
        }
        Commands::Edit {
            skip_json_validation,
        } => {
            let param = select_param_value(&ssm).await?;

            let Some(new_text) = Editor::new().edit(&param.value)? else {
                println!("Editing aborted");
                return Ok(());
            };

            if !skip_json_validation {
                let _: Value = serde_json::from_str(&new_text)
                    .with_context(|| format!("Invalid json in: \r\n{}", new_text))?;
            }

            ssm.update_parameter_value(&param.name, &new_text).await?;

            println!("Successfully updated `{}`", &param.value);

            Ok(())
        }
        Commands::Delete => {
            let param = select_param_value(&ssm).await?;

            let confirmation = Confirm::new()
                .with_prompt(format!("Delete parameter `{}`?", &param.name))
                .interact()?;

            if confirmation {
                ssm.delete_parameter(&param.name).await?;
                println!("Successfully deleted `{}`", param.name);
            } else {
                println!("Delete aborted.");
            }

            Ok(())
        }
    }
}
