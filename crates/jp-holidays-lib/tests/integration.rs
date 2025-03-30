use jp_holidays_lib::client::Client;

#[tokio::test]
async fn test_get_holiday_known_date() {
    let client = Client::init().await.unwrap();
    let holiday = client.get_holiday(1955, 1, 1).unwrap();
    assert_eq!(holiday, Some("元日"));
}

#[tokio::test]
async fn test_get_holiday_unknown_date() {
    let client = Client::init().await.unwrap();
    let holiday = client.get_holiday(1955, 1, 2).unwrap();
    assert_eq!(holiday, None);
}

#[tokio::test]
async fn test_is_holiday_true() {
    let client = Client::init().await.unwrap();
    let is_holiday = client.is_holiday(1955, 5, 5).unwrap();
    assert!(is_holiday);
}

#[tokio::test]
async fn test_is_holiday_false() {
    let client = Client::init().await.unwrap();
    let is_holiday = client.is_holiday(1955, 5, 4).unwrap();
    assert!(!is_holiday);
}

#[tokio::test]
async fn test_invalid_date() {
    let client = Client::init().await.unwrap();
    let result = client.get_holiday(1955, 2, 30);
    assert!(result.is_err());
}

#[tokio::test]
async fn test_is_day_off_holiday() {
    let client = Client::init().await.unwrap();
    let is_day_off = client.is_day_off(1955, 1, 1).unwrap();
    assert!(is_day_off);
}

#[tokio::test]
async fn test_is_day_off_weekend() {
    let client = Client::init().await.unwrap();
    let is_day_off = client.is_day_off(1955, 1, 8).unwrap();
    assert!(is_day_off);
}

#[tokio::test]
async fn test_is_day_off_weekday_non_holiday() {
    let client = Client::init().await.unwrap();
    let is_day_off = client.is_day_off(1955, 1, 5).unwrap();
    assert!(!is_day_off);
}
