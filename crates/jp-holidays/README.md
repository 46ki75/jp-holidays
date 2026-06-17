# jp-holidays

内閣府の祝日データを取得し、静的 JSON API を生成するジェネレーターです。
生成物は GitHub Pages に公開され、静的 HTTP API として利用できます。

このクレートは publish されません（`publish = false`）。CI から実行されます。

## 使い方

```bash
cargo run -p jp-holidays -- --out-dir dist
```

実行時に [`jp-holidays-lib`](../jp-holidays-lib/) の `fetch` フィーチャ経由で
内閣府から最新データを取得します。

## 生成されるファイル

```text
dist/
  .nojekyll
  index.html
  api/v1/holidays.json   # { "1955-01-01": "元日", ... }  全期間
  api/v1/{year}.json     # 指定した年（同じ形式）
  api/v1/years.json      # 年の一覧 + メタデータ
  api/v1/openapi.json    # OpenAPI 3.1 仕様（utoipa で生成）
  api/v1/docs.html       # Scalar による API リファレンス
```

詳細はワークスペースの [README](../../README.md) を参照してください。
