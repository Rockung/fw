function sum(a, b) {
  return a + b;
}
export { sum as add };          // rename when exporting

function multiply(a, b) {
  return a * b;
}
export { multiply as default }; // rename to default when exporting
