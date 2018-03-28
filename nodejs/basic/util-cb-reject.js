const util = require('util');

async function fn() {
  return await Promise.reject(null);
}
const callbackFunction = util.callbackify(fn);

callbackFunction((err, ret) => {
	if (err && err.hasOwnProperty('reason') && err.reason === null) {
		console.log('error occurs!');
	}
});
