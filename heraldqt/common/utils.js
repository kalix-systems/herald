"use strict";

function unwrap_or(maybe_val, fallback) {
  // TODO this produces false positives
  if (maybe_val) {
   return maybe_val
  } else {
    return fallback
  }
}

function friendly_timestamp(ms_epoch_time){


    print(ms_epoch_time/1000)

    const dt = new Date(ms_epoch_time);
    const now =  Date.now();
    const diff = (now - dt) / 1000;

    if(diff < 0) return ""
    if(diff < 3600) return Math.floor(diff/60) + " minutes ago"
    if(diff < 3600*24 ) return Math.floor(diff/(3600)) + " hours ago"
    return dt.toDateString()



    // check in minutes
    // check in hours
    // send date
}
