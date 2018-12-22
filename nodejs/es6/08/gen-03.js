function *calculator() {
  const num1 = yield "I am a calculator";
  const num2 = yield "I add numbers";
  console.log(`Sum is: ${num1 + num2}`);
}

const myGenerator = calculator();
console.log(myGenerator.next());
console.log(myGenerator.next(2));
console.log(myGenerator.next(3));
