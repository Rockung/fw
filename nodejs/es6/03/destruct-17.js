function createObj(name, value, {a:x, b:y, c:z}) {
    let obj = {};
    obj.x = x;
    obj.y = y;
    obj.z = z;
    obj[name] = value;
    return obj;
}
let options = {a: 1, b: 2, c: 3};
let testObj = createObj('test', 4, options);
console.log(testObj)// {x:1, y:2, z:3, test:4}