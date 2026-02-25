use super::enums::PaymentType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PaymentDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_outgoing: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_type: Option<PaymentType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub net_amount: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee_amount: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_amount: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_unfreeze_on: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid_on: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_paid_on: Option<f64>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_payment_details() {
        let json = r#"{"is_outgoing":false,"payment_type":"purchase","amount":10000,"currency":"MYR","net_amount":9700,"fee_amount":300,"paid_on":1700000000}"#;
        let pd: PaymentDetails = serde_json::from_str(json).unwrap();
        assert_eq!(pd.is_outgoing, Some(false));
        assert_eq!(pd.payment_type, Some(PaymentType::Purchase));
        assert_eq!(pd.amount, Some(10000.0));
    }
}
