use async_trait::async_trait;

use aws_custom_resource_provider_events::{
    ResponseStatus,
    ProviderResponse,
    ProviderResponseBuilder,
};
use aws_custom_resource_provider_lambda::{
    custom_resource_handler,
    HandlerConfig,
    types::CreateEvent,
    types::UpdateEvent,
    types::DeleteEvent,
    types::Provider,
};
use lambda_runtime::{Error as LambdaError};
use serde_derive::{ Serialize, Deserialize };
use serde_json::{ Value };
use aws_sdk_ssm as ssm;
use service_fn::service_fn;

use tracing::info;

//--- Resource properties specific to our custom-resource implementation
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CreateResourceProperties {
    pub ssm_name: String,
    pub ssm_desc: String,
    pub ssm_value: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct UpdateResourceProperties {
    pub ssm_name: String,
    pub ssm_desc: String,
    pub ssm_value: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct DeleteResourceProperties {
    pub ssm_name: String
}
//---

//--- our custom resource provider impl
#[derive(Clone, Debug)]
pub struct ProviderConfig {
    ssm: ssm::Client,
}

#[derive(Clone,Debug)]
struct ArnProvider {
    config: ProviderConfig,
}

impl ArnProvider {
    pub fn new(config: ProviderConfig) -> Self {
        ArnProvider {
            config: config,
        }
    }
}

#[async_trait]
impl Provider for ArnProvider {
    type Create = CreateEvent<CreateResourceProperties>;
    type Update = UpdateEvent<UpdateResourceProperties>;
    type Delete = DeleteEvent<DeleteResourceProperties>;

    async fn create(&self, create_event: Self::Create) -> ProviderResponse {
        info!("create event started");
        let event = create_event.0;
 
        if event.resource_properties.is_none() {
            return ProviderResponseBuilder::from_event(event)
                .status(ResponseStatus::Failed)
                .reason("Missing or invalid 'ResourceProperties' parameter".to_string())
                .build()
        }
        let props = event.resource_properties.as_ref().unwrap();

        let resp_res = self.config.ssm
            .put_parameter()
            .overwrite(true)
            .r#type(ssm::model::ParameterType::String)
            .name(props.ssm_name.clone())
            .value(props.ssm_value.clone())
            .description(props.ssm_desc.clone())
            .send()
            .await;
        if resp_res.is_err() {
            let aws_sdk_err = resp_res.unwrap_err();
            return ProviderResponseBuilder::from_event(event)
                .status(ResponseStatus::Failed)
                .reason(format!("Create error while attemting to call ssm::put_parameter: {:?}", aws_sdk_err))
                .build()
        }
 
        let resp = resp_res.unwrap();

        let data = vec![("ssm_param_ver", Value::from(resp.version()))]
            .into_iter()
            .collect::<Value>();

        ProviderResponseBuilder::from_event(event)
            .status(ResponseStatus::Success)
            .reason("Ok".to_string())
            .data(data)
            .build()
    }

    async fn update(&self, update_event: Self::Update) -> ProviderResponse {
        info!("update event started");
        let event = update_event.0;

        if event.resource_properties.is_none() {
            return ProviderResponseBuilder::from_event(event)
                .status(ResponseStatus::Failed)
                .reason("Missing or invalid 'ResourceProperties' parameter".to_string())
                .build()
        }
        let props = event.resource_properties.as_ref().unwrap();

        let resp_res = self.config.ssm
            .put_parameter()
            .overwrite(true)
            .r#type(ssm::model::ParameterType::String)
            .name(props.ssm_name.clone())
            .value(props.ssm_value.clone())
            .description(props.ssm_desc.clone())
            .send()
            .await;
        if resp_res.is_err() {
            let aws_sdk_err = resp_res.unwrap_err();
            return ProviderResponseBuilder::from_event(event)
                .status(ResponseStatus::Failed)
                .reason(format!("Update error while attemting to call ssm::put_parameter: {:?}", aws_sdk_err))
                .build()
        }

        let resp = resp_res.unwrap();

        let data = vec![("ssm_param_ver", Value::from(resp.version()))]
            .into_iter()
            .collect::<Value>();

        ProviderResponseBuilder::from_event(event)
            .status(ResponseStatus::Success)
            .reason("Ok".to_string())
            .data(data)
            .build()
    }

    async fn delete(&self, delete_event: Self::Delete) -> ProviderResponse {
        info!("delete event started");
        let event = delete_event.0;

        if event.resource_properties.is_none() {
            return ProviderResponseBuilder::from_event(event)
                .status(ResponseStatus::Failed)
                .reason("Missing or invalid 'ResourceProperties' parameter".to_string())
                .build()
        }
        let props = event.resource_properties.as_ref().unwrap();

        let resp_res = self.config.ssm
            .delete_parameter()
            .name(props.ssm_name.clone())
            .send()
            .await;

        if resp_res.is_err() {
            let aws_sdk_err = resp_res.unwrap_err();
            return ProviderResponseBuilder::from_event(event)
                .status(ResponseStatus::Failed)
                .reason(format!("Delete error while attemting to call ssm::delete_parameter: {:?}", aws_sdk_err))
                .build()
        }

        let _resp = resp_res.unwrap();

        ProviderResponseBuilder::from_event(event)
            .status(ResponseStatus::Success)
            .reason("Ok".to_string())
            .build()
    }
}


//---
#[tokio::main]
async fn main() -> Result<(), LambdaError> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .without_time() //disabled because CloudWatch will add ingest time.
        .init();

    info!("Lambda bootstrap invoked");

    let aws_config = aws_config::from_env().load().await;
    let ssm_client = ssm::Client::new(&aws_config);

    let provider_config = ProviderConfig {
        ssm: ssm_client,
    };

    // config aws-custom-resource-provider
    let handler_config = HandlerConfig::new(ArnProvider::new(provider_config));

    lambda_runtime::run(service_fn(handler_config, custom_resource_handler)).await?;

    Ok(())
}
