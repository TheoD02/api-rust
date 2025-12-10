use axum::{
    async_trait,
    extract::{rejection::JsonRejection, FromRequest, Request},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::de::DeserializeOwned;
use serde_json::json;
use thiserror::Error;
use validator::Validate;

// === Erreurs de validation (comme ConstraintViolationList en Symfony) ===

#[derive(Debug, Error)]
pub enum ValidationError {
    #[error("Validation failed")]
    ValidationFailed(#[from] validator::ValidationErrors),

    #[error("Invalid JSON")]
    JsonError(#[from] JsonRejection),
}

impl IntoResponse for ValidationError {
    fn into_response(self) -> Response {
        let (status, errors) = match self {
            // Erreurs de validation (comme les violations Symfony)
            ValidationError::ValidationFailed(validation_errors) => {
                let errors: Vec<_> = validation_errors
                    .field_errors()
                    .iter()
                    .map(|(field, errors)| {
                        let messages: Vec<_> = errors
                            .iter()
                            .filter_map(|e| e.message.as_ref().map(|m| m.to_string()))
                            .collect();
                        json!({
                            "field": field,
                            "messages": messages
                        })
                    })
                    .collect();

                (StatusCode::UNPROCESSABLE_ENTITY, errors)
            }
            // Erreurs JSON (malformed JSON)
            ValidationError::JsonError(err) => {
                let error = json!({
                    "field": "_json",
                    "messages": [err.to_string()]
                });
                (StatusCode::BAD_REQUEST, vec![error])
            }
        };

        let body = json!({
            "error": "Validation failed",
            "violations": errors
        });

        (status, Json(body)).into_response()
    }
}

// === ValidatedJson: Extracteur qui valide automatiquement ===
// Equivalent de: ParamConverter avec validation auto en Symfony

pub struct ValidatedJson<T>(pub T);

#[async_trait]
impl<S, T> FromRequest<S> for ValidatedJson<T>
where
    S: Send + Sync,
    T: DeserializeOwned + Validate,
    Json<T>: FromRequest<S, Rejection = JsonRejection>,
{
    type Rejection = ValidationError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        // 1. Parse le JSON
        let Json(data) = Json::<T>::from_request(req, state).await?;

        // 2. Valide les donnees (comme $validator->validate() en Symfony)
        data.validate()?;

        // 3. Retourne les donnees validees
        Ok(ValidatedJson(data))
    }
}
