import requests

res = requests.get("https://46ki75.github.io/jp-holidays/api/v1/2026.json")
holidays = res.json()

# 日付（YYYY-MM-DD）から祝日名を引く
holidays["2026-01-01"]      # "元日"
holidays.get("2026-06-17")  # None（平日）
