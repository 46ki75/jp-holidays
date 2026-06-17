import type { Holidays } from "./data";

const WEEKDAY_JP = ["日", "月", "火", "水", "木", "金", "土"] as const;

/** Local-time `YYYY-MM-DD` key (matches the API's date keys). */
export function toKey(date: Date): string {
  const y = date.getFullYear();
  const m = String(date.getMonth() + 1).padStart(2, "0");
  const d = String(date.getDate()).padStart(2, "0");
  return `${y}-${m}-${d}`;
}

/** Parse a `YYYY-MM-DD` key into a local-midnight Date. */
export function fromKey(key: string): Date {
  const [y, m, d] = key.split("-").map(Number);
  return new Date(y, m - 1, d);
}

export type Verdict =
  | { kind: "holiday"; name: string }
  | { kind: "weekend" }
  | { kind: "weekday" };

/**
 * A "day off" is a public holiday or a weekend — mirrors
 * `Client::is_day_off` in the Rust library.
 */
export function verdictFor(holidays: Holidays, date: Date): Verdict {
  const name = holidays[toKey(date)];
  if (name) return { kind: "holiday", name };
  const day = date.getDay();
  if (day === 0 || day === 6) return { kind: "weekend" };
  return { kind: "weekday" };
}

export interface HolidayEntry {
  key: string;
  date: Date;
  name: string;
}

function sortedEntries(holidays: Holidays): HolidayEntry[] {
  return Object.keys(holidays)
    .sort()
    .map((key) => ({ key, date: fromKey(key), name: holidays[key] }));
}

/** Whole days from `from` (truncated to midnight) until `to`. */
export function daysBetween(from: Date, to: Date): number {
  const a = new Date(from.getFullYear(), from.getMonth(), from.getDate());
  const b = new Date(to.getFullYear(), to.getMonth(), to.getDate());
  return Math.round((b.getTime() - a.getTime()) / 86_400_000);
}

/** The soonest holiday strictly after `from`. */
export function nextHoliday(
  holidays: Holidays,
  from: Date,
): (HolidayEntry & { daysUntil: number }) | null {
  const todayKey = toKey(from);
  for (const entry of sortedEntries(holidays)) {
    if (entry.key > todayKey) {
      return { ...entry, daysUntil: daysBetween(from, entry.date) };
    }
  }
  return null;
}

/** Up to `count` holidays on or after `from`, chronologically. */
export function upcomingHolidays(
  holidays: Holidays,
  from: Date,
  count: number,
): HolidayEntry[] {
  const todayKey = toKey(from);
  return sortedEntries(holidays)
    .filter((entry) => entry.key >= todayKey)
    .slice(0, count);
}

/** Every holiday in a given calendar year, chronologically. */
export function holidaysInYear(
  holidays: Holidays,
  year: number,
): HolidayEntry[] {
  const prefix = `${year}-`;
  return sortedEntries(holidays).filter((entry) =>
    entry.key.startsWith(prefix),
  );
}

export interface HolidayStats {
  count: number;
  minYear: number;
  maxYear: number;
}

export function statsFor(holidays: Holidays): HolidayStats {
  const keys = Object.keys(holidays);
  const years = keys.map((k) => Number(k.slice(0, 4)));
  return {
    count: keys.length,
    minYear: years.length ? Math.min(...years) : 0,
    maxYear: years.length ? Math.max(...years) : 0,
  };
}

/** e.g. `2026年6月17日（火）` */
export function formatLong(date: Date): string {
  return `${date.getFullYear()}年${date.getMonth() + 1}月${date.getDate()}日（${WEEKDAY_JP[date.getDay()]}）`;
}

/** e.g. `6月17日` */
export function formatShort(date: Date): string {
  return `${date.getMonth() + 1}月${date.getDate()}日`;
}
