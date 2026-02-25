use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PurchaseStatus {
    Created,
    Sent,
    Viewed,
    Error,
    Cancelled,
    Overdue,
    Expired,
    Blocked,
    Hold,
    Released,
    Preauthorized,
    Paid,
    Cleared,
    Settled,
    Chargeback,
    Refunded,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RefundAvailability {
    All,
    FullOnly,
    PartialOnly,
    None,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Platform {
    Web,
    Api,
    Ios,
    Android,
    Macos,
    Windows,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProductType {
    Purchases,
    BillingInvoices,
    BillingSubscriptions,
    BillingSubscriptionsInvoice,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentType {
    Purchase,
    PurchaseCharge,
    Payout,
    BankPayment,
    Refund,
    Custom,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn purchase_status_serde_roundtrip() {
        let statuses = vec![
            (PurchaseStatus::Created, "\"created\""),
            (PurchaseStatus::Paid, "\"paid\""),
            (PurchaseStatus::Cancelled, "\"cancelled\""),
            (PurchaseStatus::Hold, "\"hold\""),
            (PurchaseStatus::Preauthorized, "\"preauthorized\""),
            (PurchaseStatus::Refunded, "\"refunded\""),
            (PurchaseStatus::Chargeback, "\"chargeback\""),
        ];
        for (status, expected_json) in statuses {
            let json = serde_json::to_string(&status).unwrap();
            assert_eq!(json, expected_json);
            let deserialized: PurchaseStatus = serde_json::from_str(&json).unwrap();
            assert_eq!(deserialized, status);
        }
    }

    #[test]
    fn refund_availability_serde() {
        let json = "\"full_only\"";
        let ra: RefundAvailability = serde_json::from_str(json).unwrap();
        assert_eq!(ra, RefundAvailability::FullOnly);
    }

    #[test]
    fn platform_serde() {
        let json = "\"ios\"";
        let p: Platform = serde_json::from_str(json).unwrap();
        assert_eq!(p, Platform::Ios);
    }

    #[test]
    fn product_type_serde() {
        let json = "\"billing_invoices\"";
        let pt: ProductType = serde_json::from_str(json).unwrap();
        assert_eq!(pt, ProductType::BillingInvoices);
    }

    #[test]
    fn payment_type_serde() {
        let json = "\"purchase_charge\"";
        let pt: PaymentType = serde_json::from_str(json).unwrap();
        assert_eq!(pt, PaymentType::PurchaseCharge);
    }
}
