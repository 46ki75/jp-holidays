import { useState } from "react";
import {
  ElmCallout,
  ElmCodeBlock,
  ElmDivider,
  ElmHeading,
  ElmInlineText,
  ElmMdiIcon,
  ElmParagraph,
  ElmTab,
  ElmTabList,
  ElmTabPanel,
  ElmTabs,
  ElmTable,
  ElmTableBody,
  ElmTableCell,
  ElmTableHeader,
  ElmTableRow,
  ElmToggleTheme,
  ElmLanguageIcon,
} from "@elmethis/react";
import {
  mdiAlertOutline,
  mdiCheck,
  mdiChevronLeft,
  mdiChevronRight,
  mdiContentCopy,
  mdiEarth,
  mdiGithub,
  mdiOpenInNew,
} from "@mdi/js";
import { useHolidays } from "./holidays/use-holidays";
import { detectTimeInfo, type TimeInfo } from "./holidays/now";
import { READINGS } from "./holidays/data";
import {
  formatLong,
  formatShort,
  holidaysInYear,
  nextHoliday,
  statsFor,
  toKey,
  upcomingHolidays,
  verdictFor,
} from "./holidays/logic";
import fetchExample from "./examples/fetch.js?raw";
import curlExample from "./examples/curl.sh?raw";
import pythonExample from "./examples/requests.py?raw";
import reqwestExample from "./examples/reqwest.rs?raw";
import libExample from "./examples/lib.rs?raw";
import "./App.css";

const REPO = "https://github.com/46ki75/jp-holidays";
const BASE = "https://46ki75.github.io/jp-holidays";
const apiHref = `${import.meta.env.BASE_URL}api/v1`;

const ENDPOINTS = [
  { path: "/api/v1/holidays.json", desc: "全期間の祝日（日付キーのマップ）" },
  { path: "/api/v1/{year}.json", desc: "指定した年の祝日" },
  { path: "/api/v1/years.json", desc: "年の一覧とメタデータ" },
  { path: "/api/v1/openapi.json", desc: "OpenAPI 3.1 仕様" },
] as const;

// Examples live as standalone source files (real syntax highlighting in the
// editor) and are inlined at build time via Vite's `?raw` import. trimEnd drops
// each file's trailing newline so the code block renders flush.
const FETCH_EXAMPLE = fetchExample.trimEnd();
const CURL_EXAMPLE = curlExample.trimEnd();
const PYTHON_EXAMPLE = pythonExample.trimEnd();
const REQWEST_EXAMPLE = reqwestExample.trimEnd();
const LIB_EXAMPLE = libExample.trimEnd();

function CopyButton({ value, label }: { value: string; label: string }) {
  const [copied, setCopied] = useState(false);
  const copy = () => {
    navigator.clipboard?.writeText(value).then(() => {
      setCopied(true);
      window.setTimeout(() => setCopied(false), 1400);
    });
  };
  return (
    <button
      type="button"
      className="copy-btn"
      onClick={copy}
      aria-label={`${label} をコピー`}
      data-copied={copied}
    >
      <ElmMdiIcon d={copied ? mdiCheck : mdiContentCopy} size="1.05rem" />
    </button>
  );
}

function HolidayName({ name }: { name: string }) {
  const ruby = READINGS[name];
  return ruby ? (
    <ElmInlineText ruby={ruby}>{name}</ElmInlineText>
  ) : (
    <ElmInlineText>{name}</ElmInlineText>
  );
}

function TodayPanel({
  time,
  holidays,
  live,
}: {
  time: TimeInfo;
  holidays: Record<string, string>;
  live: boolean;
}) {
  const verdict = verdictFor(holidays, time.today);
  const next = nextHoliday(holidays, time.today);
  const label =
    verdict.kind === "holiday"
      ? "祝日"
      : verdict.kind === "weekend"
        ? "休日"
        : "平日";

  return (
    <div className={`today today--${verdict.kind}`}>
      <span className="eyebrow mono">TODAY · 今日</span>
      <p className="today-date mono">{formatLong(time.today)}</p>
      <p className="today-verdict display">{label}</p>
      {verdict.kind === "holiday" && verdict.name !== "休日" && (
        <p className="today-name display">
          「<HolidayName name={verdict.name} />」
        </p>
      )}
      {next && (
        <p className="today-next">
          次の祝日は <span className="mono">{formatShort(next.date)}</span>{" "}
          <HolidayName name={next.name} /> — あと {next.daysUntil} 日
        </p>
      )}
      {time.drift && (
        <p className="drift-note">
          <ElmMdiIcon d={mdiAlertOutline} size="0.95rem" />
          <span>
            お使いの地域では今日は {time.drift.localLabel}、日本では{" "}
            {time.drift.tokyoLabel}{" "}
            です。祝日は日本標準時（JST）で判定しています。
          </span>
        </p>
      )}
      <div className="today-meta">
        <span className="today-tz mono">
          <ElmMdiIcon d={mdiEarth} size="0.9rem" />
          {time.timeZone}
        </span>
        <span className="today-source" data-live={live}>
          {live ? "● 内閣府データに同期済み" : "○ 同梱データを表示中"}
        </span>
      </div>
    </div>
  );
}

export function YearPanel({
  holidays,
  today,
}: {
  holidays: Record<string, string>;
  today: Date;
}) {
  const { minYear, maxYear } = statsFor(holidays);
  const [year, setYear] = useState(() => today.getFullYear());

  // The dataset range can grow when the live data loads; keep the shown year
  // inside it so the stepper never points at an empty year.
  const shown = Math.min(Math.max(year, minYear), maxYear);
  const list = holidaysInYear(holidays, shown);
  const todayKey = toKey(today);
  const step = (delta: number) =>
    setYear((y) => Math.min(Math.max(y + delta, minYear), maxYear));

  return (
    <section className="section">
      <div>
        <span className="eyebrow mono">CALENDAR</span>
        <ElmHeading level={2} className="margin-zero">
          年間の祝日
        </ElmHeading>
      </div>

      <div className="year-nav" role="group" aria-label="表示する年">
        <button
          type="button"
          className="year-btn"
          onClick={() => step(-1)}
          disabled={shown <= minYear}
          aria-label="前の年"
        >
          <ElmMdiIcon d={mdiChevronLeft} size="1.5rem" />
        </button>
        <span className="year-value mono" aria-live="polite">
          {shown}
        </span>
        <button
          type="button"
          className="year-btn"
          onClick={() => step(1)}
          disabled={shown >= maxYear}
          aria-label="次の年"
        >
          <ElmMdiIcon d={mdiChevronRight} size="1.5rem" />
        </button>
      </div>

      <p className="year-count">
        {shown} 年の祝日は <span className="mono">{list.length}</span> 日です。
      </p>

      <ul key={shown} className="year-grid">
        {list.map((h) => (
          <li
            key={h.key}
            className="holiday-row"
            data-past={h.key < todayKey}
            data-today={h.key === todayKey}
          >
            <span
              className="dot"
              data-substitute={h.name === "休日"}
              aria-hidden="true"
            />
            <span className="holiday-date mono">{formatShort(h.date)}</span>
            <span className="holiday-name display">
              <HolidayName name={h.name} />
            </span>
          </li>
        ))}
      </ul>
    </section>
  );
}

function App() {
  const { holidays, live } = useHolidays();
  const [time] = useState(detectTimeInfo);
  const stats = statsFor(holidays);
  const upcoming = upcomingHolidays(holidays, time.today, 6);

  return (
    <div className="em-doc">
      <header className="topbar">
        <span className="wordmark mono">jp-holidays</span>
        <nav className="topbar-nav">
          <a className="icon-link" href={REPO} aria-label="GitHub リポジトリ">
            <ElmMdiIcon d={mdiGithub} size="1.4rem" />
          </a>
          <ElmToggleTheme />
        </nav>
      </header>

      <main>
        <section className="hero">
          <span className="eyebrow mono">国民の祝日 · PUBLIC HOLIDAYS API</span>
          <ElmHeading level={1} className="hero-title">
            日本の祝日を、
            <br />
            静的 JSON で配信。
          </ElmHeading>
          <ElmParagraph>
            内閣府が公開する「国民の祝日」を、認証不要・CORS 対応の静的な JSON
            API として配信します。サーバーは不要で、URL
            を叩くだけ。下のパネルは、この API
            から取得したデータで「今日が祝日かどうか」を判定しています。
          </ElmParagraph>

          <TodayPanel time={time} holidays={holidays} live={live} />

          <div className="stats">
            <div className="stat">
              <span className="stat-num mono">
                {stats.count.toLocaleString("en-US")}
              </span>
              <span className="stat-label">祝日エントリ</span>
            </div>
            <div className="stat">
              <span className="stat-num mono">
                {stats.minYear}–{stats.maxYear}
              </span>
              <span className="stat-label">収録期間</span>
            </div>
            <div className="stat">
              <span className="stat-num display">内閣府</span>
              <span className="stat-label">データ提供</span>
            </div>
          </div>
        </section>

        <ElmDivider />

        <section className="section">
          <span className="eyebrow mono">UPCOMING</span>
          <ElmHeading level={2}>次の祝日</ElmHeading>
          <ul className="upcoming">
            {upcoming.map((h) => (
              <li key={h.key} className="holiday-row">
                <span
                  className="dot"
                  data-substitute={h.name === "休日"}
                  aria-hidden="true"
                />
                <span className="holiday-date mono">{formatShort(h.date)}</span>
                <span className="holiday-name display">
                  <HolidayName name={h.name} />
                </span>
              </li>
            ))}
          </ul>
        </section>

        <ElmDivider />

        <YearPanel holidays={holidays} today={time.today} />

        <ElmDivider />

        <section className="section">
          <span className="eyebrow mono">ENDPOINTS</span>
          <ElmHeading level={2}>エンドポイント</ElmHeading>
          <ElmParagraph>
            ベース URL は <span className="mono">{BASE}</span> です。すべて
            GET・JSON で応答します。
          </ElmParagraph>

          <div className="endpoints">
            <ElmTable>
              <ElmTableHeader>
                <ElmTableRow>
                  <ElmTableCell isHeader text="メソッド" />
                  <ElmTableCell isHeader text="パス" />
                  <ElmTableCell isHeader text="内容" />
                  <ElmTableCell isHeader text="" />
                </ElmTableRow>
              </ElmTableHeader>
              <ElmTableBody>
                {ENDPOINTS.map((ep) => (
                  <ElmTableRow key={ep.path}>
                    <ElmTableCell>
                      <span className="method mono">GET</span>
                    </ElmTableCell>
                    <ElmTableCell>
                      <span className="endpoint-path mono">{ep.path}</span>
                    </ElmTableCell>
                    <ElmTableCell>{ep.desc}</ElmTableCell>
                    <ElmTableCell>
                      <CopyButton value={`${BASE}${ep.path}`} label={ep.path} />
                    </ElmTableCell>
                  </ElmTableRow>
                ))}
              </ElmTableBody>
            </ElmTable>
          </div>

          <ElmCallout type="tip">
            このページ自体が、同じ API
            を呼び出してデータを取得しています。データは毎日自動で内閣府の最新版に更新されます。
          </ElmCallout>
        </section>

        <ElmDivider />

        <section className="section">
          <span className="eyebrow mono">USAGE</span>
          <ElmHeading level={2}>使い方</ElmHeading>
          <ElmParagraph>
            日付操作のライブラリは不要です。日付文字列（
            <span className="mono">YYYY-MM-DD</span>）でそのまま引けます。
          </ElmParagraph>

          <ElmTabs defaultValue="fetch">
            <ElmTabList>
              <ElmTab value="fetch">
                <ElmLanguageIcon language="ts" />
                &nbsp;fetch API
              </ElmTab>
              <ElmTab value="curl">
                <ElmLanguageIcon language="bash" />
                &nbsp;curl
              </ElmTab>
              <ElmTab value="python">
                <ElmLanguageIcon language="python" />
                &nbsp;Python
              </ElmTab>
              <ElmTab value="reqwest">
                <ElmLanguageIcon language="rust" />
                &nbsp;Rust (reqwest)
              </ElmTab>
              <ElmTab value="lib">
                <ElmLanguageIcon language="rust" />
                &nbsp;Rust (lib)
              </ElmTab>
            </ElmTabList>
            <ElmTabPanel value="fetch" className="margin-zero">
              <ElmCodeBlock
                code={FETCH_EXAMPLE}
                language="typescript"
                caption="2026 年の祝日を取得"
              />
            </ElmTabPanel>
            <ElmTabPanel value="curl" className="margin-zero">
              <ElmCodeBlock
                code={CURL_EXAMPLE}
                language="bash"
                caption="curl"
              />
            </ElmTabPanel>
            <ElmTabPanel value="python" className="margin-zero">
              <ElmCodeBlock
                code={PYTHON_EXAMPLE}
                language="python"
                caption="requests で 2026 年の祝日を取得"
              />
            </ElmTabPanel>
            <ElmTabPanel value="reqwest" className="margin-zero">
              <ElmCodeBlock
                code={REQWEST_EXAMPLE}
                language="rust"
                caption="reqwest で 2026 年の祝日を取得"
              />
            </ElmTabPanel>
            <ElmTabPanel value="lib" className="margin-zero">
              <ElmCodeBlock
                code={LIB_EXAMPLE}
                language="rust"
                caption="jp-holidays-lib クレートを直接使う"
              />
            </ElmTabPanel>
          </ElmTabs>
        </section>

        <ElmDivider />

        <section className="section">
          <span className="eyebrow mono">REFERENCE</span>
          <ElmHeading level={2}>リファレンス</ElmHeading>
          <div className="links">
            <a className="link-card" href={`${apiHref}/docs.html`}>
              <span className="link-head display">API リファレンス</span>
              <span className="link-sub">
                Scalar によるインタラクティブ表示
              </span>
              <ElmMdiIcon d={mdiOpenInNew} size="1.1rem" />
            </a>
            <a className="link-card" href={`${apiHref}/openapi.json`}>
              <span className="link-head display">OpenAPI 仕様</span>
              <span className="link-sub">openapi.json（3.1）</span>
              <ElmMdiIcon d={mdiOpenInNew} size="1.1rem" />
            </a>
            <a className="link-card" href={REPO}>
              <span className="link-head display">ソースコード</span>
              <span className="link-sub">GitHub · 46ki75/jp-holidays</span>
              <ElmMdiIcon d={mdiGithub} size="1.1rem" />
            </a>
          </div>
        </section>
      </main>

      <footer className="footer">
        <ElmParagraph>
          データ元:{" "}
          <ElmInlineText href="https://www8.cao.go.jp/chosei/shukujitsu/gaiyou.html">
            内閣府「国民の祝日」について
          </ElmInlineText>
          <br />
          <ElmInlineText href={REPO}>46ki75/jp-holidays</ElmInlineText> · MIT
          License
        </ElmParagraph>
      </footer>
    </div>
  );
}

export default App;
