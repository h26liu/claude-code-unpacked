export const color = {
  cyan(text) {
    return `\u001b[36m${text}\u001b[0m`;
  },
  yellow(text) {
    return `\u001b[33m${text}\u001b[0m`;
  },
  dim(text) {
    return `\u001b[2m${text}\u001b[0m`;
  },
  bold(text) {
    return `\u001b[1m${text}\u001b[0m`;
  }
};
