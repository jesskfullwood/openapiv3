use crate::*;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Describes a single API operation on a path.
pub struct Operation {
    /// A list of tags for API documentation control.
    /// Tags can be used for logical grouping of operations
    /// by resources or any other qualifier.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
    /// A short summary of what the operation does.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    /// A verbose explanation of the operation behavior.
    /// CommonMark syntax MAY be used for rich text representation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Additional external documentation for this operation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_documentation: Option<ExternalDocumentation>,
    /// Unique string used to identify the operation.
    /// The id MUST be unique among all operations described in the API.
    /// Tools and libraries MAY use the operationId to uniquely identify
    /// an operation, therefore, it is RECOMMENDED to follow common
    /// programming naming conventions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<String>,
    /// A list of parameters that are applicable for this operation.
    /// If a parameter is already defined at the Path Item, the new
    /// definition will override it but can never remove it.
    /// The list MUST NOT include duplicated parameters. A unique
    /// parameter is defined by a combination of a name and location.
    /// The list can use the Reference Object to link to parameters
    /// that are defined at the OpenAPI Object's components/parameters.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub parameters: Vec<ReferenceOr<Parameter>>,
    /// The request body applicable for this operation.
    /// The requestBody is only supported in HTTP methods
    /// where the HTTP 1.1 specification RFC7231 has explicitly
    /// defined semantics for request bodies. In other cases where
    /// the HTTP spec is vague, requestBody SHALL be ignored by consumers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_body: Option<ReferenceOr<RequestBody>>,
    /// REQUIRED. The list of possible responses as they are returned
    /// from executing this operation.
    pub responses: Responses,
}