import { useEffect, useState } from "react";
import { FALLBACK_HOLIDAYS, type Holidays } from "./data";

export interface HolidaysState {
  holidays: Holidays;
  /** True once the full dataset has been fetched from the live API. */
  live: boolean;
}

/**
 * Returns holiday data, starting from the bundled fallback so the UI renders
 * instantly, then replacing it with the full live dataset fetched from the
 * deployed API (the page documents). Failures keep the fallback.
 */
export function useHolidays(): HolidaysState {
  const [state, setState] = useState<HolidaysState>({
    holidays: FALLBACK_HOLIDAYS,
    live: false,
  });

  useEffect(() => {
    let cancelled = false;
    fetch(`${import.meta.env.BASE_URL}api/v1/holidays.json`)
      .then((res) => (res.ok ? (res.json() as Promise<Holidays>) : null))
      .then((data) => {
        if (!cancelled && data && typeof data === "object") {
          setState({ holidays: data, live: true });
        }
      })
      .catch(() => {
        /* keep the bundled fallback */
      });
    return () => {
      cancelled = true;
    };
  }, []);

  return state;
}
