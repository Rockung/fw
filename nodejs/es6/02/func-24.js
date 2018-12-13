var getNumber = new Function("number = 42", "return number;");
console.log(getNumber()); // 42