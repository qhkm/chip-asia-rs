use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BankAccount {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_account: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_code: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_bank_account() {
        let json = r#"{"bank_account":"DE89370400440532013000","bank_code":"COBADEFFXXX"}"#;
        let ba: BankAccount = serde_json::from_str(json).unwrap();
        assert_eq!(ba.bank_account.as_deref(), Some("DE89370400440532013000"));
        assert_eq!(ba.bank_code.as_deref(), Some("COBADEFFXXX"));
    }

    #[test]
    fn serialize_bank_account_empty() {
        let ba = BankAccount::default();
        let json = serde_json::to_string(&ba).unwrap();
        assert_eq!(json, "{}");
    }
}
