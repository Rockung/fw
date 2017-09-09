const crypto = require('crypto');

const ciphers = crypto.getCiphers();
console.log(ciphers);

const curves = crypto.getCurves();
console.log(curves);

const hashes = crypto.getHashes();
console.log(hashes);
