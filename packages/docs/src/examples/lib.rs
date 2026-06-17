// jp-holidays-lib = "2"
use jp_holidays_lib::Client;

fn main() {
    // 祝日データを同梱。ネットワーク不要。
    let client = Client::new();

    // 日付から祝日名を引く（平日は None）
    assert_eq!(client.get_holiday_ymd(2026, 1, 1).unwrap(), Some("元日"));
    assert_eq!(client.get_holiday_ymd(2026, 6, 17).unwrap(), None);

    // 祝日・土日（休日）の判定
    assert!(client.is_day_off_ymd(2026, 1, 1).unwrap());
}
