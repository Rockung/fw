const buf1 = Buffer.from('this is a tést');

// Prints: this is a tést
console.log(buf1.toString());
console.log(buf1.toString('hex'));

// Prints: this is a tC)st
console.log(buf1.toString('ascii'));


const buf2 = Buffer.from('7468697320697320612074c3a97374', 'hex');

// Prints: this is a tést
console.log(buf2.toString());
