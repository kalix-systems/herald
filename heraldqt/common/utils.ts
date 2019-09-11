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

export function safeSwitch<T>(
  cond: boolean,
  first: T,
  second: T
): T | undefined {
  if (typeof cond !== "boolean") {
    throw new Error("condition was not of type boolean");
  }

  if (cond) {
    return first;
  } else {
    return second;
  }
}

/*
 * Generates qrc URI from file path
 */
export function safeToQrcURI(url: string): string {
  if (typeof url !== "string") {
    throw new Error("Expected url to be string");
  }
  return "file:" + url;
}
