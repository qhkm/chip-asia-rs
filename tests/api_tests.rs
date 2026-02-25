use chip_sdk::model::*;
use chip_sdk::{ChipClient, PaymentMethodsOptions};

fn test_client(base_url: &str) -> ChipClient {
    ChipClient::builder()
        .base_url(base_url)
        .bearer_token("test-token")
        .build()
        .unwrap()
}

#[tokio::test]
async fn test_payment_methods() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("GET", "/payment_methods/")
        .match_query(mockito::Matcher::AllOf(vec![
            mockito::Matcher::UrlEncoded("brand_id".into(), "brand-123".into()),
            mockito::Matcher::UrlEncoded("currency".into(), "MYR".into()),
        ]))
        .match_header("authorization", "Bearer test-token")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"available_payment_methods":["visa","mastercard"],"card_methods":["visa","mastercard"],"names":{"visa":"Visa","mastercard":"Mastercard"}}"#)
        .create_async().await;

    let client = test_client(&server.url());
    let result = client.payment_methods("brand-123", "MYR", None).await;
    assert!(result.is_ok());
    let methods = result.unwrap();
    assert_eq!(methods.available_payment_methods.as_ref().unwrap().len(), 2);
    mock.assert_async().await;
}

#[tokio::test]
async fn test_payment_methods_with_options() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("GET", "/payment_methods/")
        .match_query(mockito::Matcher::AllOf(vec![
            mockito::Matcher::UrlEncoded("brand_id".into(), "brand-123".into()),
            mockito::Matcher::UrlEncoded("currency".into(), "MYR".into()),
            mockito::Matcher::UrlEncoded("country".into(), "MY".into()),
            mockito::Matcher::UrlEncoded("recurring".into(), "true".into()),
        ]))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"available_payment_methods":["visa"]}"#)
        .create_async()
        .await;

    let client = test_client(&server.url());
    let opts = PaymentMethodsOptions {
        country: Some("MY".into()),
        recurring: Some(true),
        ..Default::default()
    };
    let result = client.payment_methods("brand-123", "MYR", Some(opts)).await;
    assert!(result.is_ok());
    mock.assert_async().await;
}

#[tokio::test]
async fn test_create_purchase() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("POST", "/purchases/")
        .match_header("authorization", "Bearer test-token")
        .match_header("content-type", "application/json")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"id":"purchase-uuid","status":"created","checkout_url":"https://gate.chip-in.asia/checkout/purchase-uuid","brand_id":"brand-123","client":{"email":"test@test.com"},"purchase":{"products":[{"name":"Test","price":100}]}}"#)
        .create_async().await;

    let client = test_client(&server.url());
    let purchase = Purchase {
        brand_id: Some("brand-123".into()),
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
        ..Default::default()
    };
    let result = client.create_purchase(&purchase).await;
    assert!(result.is_ok());
    let created = result.unwrap();
    assert_eq!(created.id.as_deref(), Some("purchase-uuid"));
    assert_eq!(created.status, Some(PurchaseStatus::Created));
    assert!(created.checkout_url.is_some());
    mock.assert_async().await;
}

#[tokio::test]
async fn test_get_purchase() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("GET", "/purchases/abc-123/")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"id":"abc-123","status":"paid","brand_id":"brand-1"}"#)
        .create_async()
        .await;

    let client = test_client(&server.url());
    let result = client.get_purchase("abc-123").await;
    assert!(result.is_ok());
    let p = result.unwrap();
    assert_eq!(p.id.as_deref(), Some("abc-123"));
    assert_eq!(p.status, Some(PurchaseStatus::Paid));
    mock.assert_async().await;
}

#[tokio::test]
async fn test_cancel_purchase() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("POST", "/purchases/abc-123/cancel/")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"id":"abc-123","status":"cancelled"}"#)
        .create_async()
        .await;

    let client = test_client(&server.url());
    let result = client.cancel_purchase("abc-123").await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap().status, Some(PurchaseStatus::Cancelled));
    mock.assert_async().await;
}

#[tokio::test]
async fn test_capture_purchase_full() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("POST", "/purchases/abc-123/capture/")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"id":"abc-123","status":"paid"}"#)
        .create_async()
        .await;

    let client = test_client(&server.url());
    let result = client.capture_purchase("abc-123", None).await;
    assert!(result.is_ok());
    mock.assert_async().await;
}

#[tokio::test]
async fn test_capture_purchase_partial() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("POST", "/purchases/abc-123/capture/")
        .match_body(mockito::Matcher::Json(
            serde_json::json!({"amount": 5000.0}),
        ))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"id":"abc-123","status":"paid"}"#)
        .create_async()
        .await;

    let client = test_client(&server.url());
    let result = client.capture_purchase("abc-123", Some(5000.0)).await;
    assert!(result.is_ok());
    mock.assert_async().await;
}

#[tokio::test]
async fn test_charge_purchase() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("POST", "/purchases/abc-123/charge/")
        .match_body(mockito::Matcher::Json(
            serde_json::json!({"recurring_token": "token-uuid"}),
        ))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"id":"abc-123","status":"paid"}"#)
        .create_async()
        .await;

    let client = test_client(&server.url());
    let result = client.charge_purchase("abc-123", "token-uuid").await;
    assert!(result.is_ok());
    mock.assert_async().await;
}

#[tokio::test]
async fn test_refund_purchase() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("POST", "/purchases/abc-123/refund/")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"id":"abc-123","status":"refunded"}"#)
        .create_async()
        .await;

    let client = test_client(&server.url());
    let result = client.refund_purchase("abc-123", Some(1000.0)).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap().status, Some(PurchaseStatus::Refunded));
    mock.assert_async().await;
}

#[tokio::test]
async fn test_release_purchase() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("POST", "/purchases/abc-123/release/")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"id":"abc-123","status":"released"}"#)
        .create_async()
        .await;

    let client = test_client(&server.url());
    let result = client.release_purchase("abc-123").await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap().status, Some(PurchaseStatus::Released));
    mock.assert_async().await;
}

#[tokio::test]
async fn test_delete_recurring_token() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("POST", "/purchases/abc-123/delete_recurring_token/")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"id":"abc-123","is_recurring_token":false}"#)
        .create_async()
        .await;

    let client = test_client(&server.url());
    let result = client.delete_recurring_token("abc-123").await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap().is_recurring_token, Some(false));
    mock.assert_async().await;
}

#[tokio::test]
async fn test_api_error_response() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("GET", "/purchases/bad-id/")
        .with_status(404)
        .with_body("Not Found")
        .create_async()
        .await;

    let client = test_client(&server.url());
    let result = client.get_purchase("bad-id").await;
    assert!(result.is_err());
    let err = result.unwrap_err();
    match err {
        chip_sdk::ChipError::Api { status, message } => {
            assert_eq!(status, 404);
            assert_eq!(message, "Not Found");
        }
        _ => panic!("Expected Api error, got {:?}", err),
    }
    mock.assert_async().await;
}
