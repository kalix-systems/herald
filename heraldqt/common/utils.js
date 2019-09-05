"use strict";

function unwrap_or(maybe_val, fallback) {
  // TODO this produces false positives
  if (maybe_val) {
    return maybe_val;
  } else {
    return fallback;
  }
}

function friendly_timestamp(ms_epoch_time) {
  const second_ms_ratio = 1000;
  const seconds_per_minute = 60;
  const seconds_per_hour = 3600;
  const seconds_per_day = 3600 * 24;

  const dt = new Date(ms_epoch_time);
  const now = Date.now();
  const diff = (now - dt) / second_ms_ratio;

  if (diff < 0) return "";

  if (diff < seconds_per_minute) return "NOW";

  if (diff < seconds_per_hour) {
    return Math.floor(diff / seconds_per_minute) + " MIN AGO";
  }

  if (diff < seconds_per_day) {
    return Math.floor(diff / seconds_per_hour) + " HR AGO";
  }

  return dt.toDateString();
}
