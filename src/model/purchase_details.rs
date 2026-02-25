use super::product::Product;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PurchaseDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    pub products: Vec<Product>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub debt: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtotal_override: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_tax_override: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_discount_override: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_override: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_client_details: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_strict: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_message: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_purchase_details() {
        let pd = PurchaseDetails {
            currency: Some("MYR".into()),
            products: vec![Product {
                name: "Widget".into(),
                price: 100.0,
                ..Default::default()
            }],
            ..Default::default()
        };
        let json = serde_json::to_value(&pd).unwrap();
        assert_eq!(json["currency"], "MYR");
        assert_eq!(json["products"][0]["name"], "Widget");
        assert!(json.get("language").is_none());
    }

    #[test]
    fn deserialize_purchase_details_with_total() {
        let json =
            r#"{"currency":"EUR","products":[{"name":"A","price":50}],"total":50,"language":"en"}"#;
        let pd: PurchaseDetails = serde_json::from_str(json).unwrap();
        assert_eq!(pd.total, Some(50.0));
        assert_eq!(pd.language.as_deref(), Some("en"));
    }
}
