import { fireEvent, render, screen } from "@testing-library/react";
import { YearPanel } from "./App";
import type { Holidays } from "./holidays/data";

// 2024 → 2027, with 2026 holding two entries and 2027 a single one. today is
// a 2026 weekday, so the default view opens on 2026.
const HOLIDAYS: Holidays = {
  "2024-01-01": "元日",
  "2025-01-01": "元日",
  "2026-01-01": "元日",
  "2026-05-05": "こどもの日",
  "2027-01-01": "元日",
};
const TODAY = new Date(2026, 5, 17); // 2026-06-17

function setup() {
  const { container } = render(<YearPanel holidays={HOLIDAYS} today={TODAY} />);
  const year = () => container.querySelector(".year-value")?.textContent;
  const rows = () => container.querySelectorAll(".year-grid .holiday-row");
  const prev = screen.getByRole("button", { name: "前の年" });
  const next = screen.getByRole("button", { name: "次の年" });
  return { container, year, rows, prev, next };
}

describe("YearPanel", () => {
  it("opens on the current year and lists its holidays", () => {
    const { year, rows } = setup();
    expect(
      screen.getByRole("heading", { name: /年間の祝日/ }),
    ).toBeInTheDocument();
    expect(year()).toBe("2026");
    expect(rows()).toHaveLength(2);
    expect(screen.getByText("1月1日")).toBeInTheDocument();
    expect(screen.getByText("5月5日")).toBeInTheDocument();
  });

  it("dims dates that have already passed in the current year", () => {
    const { container } = setup();
    // Both 2026 holidays (Jan, May) precede today (June).
    expect(
      container.querySelectorAll('.holiday-row[data-past="true"]'),
    ).toHaveLength(2);
  });

  it("steps forward and backward through the available years", () => {
    const { year, rows, prev, next } = setup();

    fireEvent.click(next);
    expect(year()).toBe("2027");
    expect(rows()).toHaveLength(1);

    fireEvent.click(prev);
    fireEvent.click(prev);
    fireEvent.click(prev);
    expect(year()).toBe("2024");
  });

  it("disables the arrows at the dataset bounds", () => {
    const { year, prev, next } = setup();

    fireEvent.click(prev);
    fireEvent.click(prev);
    expect(year()).toBe("2024");
    expect(prev).toBeDisabled();

    fireEvent.click(next);
    fireEvent.click(next);
    fireEvent.click(next);
    expect(year()).toBe("2027");
    expect(next).toBeDisabled();
  });
});
