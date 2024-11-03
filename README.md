# 日本の祝日 REST API

この API は、日本の祝日情報を静的にビルドした JSON 形式で提供する HTTP REST API です。

## エンドポイント

API のベースパスは、現在 `46ki75.github.io/jp-holidays/api/v1/` です。

### 特定日の祝日情報の取得

特定の日付が祝日かどうかを確認するには、例えば `46ki75.github.io/jp-holidays/api/v1/2024-11-03.json` にリクエストを送信します。以下のような JSON レスポンスが返されます。

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

> [!NOTE]
>
> `public` は祝日であるかどうか、`holiday` は休日（祝日を含む）であるかどうかを示しています。

### 特定月の祝日一覧の取得

特定の月に含まれる祝日一覧を取得するには、例えば `46ki75.github.io/jp-holidays/api/v1/2024-11.json` にリクエストを送信します。次のようなレスポンスが得られます。

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

### 各エンドポイントのリファレンス

以下の表は、エンドポイントとそのレスポンス内容の対応を示しています。

| エンドポイント    | レスポンス内容                                               |
| ----------------- | ------------------------------------------------------------ |
| `yyyy-MM-DD.json` | 特定日の祝日情報。祝日でない日付でもレスポンスが返されます。 |
| `yyyy-MM.json`    | 特定月の祝日リスト。平日は含まれません。                     |
| `yyyy.json`       | 特定年の祝日リスト。平日は含まれません。                     |
| `list.json`       | 現在公開されている祝日の全リスト。平日は含まれません。       |
