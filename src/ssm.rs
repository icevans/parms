use aws_config::meta::region::RegionProviderChain;
use aws_sdk_ssm::operation::describe_parameters::DescribeParametersOutput;
use aws_sdk_ssm::{config::Region, Client};

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

    pub async fn get_parameter_names(&self) -> Option<Vec<String>> {
        let paged_response: Result<Vec<DescribeParametersOutput>, _> = self
            .client
            .describe_parameters()
            .into_paginator()
            .page_size(50)
            .send()
            .collect()
            .await;

        let paged_response = paged_response.ok()?;

        let names: Vec<String> = paged_response
            .into_iter()
            .flat_map(|page_of_parameters| {
                page_of_parameters.parameters.expect("wtf an empty page")
            })
            .map(|parameter| parameter.name.expect("wtf a parameter without a name"))
            .collect();

        if names.is_empty() {
            None
        } else {
            Some(names)
        }
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
}
