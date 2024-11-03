# 日本の祝日 REST API

この API は、日本の祝日情報を静的にビルドした JSON 形式で提供する HTTP REST API です。

## エンドポイント

API のベースパスは `https://46ki75.github.io/jp-holidays/api/v1/` です。

### 特定日の祝日情報の取得

特定の日付が祝日かどうかを確認するには、`https://46ki75.github.io/jp-holidays/api/v1/2024-11-03.json` にリクエストを送信します。以下のような JSON レスポンスが返されます。

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

特定の月に含まれる祝日一覧を取得するには、`https://46ki75.github.io/jp-holidays/api/v1/2024-11.json` にリクエストを送信します。次のような JSON レスポンスが得られます。

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

### エンドポイントリファレンス

以下の表に、各エンドポイントとそのレスポンス内容をまとめています。

| エンドポイント    | レスポンス内容                                               |
| ----------------- | ------------------------------------------------------------ |
| `yyyy-MM-DD.json` | 特定日の祝日情報。祝日でない日付でもレスポンスが返されます。 |
| `yyyy-MM.json`    | 特定月の祝日一覧。平日は含まれません。                       |
| `yyyy.json`       | 特定年の祝日一覧。平日は含まれません。                       |
| `list.json`       | 現在公開されている全ての祝日一覧。平日は含まれません。       |

## REST API の詳細

本 API は、[内閣府](https://www8.cao.go.jp/chosei/shukujitsu/gaiyou.html) の提供する CSV ファイルに基づいて祝日情報を提供します。そのため、データソースのスキーマやエンコード、パスが変更された場合、しばらく古いデータが提供される場合があります。

### 静的ビルド

本 API は、データソースから取得した CSV をデシリアライズし、JSON 形式に変換した後、静的ファイルとして GitHub Pages にデプロイしています。GitHub Pages の静的ホスティングにより、レスポンスの `content-type` はファイル拡張子に基づいて設定されます（[対応表](https://www.iana.org/assignments/media-types/media-types.xhtml) 参照）。各エンドポイントには `.json` 拡張子がついていますのでご注意ください。

### 静的ファイルの更新頻度

静的ファイルは GitHub Actions により日次で更新されます。

## API の利用について

本 API は、主に個人のワークフローの自動化や RPA の補助を想定して提供されています。商用環境での利用は想定されておらず、商用環境での利用を検討される場合は SLA を明示している祝日 API サービスの利用を推奨します。
