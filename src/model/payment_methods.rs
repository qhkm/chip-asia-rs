use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PaymentMethods {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_payment_methods: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub by_country: Option<HashMap<String, Vec<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_names: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_methods: Option<Vec<String>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_payment_methods() {
        let json = r#"{"available_payment_methods":["visa","mastercard","fpx"],"by_country":{"MY":["visa","fpx"],"any":["mastercard"]},"country_names":{"MY":"Malaysia","any":"Other"},"names":{"visa":"Visa","mastercard":"Mastercard","fpx":"FPX"},"card_methods":["visa","mastercard"]}"#;
        let pm: PaymentMethods = serde_json::from_str(json).unwrap();
        assert_eq!(pm.available_payment_methods.as_ref().unwrap().len(), 3);
        assert_eq!(pm.by_country.as_ref().unwrap().get("MY").unwrap().len(), 2);
        assert_eq!(pm.names.as_ref().unwrap().get("visa").unwrap(), "Visa");
    }
}
