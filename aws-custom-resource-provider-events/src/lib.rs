use serde_derive::{Deserialize,Serialize};
use serde_json::Value;


// see: https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/crpg-ref-requests.html#crpg-ref-request-fields


pub fn request_type(event: &Value) -> Option<RequestType> {
    event.get("RequestType").and_then(|s| s.as_str()).and_then(|v| match v {
        "Create" => Some(RequestType::Create),
        "Update" => Some(RequestType::Update),
        "Delete" => Some(RequestType::Delete),
        _ => None
    })
}

pub trait ProviderRequestEventDetails {
    fn request_type(&self) -> RequestType;
    fn response_url(&self) -> String;
    fn stack_id(&self) -> String;
    fn request_id(&self) -> String;
    fn resource_type(&self) -> String;
    fn logical_resource_id(&self) -> String;
}


#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum RequestType {
    #[serde(rename = "Create")]
    Create,
    #[serde(rename = "Update")]
    Update,
    #[serde(rename = "Delete")]
    Delete,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ProviderRequestCreateEvent<T> {
    /*
     * The request type is set by the AWS CloudFormation stack operation (create-stack, update-stack, or delete-stack)
     * that was initiated by the template developer for the stack that contains the custom resource.
     */
    #[serde(rename = "RequestType")]
    pub request_type: RequestType,

    /*
     * The response URL identifies a presigned S3 bucket that receives responses from the custom resource provider to AWS CloudFormation.
     */
    #[serde(rename = "ResponseURL")]
    pub response_url: String,

    /*
     * The Amazon Resource Name (ARN) that identifies the stack that contains the custom resource.
     * Combining the StackId with the RequestId forms a value that you can use to uniquely identify a request on a particular custom resource.
     */
    #[serde(rename = "StackId")]
    pub stack_id: String,

    /*
     * A unique ID for the request.
     * Combining the StackId with the RequestId forms a value that you can use to uniquely identify a request on a particular custom resource.
     */
    #[serde(rename = "RequestId")]
    pub request_id: String,

    /*
     * The template developer-chosen resource type of the custom resource in the AWS CloudFormation template.
     * Custom resource type names can be up to 60 characters long and can include alphanumeric and the following characters: _@-.
     */
    #[serde(rename = "ResourceType")]
    pub resource_type: String,

    /*
     * The template developer-chosen name (logical ID) of the custom resource in the AWS CloudFormation template.
     * This is provided to facilitate communication between the custom resource provider and the template developer.
     */
    #[serde(rename = "LogicalResourceId")]
    pub logical_resource_id: String,

    /*
     * This field contains the contents of the Properties object sent by the template developer.
     * Its contents are defined by the custom resource provider.
     */
    #[serde(rename = "ResourceProperties")]
    pub resource_properties: Option<T>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ProviderRequestUpdateEvent<T> {
    /*
     * The request type is set by the AWS CloudFormation stack operation (create-stack, update-stack, or delete-stack)
     * that was initiated by the template developer for the stack that contains the custom resource.
     */
    #[serde(rename = "RequestType")]
    pub request_type: RequestType,

    /*
     * The response URL identifies a presigned S3 bucket that receives responses from the custom resource provider to AWS CloudFormation.
     */
    #[serde(rename = "ResponseURL")]
    pub response_url: String,

    /*
     * The Amazon Resource Name (ARN) that identifies the stack that contains the custom resource.
     * Combining the StackId with the RequestId forms a value that you can use to uniquely identify a request on a particular custom resource.
     */
    #[serde(rename = "StackId")]
    pub stack_id: String,

    /*
     * A unique ID for the request.
     * Combining the StackId with the RequestId forms a value that you can use to uniquely identify a request on a particular custom resource.
     */
    #[serde(rename = "RequestId")]
    pub request_id: String,

    /*
     * The template developer-chosen resource type of the custom resource in the AWS CloudFormation template.
     * Custom resource type names can be up to 60 characters long and can include alphanumeric and the following characters: _@-.
     */
    #[serde(rename = "ResourceType")]
    pub resource_type: String,


    /*
     * The template developer-chosen name (logical ID) of the custom resource in the AWS CloudFormation template.
     * This is provided to facilitate communication between the custom resource provider and the template developer.
     */
    #[serde(rename = "LogicalResourceId")]
    pub logical_resource_id: String,

    /*
     * A required custom resource provider-defined physical ID that is unique for that provider.
     * Required: Always sent with Update and Delete requests; never sent with Create.
     */
    #[serde(rename = "PhysicalResourceId")]
    pub physical_resource_id: String,

    /*
     * This field contains the contents of the Properties object sent by the template developer.
     * Its contents are defined by the custom resource provider.
     */
    #[serde(rename = "ResourceProperties")]
    pub resource_properties: Option<T>,
   
    /*
     * Used only for Update requests.
     * Contains the resource properties that were declared previous to the update request.
     */
    #[serde(rename = "OldResourceProperties")]
    pub old_resource_properties: Option<T>,
}


#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ProviderRequestDeleteEvent<T> {
    /*
     * The request type is set by the AWS CloudFormation stack operation (create-stack, update-stack, or delete-stack)
     * that was initiated by the template developer for the stack that contains the custom resource.
     */
    #[serde(rename = "RequestType")]
    pub request_type: RequestType,

    /*
     * The response URL identifies a presigned S3 bucket that receives responses from the custom resource provider to AWS CloudFormation.
     */
    #[serde(rename = "ResponseURL")]
    pub response_url: String,

    /*
     * The Amazon Resource Name (ARN) that identifies the stack that contains the custom resource.
     * Combining the StackId with the RequestId forms a value that you can use to uniquely identify a request on a particular custom resource.
     */
    #[serde(rename = "StackId")]
    pub stack_id: String,

    /*
     * A unique ID for the request.
     * Combining the StackId with the RequestId forms a value that you can use to uniquely identify a request on a particular custom resource.
     */
    #[serde(rename = "RequestId")]
    pub request_id: String,

    /*
     * The template developer-chosen resource type of the custom resource in the AWS CloudFormation template.
     * Custom resource type names can be up to 60 characters long and can include alphanumeric and the following characters: _@-.
     */
    #[serde(rename = "ResourceType")]
    pub resource_type: String,


    /*
     * The template developer-chosen name (logical ID) of the custom resource in the AWS CloudFormation template.
     * This is provided to facilitate communication between the custom resource provider and the template developer.
     */
    #[serde(rename = "LogicalResourceId")]
    pub logical_resource_id: String,

    /*
     * A required custom resource provider-defined physical ID that is unique for that provider.
     * Required: Always sent with Update and Delete requests; never sent with Create.
     */
    #[serde(rename = "PhysicalResourceId")]
    pub physical_resource_id: String,

    /*
     * This field contains the contents of the Properties object sent by the template developer.
     * Its contents are defined by the custom resource provider.
     */
    #[serde(rename = "ResourceProperties")]
    pub resource_properties: Option<T>,
   
}


impl <T> ProviderRequestEventDetails for ProviderRequestCreateEvent<T>  {
    fn request_type(&self) -> RequestType { self.request_type.clone() }
    fn response_url(&self) -> String { self.response_url.clone() }
    fn stack_id(&self) -> String { self.stack_id.clone() }
    fn request_id(&self) -> String { self.request_id.clone() }
    fn resource_type(&self) -> String { self.resource_type.clone() }
    fn logical_resource_id(&self) -> String { self.logical_resource_id.clone() }

}


impl <T> ProviderRequestEventDetails for ProviderRequestUpdateEvent<T>  {
    fn request_type(&self) -> RequestType { self.request_type.clone() }
    fn response_url(&self) -> String { self.response_url.clone() }
    fn stack_id(&self) -> String { self.stack_id.clone() }
    fn request_id(&self) -> String { self.request_id.clone() }
    fn resource_type(&self) -> String { self.resource_type.clone() }
    fn logical_resource_id(&self) -> String { self.logical_resource_id.clone() }

}


impl <T> ProviderRequestEventDetails for ProviderRequestDeleteEvent<T>  {
    fn request_type(&self) -> RequestType { self.request_type.clone() }
    fn response_url(&self) -> String { self.response_url.clone() }
    fn stack_id(&self) -> String { self.stack_id.clone() }
    fn request_id(&self) -> String { self.request_id.clone() }
    fn resource_type(&self) -> String { self.resource_type.clone() }
    fn logical_resource_id(&self) -> String { self.logical_resource_id.clone() }

}


//#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
//pub struct ProviderRequestEvent<T> {
//    /*
//     * The request type is set by the AWS CloudFormation stack operation (create-stack, update-stack, or delete-stack)
//     * that was initiated by the template developer for the stack that contains the custom resource.
//     */
//    #[serde(rename = "RequestType")]
//    pub request_type: RequestType,
//
//    /*
//     * The response URL identifies a presigned S3 bucket that receives responses from the custom resource provider to AWS CloudFormation.
//     */
//    #[serde(rename = "ResponseURL")]
//    pub response_url: String,
//
//    /*
//     * The Amazon Resource Name (ARN) that identifies the stack that contains the custom resource.
//     * Combining the StackId with the RequestId forms a value that you can use to uniquely identify a request on a particular custom resource.
//     */
//    #[serde(rename = "StackId")]
//    pub stack_id: String,
//
//    /*
//     * A unique ID for the request.
//     * Combining the StackId with the RequestId forms a value that you can use to uniquely identify a request on a particular custom resource.
//     */
//    #[serde(rename = "RequestId")]
//    pub request_id: String,
//
//    /*
//     * The template developer-chosen resource type of the custom resource in the AWS CloudFormation template.
//     * Custom resource type names can be up to 60 characters long and can include alphanumeric and the following characters: _@-.
//     */
//    #[serde(rename = "ResourceType")]
//    pub resource_type: String,
//
//
//    /*
//     * The template developer-chosen name (logical ID) of the custom resource in the AWS CloudFormation template.
//     * This is provided to facilitate communication between the custom resource provider and the template developer.
//     */
//    #[serde(rename = "LogicalResourceId")]
//    pub logical_resource_id: String,
//
//    /*
//     * A required custom resource provider-defined physical ID that is unique for that provider.
//     * Required: Always sent with Update and Delete requests; never sent with Create.
//     */
//    #[serde(rename = "PhysicalResourceId")]
//    pub physical_resource_id: Option<String>,
//
//    /*
//     * This field contains the contents of the Properties object sent by the template developer.
//     * Its contents are defined by the custom resource provider.
//     */
//    #[serde(rename = "ResourceProperties")]
//    pub resource_properties: Option<T>,
//   
//    /*
//     * Used only for Update requests.
//     * Contains the resource properties that were declared previous to the update request.
//     */
//    #[serde(rename = "OldResourceProperties")]
//    pub old_resource_properties: Option<T>,
//}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum ResponseStatus {
    #[serde(rename = "SUCCESS")]
    Success,
    #[serde(rename = "FAILED")]
    Failed,
}

pub struct ProviderResponseBuilder {
    status: ResponseStatus,
    reason: String,
    physical_resource_id: String,
    stack_id: String,
    request_id: String,
    logical_resource_id: String,
    no_echo: bool,
    data: Option<Value>,
}

impl ProviderResponseBuilder {
    pub fn from_event <T> (event:T) -> Self where T: ProviderRequestEventDetails {
        let physical_resource_id = build_physical_resource_id(
            event.stack_id(),
            event.logical_resource_id(),
        );
 
        ProviderResponseBuilder {
            status: ResponseStatus::Failed,
            reason: "reason not given".to_string(),
            physical_resource_id: physical_resource_id,
            stack_id: event.stack_id(),
            request_id: event.request_id(),
            logical_resource_id: event.logical_resource_id(),
            no_echo: false,
            data: None as Option<Value>,
        }
    }
     
    pub fn status(mut self, status: ResponseStatus) -> ProviderResponseBuilder {
        self.status = status;
        self
    }
    pub fn reason(mut self, reason: String) -> ProviderResponseBuilder {
        self.reason = reason;
        self
    }
    pub fn physical_resource_id(mut self, physical_resource_id: String) -> ProviderResponseBuilder {
        self.physical_resource_id = physical_resource_id;
        self
    }
    pub fn stack_id(mut self, stack_id: String) -> ProviderResponseBuilder {
        self.stack_id = stack_id;
        self
    }
   
    pub fn request_id(mut self, request_id: String) -> ProviderResponseBuilder {
        self.request_id = request_id;
        self
    }
    pub fn logical_resource_id(mut self, logical_resource_id: String) -> ProviderResponseBuilder {
        self.logical_resource_id = logical_resource_id;
        self
    }
    pub fn no_echo(mut self, no_echo: bool) -> ProviderResponseBuilder {
        self.no_echo = no_echo;
        self
    }
    pub fn data(mut self, data: Value) -> ProviderResponseBuilder {
        self.data = Some(data);
        self
    }

    pub fn build (self) -> ProviderResponse {
        ProviderResponse {
            status: self.status,
            reason: self.reason,
            physical_resource_id: self.physical_resource_id,
            stack_id: self.stack_id,
            request_id: self.request_id,
            logical_resource_id: self.logical_resource_id,
            no_echo: self.no_echo,
            data: self.data,
        }
    }
}

pub fn build_physical_resource_id(stack_id: String, logical_resource_id: String)  -> String {
    let mut phy_dig_ctx = ring::digest::Context::new(&ring::digest::SHA256);
    phy_dig_ctx.update(stack_id.as_bytes());
    phy_dig_ctx.update(logical_resource_id.as_bytes());
    phy_dig_ctx.finish().as_ref().iter().map(|b| format!("{:02x}",b)).collect::<String>()
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ProviderResponse {
    /*
     * The status value sent by the custom resource provider in response to an AWS CloudFormation-generated request.
     */
    #[serde(rename = "Status")]
    pub status: ResponseStatus,

    /*
     * Describes the reason for a failure response.
     * Required: Required if Status is FAILED. It's optional otherwise.
     */
    #[serde(rename = "Reason")]
    pub reason: String,

    /*
     * This value should be an identifier unique to the custom resource vendor, and can be up to 1 KB in size.
     * The value must be a non-empty string and must be identical for all responses for the same resource.
     */
    #[serde(rename = "PhysicalResourceId")]
    pub physical_resource_id: String,


    /*
     * The Amazon Resource Name (ARN) that identifies the stack that contains the custom resource.
     * This response value should be copied verbatim from the request.
     */
    #[serde(rename = "StackId")]
    pub stack_id: String,

    /*
     * A unique ID for the request.
     * This response value should be copied verbatim from the request.
     */
    #[serde(rename = "RequestId")]
    pub request_id: String,

    /*
     * The template developer-chosen name (logical ID) of the custom resource in the AWS CloudFormation template.
     * This response value should be copied verbatim from the request.
     */
    #[serde(rename = "LogicalResourceId")]
    pub logical_resource_id: String,

    /*
     *  Indicates whether to mask the output of the custom resource when retrieved by using the Fn::GetAtt function.
     *  If set to true, all returned values are masked with asterisks (*****), except for those stored in the Metadata section of the template.
     *  AWS CloudFormation does not transform, modify, or redact any information you include in the Metadata section.
     *  The default value is false.
     */
    #[serde(rename = "NoEcho")]
    pub no_echo: bool,

    /*
     * The custom resource provider-defined name-value pairs to send with the response.
     * You can access the values provided here by name in the template with Fn::GetAtt.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "Data")]
    pub data: Option<Value>,
}

