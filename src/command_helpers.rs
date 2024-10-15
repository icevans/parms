use crate::param::Param;
use crate::ssm::Ssm;
use anyhow::bail;
use dialoguer::theme::ColorfulTheme;
use dialoguer::FuzzySelect;

pub async fn select_param_value(ssm: &Ssm) -> anyhow::Result<Param> {
    let mut parameter_names = ssm.get_parameter_names().await?;

    if parameter_names.is_empty() {
        bail!("no parameters found -- is your region configured?");
    }

    let selected_index = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select parameter (type for fuzzy search):")
        .max_length(10)
        .items(&parameter_names)
        .interact()
        .unwrap();

    let value = ssm
        .get_parameter_value(&parameter_names[selected_index])
        .await?;

    Ok(Param::new(parameter_names.remove(selected_index), value))
}
