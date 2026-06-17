# jp-holidays-lib

内閣府の公開する [「国民の祝日」について](https://www8.cao.go.jp/chosei/shukujitsu/gaiyou.html) より
祝日判定機能を提供するクレートです。

祝日データはビルド時にクレートへ同梱されており、**ネットワークも非同期ランタイムも不要**で利用できます。
常に最新のデータが必要な場合は、`fetch` フィーチャを有効にして実行時に内閣府から取得することもできます。

## インストール

以下のコマンドでクレートを追加してください。

```bash
cargo add jp-holidays-lib
```

日付操作に [`chrono`](https://crates.io/crates/chrono) を使用しているため、こちらも追加してください。

```bash
cargo add chrono
```

実行時に最新データを取得したい場合は `fetch` フィーチャを有効にします（`reqwest` と非同期ランタイムが必要です）。

```bash
cargo add jp-holidays-lib --features fetch
cargo add tokio --features=full
```

使用例のコードがリポジトリの `examples/` 以下に格納されています。

## 使用方法

同梱データからクライアントを作成します（同期・ネットワーク不要）。

```rs
let client = jp_holidays_lib::Client::new();
```

その後は以下の例のように使用してください。
Rust の日付操作のデファクトスタンダードである chrono をベースに API が提供されています。

```rs
use chrono::NaiveDate;
use jp_holidays_lib::Client;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();

    // 祝日を取得
    let date = NaiveDate::from_ymd_opt(1955, 11, 23).ok_or("存在しない日付です".to_string())?;

    let maybe_holiday = client.get_holiday(date);

    match maybe_holiday {
        Some(holiday) => println!("1955年 11月 23日 は{}", holiday),
        None => println!("1955年 11月 23日 は祝日ではありません"),
    };

    // 祝日かどうか確認
    let date = NaiveDate::from_ymd_opt(1956, 3, 21).ok_or("存在しない日付です".to_string())?;

    let is_holiday = client.is_holiday(date);

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
```

### 関連関数

- `Client::new()`: 同梱データからクライアントを作成します（同期）。
- `Client::from_csv()`: 内閣府形式の UTF-8 CSV 文字列からクライアントを作成します。
- `Client::fetch()`: *(`fetch` フィーチャ)* 内閣府から最新データを取得します（非同期）。

### メソッド

- `get_holiday()`: `chrono::NaiveDate` を渡して祝日を取得します。
- `get_holiday_ymd()`: 年月日を渡して祝日を取得します。
- `is_holiday()`:  `chrono::NaiveDate` を渡して祝日かどうかを判定します。
- `is_holiday_ymd()`:  年月日を渡して祝日かどうかを判定します。
- `is_day_off()`: `chrono::NaiveDate` を渡して休日かどうかを判定します。
- `is_day_off_ymd()`: 年月日を渡して休日かどうかを判定します。
- `list_holidays()`: 公開されている祝日をすべて取得します (`BTreeMap<NaiveDate, String>`)

## 最新データの取得（`fetch` フィーチャ）

`fetch` フィーチャを有効にすると、実行時に内閣府から最新の祝日データを取得できます。
取得はネットワークアクセスを伴うため、`tokio::sync::OnceCell` などでキャッシュすると効率的です。

```rs
use chrono::NaiveDate;
use jp_holidays_lib::{Client, Error};

// Client::fetch() は非同期に内閣府から祝日情報を取得するため、
// tokio::sync::OnceCell を使って初回のみ取得し、その後はキャッシュを使用します。
static CACHE: tokio::sync::OnceCell<Client> = tokio::sync::OnceCell::const_new();

async fn get_client() -> Result<&'static Client, Error> {
    CACHE.get_or_try_init(Client::fetch).await
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = get_client().await?;

    let date = NaiveDate::from_ymd_opt(1956, 3, 21).ok_or("存在しない日付です".to_string())?;
    println!("祝日かどうか: {}", client.is_holiday(date));

    Ok(())
}
```

## v1 からの移行

v2.0.0 では祝日データを同梱する方式に変更し、いくつかの破壊的変更があります。

- `Client::init().await?` は廃止されました。
  - 既定では同期の `Client::new()` を使用してください（同梱データ・ネットワーク不要）。
  - 実行時取得が必要な場合は `fetch` フィーチャを有効にし、`Client::fetch().await?` を使用してください。
- ルートでの再エクスポートを追加しました。`jp_holidays_lib::client::Client` の代わりに
  `jp_holidays_lib::Client` を利用できます。
- `Error` 型を見直し、メッセージを英語化するとともに、元のエラー（`reqwest`・`chrono`）を
  ラップするようにしました。
