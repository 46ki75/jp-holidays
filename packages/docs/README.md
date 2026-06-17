# docs

`jp-holidays` のトップページ（landing page）です。React + Vite で構築し、
UI は [`@elmethis/react`](https://www.npmjs.com/package/@elmethis/react) を使用しています。

ビルド結果が GitHub Pages の `index.html` になります。ページは公開中の API
（`/api/v1/holidays.json`）を実際に呼び出し、「今日が祝日かどうか」を表示します。
ネットワークが無い場合は同梱のフォールバックデータを表示します。

## 開発

```bash
pnpm install
pnpm dev        # http://localhost:5173/jp-holidays/
pnpm build      # 型チェック + 本番ビルド -> dist/
pnpm preview    # ビルド結果のプレビュー
```

GitHub Pages のプロジェクトパスに合わせて Vite の `base` を `/jp-holidays/`
に設定しています。API への参照は `import.meta.env.BASE_URL` 基準です。

## デプロイ

CI（`.github/workflows/lib-pages-deploy.yaml`）が `pnpm build` でこのアプリを生成し、
その出力ディレクトリへ `jp-holidays` ジェネレーターが JSON API をマージして公開します。
ローカルでは `just build-site`（リポジトリ root）で同じ成果物を再現できます。

## 構成

```text
src/
  main.tsx              # エントリ。elmethis の CSS とトークンを読み込む
  App.tsx               # ページ本体
  index.css / App.css   # ベース + レイアウト
  holidays/
    data.ts             # フォールバックの祝日データ + ふりがな
    logic.ts            # 祝日/休日/平日 判定・整形（純粋関数）
    use-holidays.ts     # 同梱データ即時表示 → ライブ API で上書き
```
