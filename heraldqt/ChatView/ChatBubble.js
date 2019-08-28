"use strict";

/**
 *  Calculates width of ChatBubble.
 *
 *  @param {number} chat_pane_width The width of the ChatPane
 *  @param {number} message_metrics_width The default width of the message
 *  @return {number} The width of the ChatBubble
 */
function calculate_width(chat_pane_width, message_metrics_width) {
  const too_long = message_metrics_width >= chat_pane_width / 2;

  if (too_long) {
    // don't use more than half of the chat pane
    return chat_pane_width / 2;
  }
}
