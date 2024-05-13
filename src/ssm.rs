use aws_config::meta::region::RegionProviderChain;
use aws_sdk_ssm::{config::Region, Client, Error};

pub struct Ssm {
    client: Client,
}

impl Ssm {
    pub async fn new(region: &str) -> Self {
        let region_provider = RegionProviderChain::first_try(Region::new(String::from(region)))
            .or_default_provider()
            .or_else("us-east-2");

        let config = aws_config::from_env().region(region_provider).load().await;

        Self {
            client: Client::new(&config),
        }
    }

    pub async fn get_parameter_names(&self) -> Result<Vec<String>, Error> {
        let paged_response: Result<Vec<_>, _> = self
            .client
            .describe_parameters()
            .into_paginator()
            .page_size(50)
            .send()
            .collect()
            .await;

        let names: Vec<String> = paged_response?
            .into_iter()
            .flat_map(|page_of_parameters| {
                page_of_parameters.parameters.expect("wtf an empty page")
            })
            .map(|parameter| parameter.name.expect("wtf a parameter without a name"))
            .collect();

        Ok(names)
    }

    pub async fn get_parameter_value(&self, parameter_name: &str) -> Option<String> {
        let response = self
            .client
            .get_parameter()
            .name(parameter_name)
            .with_decryption(true)
            .send()
            .await;

        let response = response.ok()?;
        let parameter = response.parameter?;

        parameter.value.or(Some("".to_owned()))
    }

    pub async fn update_parameter_value(
        &self,
        parameter_name: &str,
        new_value: &str,
    ) -> Result<(), Error> {
        let response = self
            .client
            .put_parameter()
            .name(parameter_name)
            .value(new_value)
            .overwrite(true)
            .send()
            .await;

        if let Err(e) = response {
            return Err(e.into());
        }

        Ok(())
    }
}
