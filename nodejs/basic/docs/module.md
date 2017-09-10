To determine whether a file has been run directly
  
  ```
	if (require.main === module) {}
  ```

Node.js wraps a module with a function wrapper

  ```
  (function(exports, require, module, __filaname, __dirname) {
		// module code actually lives in here
	});
  ```

when **module.exports** completely replaced by a new object,
it is common to also reassign **exports**

  ```
	module.exports = exports = function Constructor() {
		// function body here
  };
  ```
**exports** is a shortcut to **module.exports**, but it is not reliable.

