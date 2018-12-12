if (true) {
    console.log(typeof anUndeclaredVariable); // 'undefined'
    console.log(typeof random); // ReferenceError (TDZ)
let random; }