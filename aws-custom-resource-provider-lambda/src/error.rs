use std::error;
use std::fmt;
use aws_custom_resource_provider_events::{ RequestType  };

//#[derive(PartialEq)]
pub enum Error {
    /// the RequestType value sent in the request event is not a recognized type, Create, Update, Delete are currently supported
    InvalidRequestType(serde_json::Value),

    /// the ResponseURL paramater failed to parse into a valid  Uri value from the Hyper crate
    InvalidResponseURI(),

    /// an event was received, with the given Request Type, but SerDe deserialization failed.
    InvalidEvent(serde_json::Error, RequestType),
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::InvalidRequestType(value) => 
                f.debug_struct("InvalidRequestType")
                    .field("event", &format_args!("{}", serde_json::to_string(&value).unwrap_or("<event unavailable>".to_string())))
                    .finish(),
            Self::InvalidEvent(serde_err, event_request_type) =>
                f.debug_struct("InvalidCreateEvent")
                    .field("serde_error", &format_args!("{:?}", serde_err))
                    .field("request_type", &format_args!("{:?}", event_request_type))
                    .finish(),
            Self::InvalidResponseURI() =>
                f.debug_struct("InvalidResponseURI")
                    .finish(),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self, f)
    }
}


impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Self::InvalidRequestType(_) => "invalid or unknown RequestType",
            Self::InvalidEvent(_,_) => "invalid result from event deserialization attempt",
            Self::InvalidResponseURI() => "invalid or missing ResponseURI value in the event",
        }
    }
}

