use actix_web::web::Json;
use assert_json_diff::assert_json_eq;
use netwopenapi::actix::{AcceptedJson, CreatedJson, NoContent};
use netwopenapi_core::PathItemDefinition;
use netwopenapi_gen::api_operation;
use schemars::_serde_json::json;

#[allow(clippy::todo)]
mod test_models {
  use actix_web::dev::Payload;
  use actix_web::http::StatusCode;
  use actix_web::{Error, FromRequest, HttpRequest, ResponseError};
  use netwopenapi_gen::{ApiComponent, ApiErrorComponent, ApiSecurity};
  use schemars::JsonSchema;
  use serde::{Deserialize, Serialize};
  use std::fmt::{Display, Formatter};
  use std::future::Ready;

  #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema, ApiComponent)]
  pub(crate) struct Test {
    pub(crate) test: String,
  }

  #[derive(Serialize, Deserialize, Debug, Clone, ApiErrorComponent)]
  #[openapi_error(status(code = 405, description = "Invalid input"))]
  pub(crate) enum ErrorResponse {
    MethodNotAllowed(String),
  }

  impl Display for ErrorResponse {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
      todo!()
    }
  }

  impl ResponseError for ErrorResponse {
    fn status_code(&self) -> StatusCode {
      todo!()
    }
  }

  #[derive(Serialize, Deserialize, Debug, Clone, ApiErrorComponent)]
  #[openapi_error(status(code = 401), status(code = 403), status(code = 404), status(code = 405))]
  pub(crate) enum MultipleErrorResponse {
    MethodNotAllowed(String),
  }

  impl Display for MultipleErrorResponse {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
      todo!()
    }
  }

  impl ResponseError for MultipleErrorResponse {
    fn status_code(&self) -> StatusCode {
      todo!()
    }
  }

  #[derive(ApiSecurity)]
  #[openapi_security(scheme(security_type(oauth2(flows(implicit(
    authorization_url = "https://authorize.com",
    refresh_url = "https://refresh.com",
    scopes(scope = "all:read", description = "Read all the things"),
    scopes(scope = "all:write", description = "Write all the things")
  ))))))]
  pub(crate) struct ApiKey;

  impl FromRequest for ApiKey {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(_req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
      todo!()
    }
  }
}

#[test]
#[allow(dead_code)]
fn api_operation() {
  /// Add a new pet to the store
  /// Add a new pet to the store
  /// Plop
  #[api_operation(tag = "pet")]
  pub(crate) async fn test(
    // Create a new pet in the store
    body: Json<test_models::Test>,
  ) -> Result<Json<test_models::Test>, test_models::ErrorResponse> {
    Ok(Json(body.0))
  }

  let components = __openapi_test::components();
  // only one component here because: error does not have schema and Test is used both for query and response
  assert_eq!(components.len(), 1);
  let components = serde_json::to_value(components).expect("Unable to serialize as Json");

  let operation = __openapi_test::operation();
  let operation = serde_json::to_value(operation).expect("Unable to serialize as Json");

  assert_json_eq!(
    components,
    json!([
      {
        "schemas": {
          "Test": {
            "properties": {
              "test": {
                "type": "string"
              }
            },
            "required": [
              "test"
            ],
            "title": "Test",
            "type": "object"
          }
        }
      }
    ])
  );
  assert_json_eq!(
    operation,
    json!({
      "deprecated": false,
      "description": "Add a new pet to the store\\\nPlop",
      "operationId": "test",
      "requestBody": {
        "content": {
          "application/json": {
            "schema": {
              "$ref": "#/components/schemas/Test"
            }
          }
        },
        "required": true
      },
      "responses": {
        "200": {
          "content": {
            "application/json": {
              "schema": {
                "$ref": "#/components/schemas/Test"
              }
            }
          },
          "description": ""
        },
        "405": {
          "description": "Invalid input"
        }
      },
      "summary": "Add a new pet to the store",
      "tags": [
        "pet"
      ]
    })
  );
}

#[test]
#[allow(dead_code)]
fn api_operation_no_content() {
  /// Add a new pet to the store
  /// Add a new pet to the store
  /// Plop
  #[api_operation(tag = "pet")]
  pub(crate) async fn test(
    // Create a new pet in the store
    _body: Json<test_models::Test>,
  ) -> Result<NoContent, test_models::ErrorResponse> {
    Ok(NoContent)
  }

  let components = __openapi_test::components();
  // only one component here because: error does not have schema and Test is used both for query and response
  assert_eq!(components.len(), 1);
  let components = serde_json::to_value(components).expect("Unable to serialize as Json");

  let operation = __openapi_test::operation();
  let operation = serde_json::to_value(operation).expect("Unable to serialize as Json");

  assert_json_eq!(
    components,
    json!([
      {
        "schemas": {
          "Test": {
            "properties": {
              "test": {
                "type": "string"
              }
            },
            "required": [
              "test"
            ],
            "title": "Test",
            "type": "object"
          }
        }
      }
    ])
  );
  assert_json_eq!(
    operation,
    json!({
      "deprecated": false,
      "description": "Add a new pet to the store\\\nPlop",
      "operationId": "test",
      "requestBody": {
        "content": {
          "application/json": {
            "schema": {
              "$ref": "#/components/schemas/Test"
            }
          }
        },
        "required": true
      },
      "responses": {
        "204": {
          "description": ""
        },
        "405": {
          "description": "Invalid input"
        }
      },
      "summary": "Add a new pet to the store",
      "tags": [
        "pet"
      ]
    })
  );
}

#[test]
#[allow(dead_code)]
fn api_operation_created_json() {
  /// Add a new pet to the store
  /// Add a new pet to the store
  /// Plop
  #[api_operation(tag = "pet")]
  pub(crate) async fn test(
    // Create a new pet in the store
    body: Json<test_models::Test>,
  ) -> Result<CreatedJson<test_models::Test>, test_models::ErrorResponse> {
    Ok(CreatedJson(body.0))
  }

  let components = __openapi_test::components();
  // only one component here because: error does not have schema and Test is used both for query and response
  assert_eq!(components.len(), 1);
  let components = serde_json::to_value(components).expect("Unable to serialize as Json");

  let operation = __openapi_test::operation();
  let operation = serde_json::to_value(operation).expect("Unable to serialize as Json");

  assert_json_eq!(
    components,
    json!([
      {
        "schemas": {
          "Test": {
            "properties": {
              "test": {
                "type": "string"
              }
            },
            "required": [
              "test"
            ],
            "title": "Test",
            "type": "object"
          }
        }
      }
    ])
  );
  assert_json_eq!(
    operation,
    json!({
      "deprecated": false,
      "description": "Add a new pet to the store\\\nPlop",
      "operationId": "test",
      "requestBody": {
        "content": {
          "application/json": {
            "schema": {
              "$ref": "#/components/schemas/Test"
            }
          }
        },
        "required": true
      },
      "responses": {
        "201": {
          "$ref": "#/components/schemas/Test"
        },
        "405": {
          "description": "Invalid input"
        }
      },
      "summary": "Add a new pet to the store",
      "tags": [
        "pet"
      ]
    })
  );
}

#[test]
#[allow(dead_code)]
fn api_operation_accepted_json() {
  /// Add a new pet to the store
  /// Add a new pet to the store
  /// Plop
  #[api_operation(tag = "pet")]
  pub(crate) async fn test(
    // Create a new pet in the store
    body: Json<test_models::Test>,
  ) -> Result<AcceptedJson<test_models::Test>, test_models::ErrorResponse> {
    Ok(AcceptedJson(body.0))
  }

  let components = __openapi_test::components();
  // only one component here because: error does not have schema and Test is used both for query and response
  assert_eq!(components.len(), 1);
  let components = serde_json::to_value(components).expect("Unable to serialize as Json");

  let operation = __openapi_test::operation();
  let operation = serde_json::to_value(operation).expect("Unable to serialize as Json");

  assert_json_eq!(
    components,
    json!([
      {
        "schemas": {
          "Test": {
            "properties": {
              "test": {
                "type": "string"
              }
            },
            "required": [
              "test"
            ],
            "title": "Test",
            "type": "object"
          }
        }
      }
    ])
  );
  assert_json_eq!(
    operation,
    json!({
      "deprecated": false,
      "description": "Add a new pet to the store\\\nPlop",
      "operationId": "test",
      "requestBody": {
        "content": {
          "application/json": {
            "schema": {
              "$ref": "#/components/schemas/Test"
            }
          }
        },
        "required": true
      },
      "responses": {
        "202": {
          "$ref": "#/components/schemas/Test"
        },
        "405": {
          "description": "Invalid input"
        }
      },
      "summary": "Add a new pet to the store",
      "tags": [
        "pet"
      ]
    })
  );
}

#[test]
#[allow(dead_code)]
fn api_operation_deprecated() {
  /// Add a new pet to the store
  /// Add a new pet to the store
  /// Plop
  #[api_operation(tag = "pet", deprecated)]
  pub(crate) async fn test(
    // Create a new pet in the store
    body: Json<test_models::Test>,
  ) -> Result<CreatedJson<test_models::Test>, test_models::ErrorResponse> {
    Ok(CreatedJson(body.0))
  }

  let components = __openapi_test::components();
  assert_eq!(components.len(), 1);
  let components = serde_json::to_value(components).expect("Unable to serialize as Json");

  let operation = __openapi_test::operation();
  let operation = serde_json::to_value(operation).expect("Unable to serialize as Json");

  assert_json_eq!(
    components,
    json!([
      {
        "schemas": {
          "Test": {
            "properties": {
              "test": {
                "type": "string"
              }
            },
            "required": [
              "test"
            ],
            "title": "Test",
            "type": "object"
          }
        }
      }
    ])
  );
  assert_json_eq!(
    operation,
    json!({
      "deprecated": true,
      "description": "Add a new pet to the store\\\nPlop",
      "operationId": "test",
      "requestBody": {
        "content": {
          "application/json": {
            "schema": {
              "$ref": "#/components/schemas/Test"
            }
          }
        },
        "required": true
      },
      "responses": {
        "201": {
          "$ref": "#/components/schemas/Test"
        },
        "405": {
          "description": "Invalid input"
        }
      },
      "summary": "Add a new pet to the store",
      "tags": [
        "pet"
      ]
    })
  );

  /// Add a new pet to the store
  /// Add a new pet to the store
  /// Plop
  #[api_operation(tag = "pet")]
  #[deprecated]
  pub(crate) async fn test2(
    // Create a new pet in the store
    body: Json<test_models::Test>,
  ) -> Result<CreatedJson<test_models::Test>, test_models::ErrorResponse> {
    Ok(CreatedJson(body.0))
  }

  let components = __openapi_test2::components();
  assert_eq!(components.len(), 1);
  let components = serde_json::to_value(components).expect("Unable to serialize as Json");

  let operation = __openapi_test2::operation();
  let operation = serde_json::to_value(operation).expect("Unable to serialize as Json");

  assert_json_eq!(
    components,
    json!([
      {
        "schemas": {
          "Test": {
            "properties": {
              "test": {
                "type": "string"
              }
            },
            "required": [
              "test"
            ],
            "title": "Test",
            "type": "object"
          }
        }
      }
    ])
  );
  assert_json_eq!(
    operation,
    json!({
      "deprecated": true,
      "description": "Add a new pet to the store\\\nPlop",
      "operationId": "test2",
      "requestBody": {
        "content": {
          "application/json": {
            "schema": {
              "$ref": "#/components/schemas/Test"
            }
          }
        },
        "required": true
      },
      "responses": {
        "201": {
          "$ref": "#/components/schemas/Test"
        },
        "405": {
          "description": "Invalid input"
        }
      },
      "summary": "Add a new pet to the store",
      "tags": [
        "pet"
      ]
    })
  );
}

#[test]
#[allow(dead_code)]
fn api_operation_skip() {
  /// Add a new pet to the store
  /// Add a new pet to the store
  /// Plop
  #[api_operation(tag = "pet", skip)]
  pub(crate) async fn test(
    // Create a new pet in the store
    body: Json<test_models::Test>,
  ) -> Result<CreatedJson<test_models::Test>, test_models::ErrorResponse> {
    Ok(CreatedJson(body.0))
  }

  let components = __openapi_test::components();
  assert!(components.is_empty());

  let operation = __openapi_test::operation();
  let operation = serde_json::to_value(operation).expect("Unable to serialize as Json");

  assert_json_eq!(
    operation,
    json!({
      "responses": {}
    })
  );
}

#[test]
#[allow(dead_code)]
fn api_operation_error() {
  /// Add a new pet to the store
  /// Add a new pet to the store
  /// Plop
  #[api_operation(tag = "pet", error_code = "404", error_code = "401")]
  pub(crate) async fn test(
    // Create a new pet in the store
    body: Json<test_models::Test>,
  ) -> Result<CreatedJson<test_models::Test>, test_models::MultipleErrorResponse> {
    Ok(CreatedJson(body.0))
  }

  let components = __openapi_test::components();
  assert_eq!(components.len(), 1);
  let components = serde_json::to_value(components).expect("Unable to serialize as Json");

  let operation = __openapi_test::operation();
  let operation = serde_json::to_value(operation).expect("Unable to serialize as Json");

  assert_json_eq!(
    components,
    json!([
      {
        "schemas": {
          "Test": {
            "properties": {
              "test": {
                "type": "string"
              }
            },
            "required": [
              "test"
            ],
            "title": "Test",
            "type": "object"
          }
        }
      }
    ])
  );
  assert_json_eq!(
    operation,
    json!({
      "deprecated": false,
      "description": "Add a new pet to the store\\\nPlop",
      "operationId": "test",
      "requestBody": {
        "content": {
          "application/json": {
            "schema": {
              "$ref": "#/components/schemas/Test"
            }
          }
        },
        "required": true
      },
      "responses": {
        "201": {
          "$ref": "#/components/schemas/Test"
        },
        "401": {
          "description": "Unauthorized"
        },
        "404": {
          "description": "Not Found"
        }
      },
      "summary": "Add a new pet to the store",
      "tags": [
        "pet"
      ]
    })
  );
}

#[test]
#[allow(dead_code)]
fn api_operation_security() {
  /// Add a new pet to the store
  /// Add a new pet to the store
  /// Plop
  #[api_operation(security_scope(name = "api_key", scope = "read:pets"))]
  pub(crate) async fn test(
    // Create a new pet in the store
    body: Json<test_models::Test>,
    _key: test_models::ApiKey,
  ) -> Result<CreatedJson<test_models::Test>, test_models::MultipleErrorResponse> {
    Ok(CreatedJson(body.0))
  }

  let components = __openapi_test::components();
  assert_eq!(components.len(), 1);
  let components = serde_json::to_value(components).expect("Unable to serialize as Json");

  let operation = __openapi_test::operation();
  let operation = serde_json::to_value(operation).expect("Unable to serialize as Json");

  assert_json_eq!(
    components,
    json!([
      {
        "schemas": {
          "Test": {
            "properties": {
              "test": {
                "type": "string"
              }
            },
            "required": [
              "test"
            ],
            "title": "Test",
            "type": "object"
          }
        },
        "securitySchemes": {
          "api_key": {
            "flows": {
              "implicit": {
                "authorizationUrl": "https://authorize.com",
                "refreshUrl": "https://refresh.com",
                "scopes": {
                  "all:read": "Read all the things",
                  "all:write": "Write all the things"
                }
              }
            },
            "type": "oauth2"
          }
        }
      }
    ])
  );
  assert_json_eq!(
    operation,
    json!({
      "deprecated": false,
      "description": "Add a new pet to the store\\\nPlop",
      "operationId": "test",
      "requestBody": {
        "content": {
          "application/json": {
            "schema": {
              "$ref": "#/components/schemas/Test"
            }
          }
        },
        "required": true
      },
      "responses": {
        "201": {
          "$ref": "#/components/schemas/Test"
        },
        "401": {
          "description": "Unauthorized"
        },
        "403": {
          "description": "Forbidden"
        },
        "404": {
          "description": "Not Found"
        },
        "405": {
          "description": "Method Not Allowed"
        }
      },
      "security": [
        {
          "api_key": [
            "read:pets"
          ]
        }
      ],
      "summary": "Add a new pet to the store"
    })
  );
}
