"use strict"

function unwrap_or(maybe_val, fallback) {
    // TODO this produces false positives
    if (maybe_val) {
        return maybe_val
    } else {
        return fallback
    }
}


/**
 *  for debugging, prints out all properties of an object
 *
 *  @param {object} the object to introspect upon
*/
function introspect(object) {
    var key
    for (key in object) {
        if (typeof object[key] !== "function")
            if (key !== "objectName")
                console.log(key + ":" + object[key])
    }
}

/**
 *  returns result if index is invalid, otherwise, gets the
 *  item at the proper index. should be used at the FFI boundry
 *  to prevent panics.
 *
 *  @param {index} index to query the queriable at
 *  @param {quierable} a FUNCTION POINTER. to a function that
 *  queries across FFI
 *  @param {length} the length of the queried object, i.e. max index
 *  @param {result} a result to return in case of failure
*/
function try_index_or(index, queriable, length, result) {
     if( index < 0 || index > length)
         return result;
     else
        return queriable(index);

 }
