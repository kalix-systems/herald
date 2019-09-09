"use strict";

export function unwrapOr<T>(maybeVal: T, fallback: T): T {
  // TODO this produces false positives
  if (maybeVal) {
    return maybeVal;
  } else {
    return fallback;
  }
}

export function friendlyTimestamp(msEpochTime: number): string {
  const secondMsRatio = 1000;
  const secondsPerMinute = 60;
  const secondsPerHour = 3600;
  const secondsPerDay = 3600 * 24;

  const dt = new Date(msEpochTime);
  const now = Date.now();
  const diff = (now - dt.valueOf()) / secondMsRatio;

  if (diff < 0) return "";

  if (diff < secondsPerMinute) return "NOW";

  if (diff < secondsPerHour) {
    return Math.floor(diff / secondsPerMinute) + " MIN AGO";
  }

  if (diff < secondsPerDay) {
    return Math.floor(diff / secondsPerHour) + " HR AGO";
  }

  return dt.toDateString();
}
