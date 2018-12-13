class StudyGroup {
    constructor(name) {
        this.name = name;
    }
    set name(newName) {
        this.name = newName;
    }
}
const jsGroup = new StudyGroup("js");
// RangeError: Maximum call stack size exceeded

// Your setter property recursively setting itself, going into 
// an infinite function call. Therefore, you must never have your 
// setter method name the same as that of your property because 
// accessing the property setter by its own name inside the setter 
// creates an infinite recursive function call.