// A Promise that times out after given time (t)
function delay(t) {
  return new Promise((resolve, reject) => {
    setTimeout(resolve, t);
  });
}

// Whichever Promise fulfills first is the result passed to our handler
Promise.race([
  fetchData(),
  delay(5000).then(() => { data: "test" })
])
.then((res) => {
   // this will be "test" if fetchData() takes longer than 5 sec.
   console.log("data:", res.data);
})
.catch(function(err) {
  console.log("error:", err);
});

// In some cases, you may not want to wait for all the promises in your array 
// to resolve, but simply want to get the results of the first promise in the 
// array to fulfill. In that scenario, Promise.race() can be used.