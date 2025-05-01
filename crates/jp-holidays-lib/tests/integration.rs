use jp_holidays_lib::client::Client;

#[tokio::test]
async fn test_get_holiday_known_date() {
    let client = Client::init().await.unwrap();
    let date = {
        cfg_if::cfg_if! {
            if #[cfg(feature = "chrono")] {
                chrono::NaiveDate::from_ymd_opt(1955, 1, 1).unwrap()
            } else if #[cfg(feature = "time")] {
                time::Date::from_calendar_date(1955, time::Month::January, 1).unwrap()
            }
        }
    };
    let holiday = client.get_holiday(date).unwrap();
    assert_eq!(holiday, "元日");
}

#[tokio::test]
async fn test_get_holiday_unknown_date() {
    let client = Client::init().await.unwrap();
    let date = {
        cfg_if::cfg_if! {
            if #[cfg(feature = "chrono")] {
                chrono::NaiveDate::from_ymd_opt(1955, 1, 2).unwrap()
            } else if #[cfg(feature = "time")] {
                time::Date::from_calendar_date(1955, time::Month::January, 2).unwrap()
            }
        }
    };
    let holiday = client.get_holiday(date);
    assert_eq!(holiday, None);
}

#[tokio::test]
async fn test_is_holiday_true() {
    let client = Client::init().await.unwrap();
    let date = {
        cfg_if::cfg_if! {
            if #[cfg(feature = "chrono")] {
                chrono::NaiveDate::from_ymd_opt(1955, 5, 5).unwrap()
            } else if #[cfg(feature = "time")] {
                time::Date::from_calendar_date(1955, time::Month::May, 5).unwrap()
            }
        }
    };
    let is_holiday = client.is_holiday(date);
    assert!(is_holiday);
}

#[tokio::test]
async fn test_is_holiday_false() {
    let client = Client::init().await.unwrap();
    let date = {
        cfg_if::cfg_if! {
            if #[cfg(feature = "chrono")] {
                chrono::NaiveDate::from_ymd_opt(1955, 5, 4).unwrap()
            } else if #[cfg(feature = "time")] {
                time::Date::from_calendar_date(1955, time::Month::May, 4).unwrap()
            }
        }
    };
    let is_holiday = client.is_holiday(date);
    assert!(!is_holiday);
}

#[tokio::test]
async fn test_is_day_off_holiday() {
    let client = Client::init().await.unwrap();
    let date = {
        cfg_if::cfg_if! {
            if #[cfg(feature = "chrono")] {
                chrono::NaiveDate::from_ymd_opt(1955, 1, 1).unwrap()
            } else if #[cfg(feature = "time")] {
                time::Date::from_calendar_date(1955, time::Month::January, 1).unwrap()
            }
        }
    };
    let is_day_off = client.is_day_off(date);
    assert!(is_day_off);
}

#[tokio::test]
async fn test_is_day_off_weekend() {
    let client = Client::init().await.unwrap();
    let date = {
        cfg_if::cfg_if! {
            if #[cfg(feature = "chrono")] {
                chrono::NaiveDate::from_ymd_opt(1955, 1, 8).unwrap()
            } else if #[cfg(feature = "time")] {
                time::Date::from_calendar_date(1955, time::Month::January, 8).unwrap()
            }
        }
    };
    let is_day_off = client.is_day_off(date);
    assert!(is_day_off);
}

#[tokio::test]
async fn test_is_day_off_weekday_non_holiday() {
    let client = Client::init().await.unwrap();
    let date = {
        cfg_if::cfg_if! {
            if #[cfg(feature = "chrono")] {
                chrono::NaiveDate::from_ymd_opt(1955, 1, 5).unwrap()
            } else if #[cfg(feature = "time")] {
                time::Date::from_calendar_date(1955, time::Month::January, 5).unwrap()
            }
        }
    };
    let is_day_off = client.is_day_off(date);
    assert!(!is_day_off);
}
