const res = await fetch(
  "https://46ki75.github.io/jp-holidays/api/v1/2026.json",
);
const holidays = await res.json();

// 日付（YYYY-MM-DD）から祝日名を引く
holidays["2026-01-01"]; // "元日"
holidays["2026-07-20"]; // "海の日"
holidays["2026-06-17"]; // undefined（平日）
