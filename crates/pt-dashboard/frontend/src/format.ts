export function scoreTone(score: number): "buy" | "sell" | "flat" {
  if (score >= 0.35) {
    return "buy";
  }
  if (score <= -0.35) {
    return "sell";
  }
  return "flat";
}

export function formatBps(value: number): string {
  return `${value.toFixed(2)} bps`;
}
