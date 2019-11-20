"use strict";
export function unwrapOr(maybeVal, fallback) {
    // TODO this produces false positives
    if (maybeVal) {
        return maybeVal;
    }
    else {
        return fallback;
    }
}
export function friendlyTimestamp(msEpochTime) {
    const secondMsRatio = 1000;
    const secondsPerMinute = 60;
    const secondsPerHour = 3600;
    const secondsPerDay = 3600 * 24;
    const secondsPerWeek = 3600 * 24 * 7;
    const secondsPerYear = 3600 * 24 * 365;
    var weekdays = new Array(7);
    weekdays[0] = "Sun";
    weekdays[1] = "Mon";
    weekdays[2] = "Tues";
    weekdays[3] = "Weds";
    weekdays[4] = "Thurs";
    weekdays[5] = "Fri";
    weekdays[6] = "Sat";
    var months = new Array(12);
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
    if (diff < 0)
        return "";
    if (diff < secondsPerMinute)
        return "Now";
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
function isBoolean(maybeBool) {
    return typeof maybeBool === "boolean";
}
function isString(maybeString) {
    return typeof maybeString === "string";
}
function isObject(maybeObject) {
    return typeof maybeObject === "object";
}
function sameType(first, second) {
    if (typeof first !== typeof second) {
        throw new Error("parameters differ in type");
    }
}
function sameConstructor(first, second) {
    if (!(second instanceof first.constructor)) {
        throw new Error("parameters differ in constructor");
    }
}
export function safeSwitch(cond, first, second) {
    if (!isBoolean(cond)) {
        throw new Error("condition was not of type boolean");
    }
    // throw exception if type differs
    sameType(first, second);
    if (isObject(first)) {
        const firstObj = first;
        const secondObj = second;
        // throw exception if constructor differs
        sameConstructor(firstObj, secondObj);
    }
    if (cond) {
        return first;
    }
    else {
        return second;
    }
}
/*
 * Generates qrc URI from file path
 */
export function safeToQrcURI(url) {
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
export function safeStringOrDefault(maybeString, fallback) {
    if (isString(maybeString)) {
        return maybeString;
    }
    if (isString(fallback)) {
        return fallback;
    }
    return "";
}
/*
* returns the uri of an icon corresponding to the
* receipt code
* */
export function receiptCodeSwitch(receiptCode) {
    switch (receiptCode) {
        case 0: {
            return "";
        }
        case 1: {
            return "qrc:/single-check-receipt-icon.svg";
        }
        case 2: {
            return "qrc:/double-check-receipt-icon.svg";
        }
        case 3: {
            return "qrc:/single-check-receipt-icon.svg";
        }
    }
    return "";
}
