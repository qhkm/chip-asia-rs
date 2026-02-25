use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ClientDetails {
    pub email: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub personal_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub street_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zip_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_street_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_zip_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cc: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bcc: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legal_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brand_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registration_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_account: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_code: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_client_minimal() {
        let c = ClientDetails {
            email: "test@example.com".into(),
            ..Default::default()
        };
        let json = serde_json::to_value(&c).unwrap();
        assert_eq!(json["email"], "test@example.com");
        assert!(json.get("phone").is_none());
    }

    #[test]
    fn deserialize_client_full() {
        let json = r#"{"email":"buyer@test.com","phone":"+60 123456789","full_name":"John Doe","country":"MY","bank_account":"1234567890","bank_code":"MBBEMYKL"}"#;
        let c: ClientDetails = serde_json::from_str(json).unwrap();
        assert_eq!(c.email, "buyer@test.com");
        assert_eq!(c.phone.as_deref(), Some("+60 123456789"));
        assert_eq!(c.bank_account.as_deref(), Some("1234567890"));
    }
}
