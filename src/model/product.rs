use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Product {
    pub name: String,
    pub price: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_percent: Option<f64>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_product_minimal() {
        let p = Product {
            name: "Test Item".into(),
            price: 100.0,
            ..Default::default()
        };
        let json = serde_json::to_value(&p).unwrap();
        assert_eq!(json["name"], "Test Item");
        assert_eq!(json["price"], 100.0);
        assert!(json.get("quantity").is_none());
    }

    #[test]
    fn deserialize_product_full() {
        let json = r#"{"name":"Widget","price":250,"quantity":3,"discount":10,"tax_percent":6}"#;
        let p: Product = serde_json::from_str(json).unwrap();
        assert_eq!(p.name, "Widget");
        assert_eq!(p.price, 250.0);
        assert_eq!(p.quantity, Some(3.0));
    }

    #[test]
    fn roundtrip_product() {
        let p = Product {
            name: "Item".into(),
            price: 50.0,
            quantity: Some(2.0),
            discount: None,
            tax_percent: Some(8.0),
        };
        let json = serde_json::to_string(&p).unwrap();
        let p2: Product = serde_json::from_str(&json).unwrap();
        assert_eq!(p.name, p2.name);
        assert_eq!(p.price, p2.price);
        assert_eq!(p.quantity, p2.quantity);
    }
}
