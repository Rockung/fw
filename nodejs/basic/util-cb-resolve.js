const util = require('util');

async function fn() {
  return await Promise.resolve('hello world');
}
const callbackFunction = util.callbackify(fn);

callbackFunction((err, ret) => {
	if (err) throw err;
	console.log(ret);
});
