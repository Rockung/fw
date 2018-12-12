let notGlobal = "hello";
var isGlobal = "what up";
{ console.log(notGlobal); } // hello
{ console.log(isGlobal); } // what up
console.log(global.notGlobal) // undefined
console.log(global.isGlobal) //'what up'
