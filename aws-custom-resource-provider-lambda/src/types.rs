use crate::error::{ Error, Error::* };
use serde_json::{self, Value};
use aws_custom_resource_provider_events::{
    ProviderRequestCreateEvent,
    ProviderRequestUpdateEvent,
    ProviderRequestDeleteEvent,
    ProviderResponse,
    RequestType,
};
use async_trait::async_trait;
use std::convert::TryFrom;
use serde::de::DeserializeOwned;

//--- Create Event types
pub struct CreateEvent<P>(pub ProviderRequestCreateEvent<P>)
where P: DeserializeOwned;

impl <P> TryFrom<Value> for CreateEvent<P>
where P: DeserializeOwned {
    type Error = Error;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
            .map(|p| CreateEvent(p))
            .map_err(|e| InvalidEvent(e, RequestType::Create))
    }
}

//--- Update Event types
pub struct UpdateEvent<P>(pub ProviderRequestUpdateEvent<P>)
where P: DeserializeOwned;

impl <P> TryFrom<Value> for UpdateEvent<P>
where P: DeserializeOwned {
    type Error = Error;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
            .map(|p| UpdateEvent(p))
            .map_err(|e| InvalidEvent(e, RequestType::Update))
    }
}

//--- Delete Event types
pub struct DeleteEvent<P>(pub ProviderRequestDeleteEvent<P>)
where P: DeserializeOwned;

impl <P> TryFrom<Value> for DeleteEvent<P>
where P: DeserializeOwned {
    type Error = Error;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
            .map(|p| DeleteEvent(p))
            .map_err(|e| InvalidEvent(e, RequestType::Delete))
    }
}



#[async_trait]
pub trait Provider {
    type Create: TryFrom<Value, Error=Error>;
    type Update: TryFrom<Value, Error=Error>;
    type Delete: TryFrom<Value, Error=Error>;

    async fn create(&self, create_event: Self::Create) -> ProviderResponse;
    async fn update(&self, update_event: Self::Update) -> ProviderResponse;
    async fn delete(&self, delete_event: Self::Delete) -> ProviderResponse;
}
