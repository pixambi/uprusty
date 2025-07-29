use crate::models::webhooks::{WebhookEventResource, WebhookEventType};
use std::error::Error;
use std::fmt;

pub mod verification {
    use super::*;

    #[derive(Debug)]
    pub enum VerificationError {
        InvalidSignature,
        MissingSignature,
        InvalidHex,
        SignatureMismatch,
    }

    impl fmt::Display for VerificationError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                VerificationError::InvalidSignature => write!(f, "Invalid signature format"),
                VerificationError::MissingSignature => {
                    write!(f, "Missing X-Up-Authenticity-Signature header")
                }
                VerificationError::InvalidHex => write!(f, "Invalid hexadecimal in signature"),
                VerificationError::SignatureMismatch => write!(f, "Signature verification failed"),
            }
        }
    }

    impl Error for VerificationError {}

    pub fn verify_signature(
        secret_key: &str,
        signature_header: &str,
        raw_body: &[u8],
    ) -> Result<bool, VerificationError> {
        use hmac::{Hmac, Mac};
        use sha2::Sha256;

        let received_signature =
            hex::decode(signature_header).map_err(|_| VerificationError::InvalidHex)?;

        type HmacSha256 = Hmac<Sha256>;
        let mut mac = HmacSha256::new_from_slice(secret_key.as_bytes())
            .map_err(|_| VerificationError::InvalidSignature)?;

        mac.update(raw_body);

        let computed_signature = mac.finalize().into_bytes();

        Ok(constant_time_eq(
            &computed_signature[..],
            &received_signature,
        ))
    }

    fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
        if a.len() != b.len() {
            return false;
        }

        let mut result = 0u8;
        for (x, y) in a.iter().zip(b.iter()) {
            result |= x ^ y;
        }
        result == 0
    }

    pub fn extract_signature_from_headers<I, K, V>(headers: I) -> Result<String, VerificationError>
    where
        I: IntoIterator<Item = (K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        for (key, value) in headers {
            if key.as_ref().to_lowercase() == "x-up-authenticity-signature" {
                return Ok(value.as_ref().to_string());
            }
        }
        Err(VerificationError::MissingSignature)
    }
}

pub mod events {
    use super::*;

    pub fn parse_and_verify_event(
        secret_key: &str,
        signature_header: &str,
        raw_body: &str,
    ) -> Result<WebhookEventResource, WebhookProcessingError> {
        verification::verify_signature(secret_key, signature_header, raw_body.as_bytes())
            .map_err(WebhookProcessingError::Verification)?;
        let parsed: serde_json::Value =
            serde_json::from_str(raw_body).map_err(WebhookProcessingError::JsonParsing)?;
        let event: WebhookEventResource = serde_json::from_value(parsed["data"].clone())
            .map_err(WebhookProcessingError::EventParsing)?;

        Ok(event)
    }

    pub fn is_event_type(event: &WebhookEventResource, event_type: WebhookEventType) -> bool {
        event.attributes.event_type == event_type
    }

    pub fn extract_transaction_id(event: &WebhookEventResource) -> Option<&str> {
        event
            .relationships
            .transaction
            .as_ref()
            .map(|t| t.data.id.as_str())
    }

    pub fn extract_webhook_id(event: &WebhookEventResource) -> &str {
        &event.relationships.webhook.data.id
    }

    #[derive(Debug)]
    pub enum WebhookProcessingError {
        Verification(verification::VerificationError),
        JsonParsing(serde_json::Error),
        EventParsing(serde_json::Error),
    }

    impl fmt::Display for WebhookProcessingError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                WebhookProcessingError::Verification(e) => write!(f, "Verification error: {}", e),
                WebhookProcessingError::JsonParsing(e) => write!(f, "JSON parsing error: {}", e),
                WebhookProcessingError::EventParsing(e) => write!(f, "Event parsing error: {}", e),
            }
        }
    }

    impl Error for WebhookProcessingError {}
}

pub struct WebhookHandler {
    secret_key: String,
}

impl WebhookHandler {
    pub fn new(secret_key: &str) -> Self {
        Self {
            secret_key: secret_key.to_string(),
        }
    }

    pub fn process_request(
        &self,
        signature_header: &str,
        raw_body: &str,
    ) -> Result<WebhookEventResource, events::WebhookProcessingError> {
        events::parse_and_verify_event(&self.secret_key, signature_header, raw_body)
    }

    pub fn handle_event<F>(
        &self,
        signature_header: &str,
        raw_body: &str,
        mut handler: F,
    ) -> Result<(), events::WebhookProcessingError>
    where
        F: FnMut(&WebhookEventResource),
    {
        let event = self.process_request(signature_header, raw_body)?;
        handler(&event);
        Ok(())
    }

    pub fn handle_typed_event<F>(
        &self,
        signature_header: &str,
        raw_body: &str,
        handler: F,
    ) -> Result<(), events::WebhookProcessingError>
    where
        F: WebhookEventHandler,
    {
        let event = self.process_request(signature_header, raw_body)?;

        match event.attributes.event_type {
            WebhookEventType::TransactionCreated => {
                if let Some(transaction_id) = events::extract_transaction_id(&event) {
                    handler.on_transaction_created(transaction_id, &event);
                }
            }
            WebhookEventType::TransactionSettled => {
                if let Some(transaction_id) = events::extract_transaction_id(&event) {
                    handler.on_transaction_settled(transaction_id, &event);
                }
            }
            WebhookEventType::TransactionDeleted => {
                if let Some(transaction_id) = events::extract_transaction_id(&event) {
                    handler.on_transaction_deleted(transaction_id, &event);
                }
            }
            WebhookEventType::Ping => {
                handler.on_ping(&event);
            }
        }

        Ok(())
    }
}

pub trait WebhookEventHandler {
    fn on_transaction_created(&self, transaction_id: &str, event: &WebhookEventResource) {
        let _ = (transaction_id, event);
    }

    fn on_transaction_settled(&self, transaction_id: &str, event: &WebhookEventResource) {
        let _ = (transaction_id, event);
    }

    fn on_transaction_deleted(&self, transaction_id: &str, event: &WebhookEventResource) {
        let _ = (transaction_id, event);
    }

    fn on_ping(&self, event: &WebhookEventResource) {
        let _ = event;
    }
}

pub use events::{WebhookProcessingError, parse_and_verify_event};
pub use verification::{VerificationError, verify_signature};
