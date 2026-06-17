import { Temporal } from "@js-temporal/polyfill";

const TOKYO = "Asia/Tokyo";

export interface TimeInfo {
  /** The viewer's IANA time zone (e.g. `America/New_York`). */
  timeZone: string;
  /** Local-midnight `Date` for the viewer's current civil date (for date logic). */
  today: Date;
  /**
   * Set when the viewer's civil date differs from Japan's right now. Holiday
   * data is defined on Japanese calendar dates (JST), so a viewer in another
   * zone can be a day ahead of / behind Japan near midnight.
   */
  drift: { localLabel: string; tokyoLabel: string } | null;
}

function label(date: Temporal.PlainDate): string {
  return `${date.month}月${date.day}日`;
}

/** Detect the viewer's time zone and today's date, and any drift from Japan. */
export function detectTimeInfo(): TimeInfo {
  const timeZone = Temporal.Now.timeZoneId();
  const local = Temporal.Now.plainDateISO();
  const tokyo = Temporal.Now.plainDateISO(TOKYO);

  const today = new Date(local.year, local.month - 1, local.day);
  const drift =
    local.toString() === tokyo.toString()
      ? null
      : { localLabel: label(local), tokyoLabel: label(tokyo) };

  return { timeZone, today, drift };
}
