use jp_holidays_lib::{client::Client, error::Error};

// Client::init() は非同期に内閣府から祝日情報を取得するため、
// tokio::sync::OnceCell を使って初回のみ初期化し、その後はキャッシュを使用します。
static CACHE: tokio::sync::OnceCell<Client> = tokio::sync::OnceCell::const_new();

// キャッシュされた Client インスタンスを取得します
async fn get_client() -> Result<&'static Client, Error> {
    CACHE.get_or_try_init(Client::init).await
}

// 実行用の関数（main から呼び出し）
// スコープを抜けても Client はキャッシュされ続けます
async fn execute() -> Result<(), Box<dyn std::error::Error>> {
    let client = get_client().await?;

    // 祝日を取得
    let maybe_holiday = client.get_holiday_ymd(1955, 11, 23);
    match maybe_holiday? {
        Some(holiday) => println!("1955年 11月 23日 は{}", holiday),
        None => println!("1955年 11月 23日 は祝日ではありません"),
    };

    // 祝日かどうか確認
    let is_holiday = client.is_holiday_ymd(1956, 3, 21)?;
    println!(
        "1956 3月 21日 は{}",
        if is_holiday {
            "祝日です"
        } else {
            "祝日ではありません"
        }
    );

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    for i in 0..5 {
        let start = std::time::Instant::now();
        execute().await?;
        let duration = start.elapsed();
        println!("{}回目の実行時間: {:?}\n", i + 1, duration);
    }

    Ok(())
}
