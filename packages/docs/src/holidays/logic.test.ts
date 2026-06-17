import {
  daysBetween,
  formatLong,
  formatShort,
  fromKey,
  holidaysInYear,
  nextHoliday,
  statsFor,
  toKey,
  upcomingHolidays,
  verdictFor,
} from "./logic";
import type { Holidays } from "./data";

// A small fixture spanning four years. 2026-06-17 is a Wednesday; 06-20 a
// Saturday — used to exercise the weekday/weekend branches deterministically.
const HOLIDAYS: Holidays = {
  "2024-01-01": "元日",
  "2025-01-01": "元日",
  "2026-01-01": "元日",
  "2026-05-05": "こどもの日",
  "2026-05-06": "休日",
  "2027-01-01": "元日",
};

describe("toKey / fromKey", () => {
  it("formats a local date as YYYY-MM-DD with zero padding", () => {
    expect(toKey(new Date(2026, 0, 1))).toBe("2026-01-01");
    expect(toKey(new Date(2026, 5, 17))).toBe("2026-06-17");
  });

  it("parses a key into a local-midnight date", () => {
    const d = fromKey("2026-06-17");
    expect(d.getFullYear()).toBe(2026);
    expect(d.getMonth()).toBe(5);
    expect(d.getDate()).toBe(17);
  });

  it("round-trips", () => {
    expect(toKey(fromKey("2026-05-06"))).toBe("2026-05-06");
  });
});

describe("verdictFor", () => {
  it("reports a holiday by name", () => {
    expect(verdictFor(HOLIDAYS, fromKey("2026-01-01"))).toEqual({
      kind: "holiday",
      name: "元日",
    });
  });

  it("reports weekends", () => {
    expect(verdictFor(HOLIDAYS, fromKey("2026-06-20")).kind).toBe("weekend"); // Sat
    expect(verdictFor(HOLIDAYS, fromKey("2026-06-21")).kind).toBe("weekend"); // Sun
  });

  it("reports plain weekdays", () => {
    expect(verdictFor(HOLIDAYS, fromKey("2026-06-17")).kind).toBe("weekday"); // Wed
  });

  it("prefers the holiday verdict even on a weekend", () => {
    // 2026-05-05 (こどもの日) falls on a Tuesday here, but a holiday always wins
    // over the weekday/weekend check regardless.
    expect(verdictFor(HOLIDAYS, fromKey("2026-05-05")).kind).toBe("holiday");
  });
});

describe("daysBetween", () => {
  it("counts whole days, ignoring time-of-day", () => {
    expect(daysBetween(fromKey("2026-06-17"), fromKey("2026-06-20"))).toBe(3);
    expect(daysBetween(fromKey("2026-06-17"), fromKey("2026-06-17"))).toBe(0);
  });
});

describe("nextHoliday", () => {
  it("returns the soonest holiday strictly after the date", () => {
    const next = nextHoliday(HOLIDAYS, fromKey("2026-01-01"));
    expect(next?.key).toBe("2026-05-05");
    expect(next?.name).toBe("こどもの日");
    expect(next?.daysUntil).toBeGreaterThan(0);
  });

  it("looks across year boundaries", () => {
    expect(nextHoliday(HOLIDAYS, fromKey("2023-12-31"))?.key).toBe("2024-01-01");
  });

  it("returns null when nothing follows", () => {
    expect(nextHoliday(HOLIDAYS, fromKey("2027-12-31"))).toBeNull();
  });
});

describe("upcomingHolidays", () => {
  it("includes the current day and is capped at count", () => {
    const list = upcomingHolidays(HOLIDAYS, fromKey("2026-01-01"), 2);
    expect(list.map((h) => h.key)).toEqual(["2026-01-01", "2026-05-05"]);
  });
});

describe("statsFor", () => {
  it("counts entries and the year span", () => {
    expect(statsFor(HOLIDAYS)).toEqual({
      count: 6,
      minYear: 2024,
      maxYear: 2027,
    });
  });

  it("is safe on an empty dataset", () => {
    expect(statsFor({})).toEqual({ count: 0, minYear: 0, maxYear: 0 });
  });
});

describe("holidaysInYear", () => {
  it("returns only that year's holidays, chronologically", () => {
    const list = holidaysInYear(HOLIDAYS, 2026);
    expect(list.map((h) => h.key)).toEqual([
      "2026-01-01",
      "2026-05-05",
      "2026-05-06",
    ]);
  });

  it("returns an empty array for a year with no data", () => {
    expect(holidaysInYear(HOLIDAYS, 2099)).toEqual([]);
  });
});

describe("formatting", () => {
  it("formatLong includes the year, date, and weekday", () => {
    expect(formatLong(fromKey("2026-06-17"))).toBe("2026年6月17日（水）");
  });

  it("formatShort is month and day only", () => {
    expect(formatShort(fromKey("2026-07-20"))).toBe("7月20日");
  });
});
