// Array.prototype.includes

['apple', 'banana', 'carrot'].includes('apple');
['apple', 'banana', 'carrot'].includes('orange');

// equivalent to

['apple', 'banana', 'carrot'].indexOf('apple') >= 0;    //true
['apple', 'banana', 'carrot'].indexOf('orange') >= 0;   //false

// major difference

const arr = [NaN];
arr.includes(NaN);              // true
arr.indexOf(NaN);               // -1