function createObj(name, value, options) {
    let obj = {};
    obj.x = options.a;
    obj.y = options.b;
    obj.z = options.c;
    obj[name] = value;
    return obj;
}
let options = {a: 1, b: 2, c: 3};
let testObj = createObj('test', 4, options);
console.log(testObj)// {x:1, y:2, z:3, test:4}