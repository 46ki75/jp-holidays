# 日本の祝日 REST API

日本の祝日情報を、静的ビルドした JSON 形式で提供する HTTP REST API です。

## エンドポイント

現行バージョンでは `46ki75.github.io/jp-holidays/api/v1/` をベースパスとしています。

### 特定日の祝日情報の取得

特定の日付に祝日があるか確認するには、例えば `46ki75.github.io/jp-holidays/api/v1/2024-11-03.json` にリクエストを送信します。以下のような JSON レスポンスが返されます。

```json
{
  "name": "文化の日",
  "date": "2024-11-03",
  "year": 2024,
  "month": 11,
  "day": 3,
  "day_of_week": "Sunday",
  "day_of_week_ja": "日",
  "public": true,
  "holiday": true
}
```

### 特定月の祝日一覧の取得

特定の月にある祝日の一覧を取得するには、例えば `46ki75.github.io/jp-holidays/api/v1/2024-11.json` にリクエストを送信します。次のようなレスポンスが得られます。

```json
{
  "results": [
    {
      "name": "文化の日",
      "date": "2024-11-03",
      "year": 2024,
      "month": 11,
      "day": 3,
      "day_of_week": "Sunday",
      "day_of_week_ja": "日",
      "public": true,
      "holiday": true
    },
    {
      "name": "休日",
      "date": "2024-11-04",
      "year": 2024,
      "month": 11,
      "day": 4,
      "day_of_week": "Monday",
      "day_of_week_ja": "月",
      "public": true,
      "holiday": true
    },
    {
      "name": "勤労感謝の日",
      "date": "2024-11-23",
      "year": 2024,
      "month": 11,
      "day": 23,
      "day_of_week": "Saturday",
      "day_of_week_ja": "土",
      "public": true,
      "holiday": true
    }
  ]
}
```
