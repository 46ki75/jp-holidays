# 日本の祝日 HTTP STATIC API

[![Build and Deploy (v1)](https://github.com/46ki75/jp-holidays/actions/workflows/build-deploy-v1.yml/badge.svg)](https://github.com/46ki75/jp-holidays/actions/workflows/build-deploy-v1.yml) [![Unit Test](https://github.com/46ki75/jp-holidays/actions/workflows/unit-tests.yml/badge.svg)](https://github.com/46ki75/jp-holidays/actions/workflows/unit-tests.yml) ![現在公表されている最後の祝日](https://shields.io/badge/dynamic/json?url=https://46ki75.github.io/jp-holidays/api/v1/list.json&query=$.results[-1:].date&label=現在公表されている最後の祝日)

この API は、日本の祝日情報を静的にビルドした JSON 形式で提供する HTTP ベースの API です。

## エンドポイント

API のベースパスは `https://46ki75.github.io/jp-holidays/api/v1/` です。また、すべてのエンドポイントは静的ファイルで構成されるため、リクエストメソッドはすべて `GET` です。

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
> `public` は祝日であるかどうか、`holiday` は休日（祝日を含む）であるかどうかを示します。

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

以下は、各エンドポイントとそのレスポンス内容の対応表です。

| エンドポイント    | レスポンス内容                                               |
| ----------------- | ------------------------------------------------------------ |
| `yyyy-MM-DD.json` | 特定日の祝日情報。祝日でない日付でもレスポンスが返されます。 |
| `yyyy-MM.json`    | 特定月の祝日一覧。平日は含まれません。                       |
| `yyyy.json`       | 特定年の祝日一覧。平日は含まれません。                       |
| `list.json`       | 現在公開されている全ての祝日一覧。平日は含まれません。       |

## API の詳細

本 API は、[内閣府](https://www8.cao.go.jp/chosei/shukujitsu/gaiyou.html) の提供する CSV ファイルを元に、祝日情報を提供しています。データソースのスキーマやエンコード、パスが変更される場合があるため、古いデータが一時的に提供される場合があります。

### 静的ビルド

本 API は、内閣府提供の CSV データを取得し、JSON 形式に変換した後、静的ファイルとして GitHub Pages にデプロイしています。GitHub Pages の静的ホスティングにより、レスポンスの `content-type` はファイルの拡張子に基づいて設定されます（[対応表](https://www.iana.org/assignments/media-types/media-types.xhtml) 参照）。各エンドポイントには `.json` 拡張子が付いていますのでご注意ください。

### 静的ファイルの更新頻度

静的ファイルは GitHub Actions によって日次で更新されます。また、新しいバージョンがリリースされた際にもパイプラインがトリガーされ、デプロイが実行されます。

## API の利用について

本 API は、個人のワークフローの自動化や RPA の補助を主な用途として提供されています。商用環境での利用は想定していないため、商用利用を検討される場合は、SLA が明示されている祝日 API サービスの利用を推奨します。

更新履歴については、[リリースノート](https://github.com/46ki75/jp-holidays/releases) をご確認ください。
