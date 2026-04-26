import { describe, expect, it } from "vitest";
import { formatBps, scoreTone } from "./format";

describe("format helpers", () => {
  it("classifies tones", () => {
    expect(scoreTone(0.7)).toBe("buy");
    expect(scoreTone(-0.7)).toBe("sell");
    expect(scoreTone(0.1)).toBe("flat");
  });

  it("formats bps", () => {
    expect(formatBps(12.345)).toBe("12.35 bps");
  });
});
