It's very important for APIs to be either 100% synchronous or 100% asynchronous

```
// WARNING! DO NOT USE! BAD UNSAFE HAZARD!
function maybeSync(arg, cb) {
  if (arg) {
	  cb();
		return;
	}
	
	fs.stat('file', cb);
}
```

This API is hazardous because in the following case:

```
const maybeTrue = Math.random() > 0.5;

maybeSync(maybeTrue, () => {
  foo();
});

bar();
```

It is not clear whether foo() or bar() will be called first.

```
function definitelyAsync(arg, cb) {
  if (arg) {
	  process.nextTick(cb);
		return;
	}
	
	fs.stat('file', cb);
}
```
