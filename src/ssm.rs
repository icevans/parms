use crate::param::Param;
use anyhow::anyhow;
use aws_sdk_ssm::{config::Region, Client, Error};
use aws_sdk_ssm::types::ParameterType::SecureString;

pub struct Ssm {
    client: Client,
}

impl Ssm {
    pub async fn new(region: Option<String>) -> Self {
        let config = match region {
            None => aws_config::from_env().load().await,
            Some(region) => {
                aws_config::from_env()
                    .region(Region::new(region))
                    .load()
                    .await
            }
        };

        Self {
            client: Client::new(&config),
        }
    }

    pub async fn create_parameter(&self, param: &Param) -> Result<(), Error> {
        self
            .client
            .put_parameter()
            .name(&param.name)
            .value(&param.value)
            .r#type(SecureString)
            .send()
            .await?;

        Ok(())
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

    pub async fn get_parameter_value(&self, parameter_name: &str) -> anyhow::Result<String> {
        let response = self
            .client
            .get_parameter()
            .name(parameter_name)
            .with_decryption(true)
            .send()
            .await?;

        match response.parameter {
            None => Err(anyhow!(
                "AWS SDK fault: success response, but empty response"
            )),
            Some(parm) => Ok(parm.value.unwrap_or_else(|| "".to_string())),
        }
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
