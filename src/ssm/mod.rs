use aws_config::meta::region::RegionProviderChain;
use aws_sdk_ssm::config::Region;
use aws_sdk_ssm::types::Parameter;
use aws_sdk_ssm::Client;

pub async fn get_parameters(path: &str, region: &str) -> Option<Vec<Parameter>> {
    let region_provider = RegionProviderChain::first_try(Region::new(String::from(region)))
        .or_default_provider()
        .or_else("us-east-2");

    let config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&config);

    let resp: Result<Vec<_>, _> = client
        .get_parameters_by_path()
        .path(path)
        .with_decryption(true)
        .into_paginator()
        .send()
        .collect()
        .await;

    match resp {
        Ok(output) => {
            let mut parameters: Vec<Parameter> = Vec::new();
            for result in output {
                parameters.append(&mut result.parameters.unwrap())
            }
            Some(parameters)
        }
        Err(_) => None,
    }
}
