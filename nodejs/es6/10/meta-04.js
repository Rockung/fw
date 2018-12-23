const voterValidator = {
  set: function(obj, prop, value) {
    if (prop === "age") {
      if (!Number.isInteger(value)) {
        throw new TypeError("Input age is not an integer");
      }

      if (value < 18) {
        throw new RangeError("Input age seems invalid");
      }
    } else if (prop === "residency") {
      if (value === false) {
        throw new Error("Residency is mandatory to vote");
      } 
    }

    // The default behavior to store the value
    obj[prop] = value;
    
    // Indicate success
    return true;
  }
};

const person = new Proxy({}, voterValidator);
person.age = 23;
person.residency = false;
person.age = "young";
person.age = 200;
