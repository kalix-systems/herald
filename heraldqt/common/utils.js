"use strict";
function unwrap_or(maybe_val, fallback) {
    // TODO this produces false positives
    if (maybe_val) {
        return maybe_val;
    }
    else {
        return fallback;
    }
}
function friendly_timestamp(ms_epoch_time) {
    var second_ms_ratio = 1000;
    var seconds_per_minute = 60;
    var seconds_per_hour = 3600;
    var seconds_per_day = 3600 * 24;
    var dt = new Date(ms_epoch_time);
    var now = Date.now();
    var diff = (now - dt.valueOf()) / second_ms_ratio;
    if (diff < 0)
        return "";
    if (diff < seconds_per_minute)
        return "NOW";
    if (diff < seconds_per_hour) {
        return Math.floor(diff / seconds_per_minute) + " MIN AGO";
    }
    if (diff < seconds_per_day) {
        return Math.floor(diff / seconds_per_hour) + " HR AGO";
    }
    return dt.toDateString();
}
