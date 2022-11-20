pub mod types;
pub mod error;

use error::*;
use types::*;

use std::str::FromStr;
use tracing::info;
use lambda_runtime::{LambdaEvent, Error as LambdaError};
use serde_json::{Value};

pub type Request = LambdaEvent<Value>;
pub type Response = Result<(), LambdaError>;
pub type Client = hyper::Client<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>>;

use aws_custom_resource_provider_events::{
    RequestType,
    request_type,
};
use serde::de::DeserializeOwned;

#[derive(Clone, Debug)]
pub struct HandlerConfig<P> 
where P: Provider
{
    pub provider: P,
    pub hyper: Client,
}

impl <P, C, U, D> HandlerConfig<P>
where 
     P: Provider<Create=CreateEvent<C>,
                 Update=UpdateEvent<U>,
                 Delete=DeleteEvent<D>>,
    C: DeserializeOwned,
    U: DeserializeOwned,
    D: DeserializeOwned,
{
    pub fn new_with_client(provider:P, client: Client) -> HandlerConfig<P> {
        HandlerConfig {
            provider: provider,
            hyper: client,
        }
    }
    pub fn new(provider: P) -> HandlerConfig<P> {
        let https = hyper_rustls::HttpsConnectorBuilder::new()
            .with_native_roots()
            .https_only()
            .enable_http1()
            .enable_http2()
            .build();

        let client: Client = hyper::Client::builder().build(https);
        HandlerConfig::new_with_client(provider,client)
    }
}


// 1. determine what the request type is
// 2. deserialize request event for given request type.
// 3. delegate to request-type-specific handler within the provider
// 4. receive response result from request-type handler
// 5. write reponse to the presigned s3 url
pub async fn custom_resource_handler<P, C, U, D>(config: HandlerConfig<P>, event: Request) -> Response
where 
    P: Provider<Create=CreateEvent<C>,
                Update=UpdateEvent<U>,
                Delete=DeleteEvent<D>>,
    C: DeserializeOwned,
    U: DeserializeOwned,
    D: DeserializeOwned,
{
    use RequestType::*;
    use Error::*;
    info!("Lambda invoked, event: {:?}", &event);
    
    let request = event.payload;

    let response_url = request.get("RequestType")
        .and_then(|v| v.as_str())
        .ok_or( Error::InvalidResponseURI() )
        .and_then(|s|
            hyper::Uri::from_str(&s)
            .map_err(|_| Error::InvalidResponseURI() )
        )?;

    let req_type = request_type(&request)
        .ok_or(InvalidRequestType(request.clone()))?;

    let provider_response = match req_type {
        Create => {config.provider.create(CreateEvent::try_from(request.clone())?).await},
        Update => {config.provider.update(UpdateEvent::try_from(request.clone())?).await},
        Delete => {config.provider.delete(DeleteEvent::try_from(request.clone())?).await},
    };

    let as_json_response = serde_json::to_string(&provider_response)
        .map_err::<LambdaError, _>(std::convert::Into::into)?;

    // the request includes a presigned s3 URL to write our response to
    let s3_resp = hyper::Request::builder()
        .method(hyper::Method::PUT)
        .uri(response_url)
        .header("content-type", "application/json")
        .body(hyper::Body::from(as_json_response))
        .map_err::<LambdaError, _>(std::convert::Into::into)?;

    // write the response to the presigned s3 URL
    config.hyper.request(s3_resp).await?;
    Ok(())
}

