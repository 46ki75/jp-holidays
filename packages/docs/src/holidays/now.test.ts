import { detectTimeInfo } from "./now";

// detectTimeInfo reads the host's real clock/zone via Temporal, so this is a
// contract smoke test: it must always return a well-formed TimeInfo, and the
// drift field must either be null or carry both human-readable labels.
describe("detectTimeInfo", () => {
  it("returns a well-formed TimeInfo", () => {
    const info = detectTimeInfo();

    expect(typeof info.timeZone).toBe("string");
    expect(info.timeZone.length).toBeGreaterThan(0);

    expect(info.today).toBeInstanceOf(Date);
    expect(Number.isNaN(info.today.getTime())).toBe(false);
  });

  it("either reports no drift or labels both sides of it", () => {
    const { drift } = detectTimeInfo();
    if (drift !== null) {
      expect(typeof drift.localLabel).toBe("string");
      expect(typeof drift.tokyoLabel).toBe("string");
      expect(drift.localLabel).not.toBe(drift.tokyoLabel);
    }
  });
});
