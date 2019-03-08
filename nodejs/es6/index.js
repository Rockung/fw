import { Socket } from 'phoenix';

window.onload = function() {
  let b = document.getElementById('connect');
  b.addEventListener('click',() => {
    let socket = new Socket("wss://beam.oorja.io/socket", 
      {
        params: {userToken: "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzUxMiJ9.eyJ1c2VySWQiOiJbYW5vbl1XWWNISjhaUVE3RGQiLCJ2IjoxLCJpYXQiOjE1NTE5NTc5NzA0Njl9.rJgwXzX2HP9u-CtIUAS9QGmxWsdOJac1RHFa9r4kvv9qf-zjPGVuVKGb2wqB5H1f_N6oD-dMTMV-PzFbF8puXg"}
      }
    );
    socket.connect();
  }, false);
}

const helloWorld = () => {
    console.log("Hello! We are all set!");
    console.log("Arrow functions are working");
};
helloWorld();
