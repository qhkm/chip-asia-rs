use super::bank_account::BankAccount;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct IssuerDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legal_street_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legal_country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legal_city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legal_zip_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_accounts: Option<Vec<BankAccount>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legal_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brand_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registration_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_number: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_issuer_details() {
        let json = r#"{"website":"https://example.com","legal_name":"ACME Corp","legal_country":"MY","bank_accounts":[{"bank_account":"123456","bank_code":"MBBEMYKL"}]}"#;
        let id: IssuerDetails = serde_json::from_str(json).unwrap();
        assert_eq!(id.website.as_deref(), Some("https://example.com"));
        assert_eq!(id.bank_accounts.as_ref().unwrap().len(), 1);
    }
}
