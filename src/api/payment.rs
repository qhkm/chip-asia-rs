use crate::client::ChipClient;
use crate::error::ChipError;
use crate::model::{PaymentMethods, Purchase};

#[derive(Debug, Default)]
pub struct PaymentMethodsOptions {
    pub country: Option<String>,
    pub recurring: Option<bool>,
    pub skip_capture: Option<bool>,
    pub preauthorization: Option<bool>,
}

impl ChipClient {
    pub async fn payment_methods(
        &self,
        brand_id: &str,
        currency: &str,
        opts: Option<PaymentMethodsOptions>,
    ) -> Result<PaymentMethods, ChipError> {
        let url = format!("{}/payment_methods/", self.base_url);
        let mut request = self
            .http
            .get(&url)
            .query(&[("brand_id", brand_id), ("currency", currency)]);
        if let Some(opts) = opts {
            if let Some(country) = &opts.country {
                request = request.query(&[("country", country.as_str())]);
            }
            if let Some(true) = opts.recurring {
                request = request.query(&[("recurring", "true")]);
            }
            if let Some(true) = opts.skip_capture {
                request = request.query(&[("skip_capture", "true")]);
            }
            if let Some(true) = opts.preauthorization {
                request = request.query(&[("preauthorization", "true")]);
            }
        }
        let response = request.send().await?;
        handle_response(response).await
    }

    pub async fn create_purchase(&self, purchase: &Purchase) -> Result<Purchase, ChipError> {
        let url = format!("{}/purchases/", self.base_url);
        let response = self.http.post(&url).json(purchase).send().await?;
        handle_response(response).await
    }

    pub async fn get_purchase(&self, id: &str) -> Result<Purchase, ChipError> {
        let url = format!("{}/purchases/{}/", self.base_url, id);
        let response = self.http.get(&url).send().await?;
        handle_response(response).await
    }

    pub async fn cancel_purchase(&self, id: &str) -> Result<Purchase, ChipError> {
        let url = format!("{}/purchases/{}/cancel/", self.base_url, id);
        let response = self.http.post(&url).send().await?;
        handle_response(response).await
    }

    pub async fn capture_purchase(
        &self,
        id: &str,
        amount: Option<f64>,
    ) -> Result<Purchase, ChipError> {
        let url = format!("{}/purchases/{}/capture/", self.base_url, id);
        let body = match amount {
            Some(amt) => serde_json::json!({ "amount": amt }),
            None => serde_json::json!({}),
        };
        let response = self.http.post(&url).json(&body).send().await?;
        handle_response(response).await
    }

    pub async fn charge_purchase(
        &self,
        id: &str,
        recurring_token: &str,
    ) -> Result<Purchase, ChipError> {
        let url = format!("{}/purchases/{}/charge/", self.base_url, id);
        let body = serde_json::json!({ "recurring_token": recurring_token });
        let response = self.http.post(&url).json(&body).send().await?;
        handle_response(response).await
    }

    pub async fn refund_purchase(
        &self,
        id: &str,
        amount: Option<f64>,
    ) -> Result<Purchase, ChipError> {
        let url = format!("{}/purchases/{}/refund/", self.base_url, id);
        let body = match amount {
            Some(amt) => serde_json::json!({ "amount": amt }),
            None => serde_json::json!({}),
        };
        let response = self.http.post(&url).json(&body).send().await?;
        handle_response(response).await
    }

    pub async fn release_purchase(&self, id: &str) -> Result<Purchase, ChipError> {
        let url = format!("{}/purchases/{}/release/", self.base_url, id);
        let response = self.http.post(&url).send().await?;
        handle_response(response).await
    }

    pub async fn delete_recurring_token(&self, id: &str) -> Result<Purchase, ChipError> {
        let url = format!("{}/purchases/{}/delete_recurring_token/", self.base_url, id);
        let response = self.http.post(&url).send().await?;
        handle_response(response).await
    }
}

async fn handle_response<T: serde::de::DeserializeOwned>(
    response: reqwest::Response,
) -> Result<T, ChipError> {
    let status = response.status();
    if status.is_success() {
        let data = response.json::<T>().await?;
        Ok(data)
    } else {
        let message = response
            .text()
            .await
            .unwrap_or_else(|_| "Unknown error".to_string());
        Err(ChipError::Api {
            status: status.as_u16(),
            message,
        })
    }
}
