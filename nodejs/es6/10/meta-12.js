function sayHello() {
    console.log(`${this.name} says hello`);
}

const person = {
    name: "Jack"
};

Function.prototype.apply.call(sayHello, person);
// Jack says hello

Reflect.apply(sayHello, person, []);
// Jack says hello

Reflect.apply(sayHello, person);
// TypeError: CreateListFromArrayLike called on non-object
