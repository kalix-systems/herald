export function naturalWidth(chatPaneWidth, messageMetricsWidth) {
    return Math.min((2 * chatPaneWidth) / 3, messageMetricsWidth);
}
