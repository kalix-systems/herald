"use strict";

function unwrap_or(maybe_val, fallback) {
  // TODO this produces false positives
  if (maybe_val) {
   return maybe_val
  } else {
    return fallback
  }
}
