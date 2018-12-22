const buffer = new ArrayBuffer(16);
const int32View = new Int32Array(buffer);
for (let i = 0; i < int32View.length; i++) {
    int32View[i] = i * 2;
}
console.log(int32View); // [0, 2, 4, 6]