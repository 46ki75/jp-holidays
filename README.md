# jp-holidays

[![codecov](https://codecov.io/gh/46ki75/jp-holidays/graph/badge.svg)](https://codecov.io/gh/46ki75/jp-holidays)

日本の祝日（内閣府公開データ）を扱うためのモノレポです。

| パッケージ                                     | 種別            | 内容                                                                            |
| ---------------------------------------------- | --------------- | ------------------------------------------------------------------------------- |
| [`jp-holidays-lib`](./crates/jp-holidays-lib/) | Rust ライブラリ | 祝日判定 API。データを同梱し、`fetch` フィーチャで最新取得も可能。              |
| [`jp-holidays`](./crates/jp-holidays/)         | Rust バイナリ   | 静的 JSON API・OpenAPI 仕様を生成するジェネレーター。                           |
| [`docs`](./packages/docs/)                     | React (Vite)    | トップページ。`@elmethis/react` 製。ビルド結果が GitHub Pages の `index.html`。 |

## 静的 HTTP API

`jp-holidays` ジェネレーターが内閣府データを取得し、静的 JSON として GitHub Pages に公開します。
認証不要・CORS 対応で、任意のクライアントから直接取得できます。トップページ（`index.html`）は
`docs` パッケージの React アプリで、同じ API を呼び出して「今日が祝日か」を表示します。

ベース URL: `https://46ki75.github.io/jp-holidays/`

| パス                    | 内容                             |
| ----------------------- | -------------------------------- |
| `/api/v1/holidays.json` | 全期間の祝日（日付キーのマップ） |
| `/api/v1/{year}.json`   | 指定した年の祝日                 |
| `/api/v1/years.json`    | 利用可能な年の一覧とメタデータ   |
| `/api/v1/openapi.json`  | OpenAPI 3.1 仕様                 |
| `/api/v1/docs.html`     | Scalar による API リファレンス   |

```js
const res = await fetch(
  "https://46ki75.github.io/jp-holidays/api/v1/2025.json",
);
const holidays = await res.json();
console.log(holidays["2025-01-01"]); // "元日"
```

## 開発

タスクは [`just`](https://github.com/casey/just) で実行します。

```bash
just            # レシピ一覧
just ci         # fmt-check + clippy + ハーメティックテスト（Rust）
just test-live  # ネットワークを伴う live テスト
just build-api  # JSON API のみを生成
just build-site # React docs + JSON API をまとめてビルド（packages/docs/dist）
```

`docs` の開発は `pnpm -C packages/docs dev`（Vite, base `/jp-holidays/`）。
