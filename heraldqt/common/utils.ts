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

function isBoolean(maybeBool: unknown): boolean {
  return typeof maybeBool === "boolean";
}

function isObject(maybeObject: unknown): boolean {
  return typeof maybeObject === "object";
}

function sameType<T>(first: T, second: T): void {
  if (typeof first !== typeof second) {
    throw new Error("parameters differ in type");
  }
}

function sameConstructor<T extends object>(first: T, second: T): void {
  if (!(second instanceof first.constructor)) {
    throw new Error("parameters differ in constructor");
  }
}

export function safeSwitch<T>(
  cond: boolean,
  first: T,
  second: T
): T | undefined {
  if (!isBoolean(cond)) {
    throw new Error("condition was not of type boolean");
  }

  // throw exception if type differs
  sameType(first, second);

  if (isObject(first)) {
    const firstObj = (first as unknown) as object;
    const secondObj = (second as unknown) as object;

    // throw exception if constructor differs
    sameConstructor(firstObj, secondObj);
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
