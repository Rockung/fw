let arr = [];
for (var i=0; i < 3; i++) {
    arr.push(function () { return i });
}
let value = arr[0]();
console.log(value); // 3
