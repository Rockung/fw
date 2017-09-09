const buf = Buffer.from('buffer');

// Prints:
//   [0, 98]
//   [1, 117]
//   [2, 102]
//   [3, 102]
//   [4, 101]
//   [5, 114]
for (const pair of buf.entries()) {
  [index, byte] = pair;
  console.log('[ %d, %d ]', index, byte);
  console.log(pair);
}
