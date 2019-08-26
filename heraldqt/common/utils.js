"use strict";

function unwrap_or(maybe_val, fallback) {
  if (maybe_val) {
   return maybe_val
  } else {
    return fallback
  }
}
