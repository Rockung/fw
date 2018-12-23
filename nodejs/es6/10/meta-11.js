console.log(1["name"]);
// undefined

console.log(Reflect.get(1, "name"));
// TypeError: Reflect.get called on non-object
