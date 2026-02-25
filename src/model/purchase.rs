use super::client_details::ClientDetails;
use super::enums::{Platform, ProductType, PurchaseStatus, RefundAvailability};
use super::issuer_details::IssuerDetails;
use super::payment_details::PaymentDetails;
use super::purchase_details::PurchaseDetails;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Purchase {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub object_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_on: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_on: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client: Option<ClientDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purchase: Option<PurchaseDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment: Option<PaymentDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer_details: Option<IssuerDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_data: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<PurchaseStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_history: Option<Vec<serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub viewed_on: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_test: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brand_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_template_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_receipt: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_recurring_token: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurring_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_capture: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_recurring: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_generated: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issued: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_availability: Option<RefundAvailability>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refundable_amount: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_conversion: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_whitelist: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success_redirect: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_redirect: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_redirect: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success_callback: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_agent: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<Platform>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<ProductType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_from_ip: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checkout_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_post_url: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::Product;

    #[test]
    fn serialize_purchase_for_create() {
        let p = Purchase {
            brand_id: Some("test-brand-id".into()),
            client: Some(ClientDetails {
                email: "test@test.com".into(),
                ..Default::default()
            }),
            purchase: Some(PurchaseDetails {
                products: vec![Product {
                    name: "Test".into(),
                    price: 100.0,
                    ..Default::default()
                }],
                ..Default::default()
            }),
            success_redirect: Some("https://example.com/success".into()),
            failure_redirect: Some("https://example.com/failure".into()),
            ..Default::default()
        };
        let json = serde_json::to_value(&p).unwrap();
        assert_eq!(json["brand_id"], "test-brand-id");
        assert_eq!(json["client"]["email"], "test@test.com");
        assert_eq!(json["purchase"]["products"][0]["name"], "Test");
        assert!(json.get("id").is_none());
        assert!(json.get("status").is_none());
    }

    #[test]
    fn deserialize_purchase_response() {
        let json = r#"{"id":"abc-123","type":"purchase","status":"created","brand_id":"brand-uuid","checkout_url":"https://gate.chip-in.asia/checkout/abc-123","is_test":true,"client":{"email":"test@test.com"},"purchase":{"products":[{"name":"Item","price":500}],"currency":"MYR"},"refund_availability":"all","platform":"api","created_on":1700000000}"#;
        let p: Purchase = serde_json::from_str(json).unwrap();
        assert_eq!(p.id.as_deref(), Some("abc-123"));
        assert_eq!(p.object_type.as_deref(), Some("purchase"));
        assert_eq!(p.status, Some(PurchaseStatus::Created));
        assert_eq!(
            p.checkout_url.as_deref(),
            Some("https://gate.chip-in.asia/checkout/abc-123")
        );
        assert_eq!(p.refund_availability, Some(RefundAvailability::All));
        assert_eq!(p.platform, Some(Platform::Api));
    }

    #[test]
    fn purchase_roundtrip() {
        let p = Purchase {
            brand_id: Some("brand-1".into()),
            status: Some(PurchaseStatus::Paid),
            ..Default::default()
        };
        let json = serde_json::to_string(&p).unwrap();
        let p2: Purchase = serde_json::from_str(&json).unwrap();
        assert_eq!(p.brand_id, p2.brand_id);
        assert_eq!(p.status, p2.status);
    }
}
