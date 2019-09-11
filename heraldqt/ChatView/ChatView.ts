export function naturalWidth(
  chatPaneWidth: number,
  messageMetricsWidth: number
): number {
  return Math.min((2 * chatPaneWidth) / 3, messageMetricsWidth);
}
