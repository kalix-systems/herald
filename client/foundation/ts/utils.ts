"use strict";

export function unwrapOr<T>(maybeVal: T, fallback: T): T {
  // TODO this produces false positives
  if (maybeVal) {
    return maybeVal;
  } else {
    return fallback;
  }
}

export function friendlyFileSize(byteSize: number): string {
  if (byteSize < 1000) {
    return byteSize + " B";
  }

  if (byteSize < 10 ** 6) {
    const kb = byteSize / 1000;
    return Math.round(kb) + " KB";
  }

  if (byteSize < 10 ** 9) {
    const mb = byteSize / 10 ** 6;
    return Math.round(10 * mb) / 10 + " MB";
  }

  const gb = byteSize / 10 ** 9;
  return Math.round(10 * gb) / gb + " GB";
}

export function friendlyTimestamp(msEpochTime: number): string {
  const secondMsRatio = 1000;
  const secondsPerMinute = 60;
  const secondsPerHour = 3600;
  const secondsPerDay = 3600 * 24;
  const secondsPerWeek = 3600 * 24 * 7;
  const secondsPerYear = 3600 * 24 * 365;

  const weekdays = new Array(7);
  weekdays[0] = "Sun";
  weekdays[1] = "Mon";
  weekdays[2] = "Tues";
  weekdays[3] = "Weds";
  weekdays[4] = "Thurs";
  weekdays[5] = "Fri";
  weekdays[6] = "Sat";

  const months = new Array(12);
  months[0] = "Jan";
  months[1] = "Feb";
  months[2] = "Mar";
  months[3] = "Apr";
  months[4] = "May";
  months[5] = "Jun";
  months[6] = "Jul";
  months[7] = "Aug";
  months[8] = "Sep";
  months[9] = "Oct";
  months[10] = "Nov";
  months[11] = "Dec";

  const dt = new Date(msEpochTime);
  const now = Date.now();
  const diff = (now - dt.valueOf()) / secondMsRatio;

  if (diff < 0) return "";

  if (diff < secondsPerMinute) return "Now";

  if (diff < secondsPerHour) {
    return Math.floor(diff / secondsPerMinute) + " min";
  }

  if (diff < secondsPerDay) {
    return Math.floor(diff / secondsPerHour) + " hr";
  }

  if (diff < secondsPerWeek) {
    const dayNum = dt.getDay();
    return weekdays[dayNum];
  }

  if (diff < secondsPerYear) {
    const monthNum = dt.getMonth();
    const dateNum = dt.getDate();
    return months[monthNum] + " " + dateNum;
  }

  //not using datestring because don't want day of the week
  const monthNum = dt.getMonth();
  const dateNum = dt.getDate();
  return months[monthNum] + " " + dateNum + " " + dt.getFullYear();
}

export function expireTimeShort(expireTime: number, insertTime: number): string {
  const secondsPerMinute = 60;
  const secondsPerHour = 3600;
  const secondsPerDay = 3600 * 24;
  const secondsPerWeek = 3600 * 24 * 7;
  const secondsPerMonth = 3600 * 24 * 7 * 4
  //using 7 * 4 * 12 instead of 365 because we want to not allow e.g. 13MO as a return
  const secondsPerYear = 3600 * 24 * 7 * 4 * 12;

  const currentTime = Date.now()

  const diff = Math.round((expireTime - currentTime) / 1000);

  if (diff < 0) return "";

  if (diff < secondsPerMinute) return diff + " SEC";

  if (diff < secondsPerHour) {
    return Math.round(diff / secondsPerMinute) + " MIN";
  }

  if (diff < secondsPerDay) {
    return Math.round(diff / secondsPerHour) + " HR";
  }

  if (diff < secondsPerWeek) {
    return Math.round(diff / secondsPerDay) + " D"
  }

  if (diff < secondsPerMonth) {

    return Math.round(diff / secondsPerWeek) + " WK"
  }
  if (diff < secondsPerYear) {
    return Math.round(diff / secondsPerMonth) + " MO"
  }
  return Math.round(diff / secondsPerYear) + " Y"


}

function isBoolean(maybeBool: unknown): boolean {
  return typeof maybeBool === "boolean";
}

function isString(maybeString: unknown): boolean {
  return typeof maybeString === "string";
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

/*
 * If `maybeString` is a valid string, it will be returned.
 * If `maybeString` is not and a valid fallback is provided, it will be used.
 * Finally, if neither of the previous conditions hold, an empty string will be
 * returned.
 * */
export function safeStringOrDefault(
  maybeString: unknown,
  fallback?: unknown
): string {
  if (isString(maybeString)) {
    return maybeString as string;
  }

  if (isString(fallback)) {
    return fallback as string;
  }

  return "";
}

export function initialize(name: string): string {
  const tokens = name.split(" ").slice(0, 3);
  var str = "";
  tokens.forEach(function anon(string) {
    str += string[0].toUpperCase();
  });
  return str;
}

/*
 * returns the uri of an icon corresponding to the
 * receipt code
 * */
export function receiptCodeSwitch(receiptCode: MessageReceiptStatus): string {
  switch (receiptCode) {
    case MessageReceiptStatus.Nil: {
      return "";
    }
    case MessageReceiptStatus.Received: {
      return "qrc:/single-check-receipt-icon.svg";
    }
    case MessageReceiptStatus.Read: {
      return "qrc:/double-check-receipt-icon.svg";
    }
    default:
      return "";
  }
}

export function timerIcon(expireTime: number, insertTime: number): string {
  var timeNow = Date.now();
  var proportion = (timeNow - insertTime) / (expireTime - insertTime);
  if (proportion < 0.25) return "qrc:/mini-timer-icons/full.svg";
  else if (proportion < 0.5) return "qrc:/mini-timer-icons/almost-full.svg";
  else if (proportion < 0.75) return "qrc:/mini-timer-icons/almost-empty.svg";
  else return "qrc:/mini-timer-icons/empty.svg";
}

export function userTime(timestamp: number): string {
        var d = new Date(timestamp)
        var year = d.getFullYear()
        var month = ("0" + (d.getMonth() + 1)).slice(-2)
        var day = ("0" + d.getDate()).slice(-2)
        var hour = d.getHours()
        var min = ("0" + d.getMinutes()).slice(-2)
        var sec = ("0" + d.getSeconds()).slice(-2)

        var time = year + "-" + month + "-" + day + " " + hour + ":" + min + ":" + sec

        return time
    }

export function auxString(code: number, content: string): string {
switch (code) {
    case 0: {
        return " set the expiration time"
        break;
    }

    case 1: {
        return " set the title to " + content
        break;
    }

    case 2: {
        return " set the color"
        break;
}
 case 3: {
      return " set the picture"
      break;
 }

 default : {
     return ""
     break;
 }
}
}
