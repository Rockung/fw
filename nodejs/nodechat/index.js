var express = require('express');
var app = express()
var session = require('express-session');
var RedisStore = require('connect-redis')(session);

/*
app.set('trust proxy', 1) // trust first proxy 
app.use(session({
  secret: 'keyboard cat',
  resave: false,
  saveUninitialized: true,
  cookie: { secure: false }
}))
*/

app.use(session({
    store: new RedisStore({url: 'redis://localhost'}),
    secret: 'keyboard cat'
}));

app.use(function(req, res, next){
	if(req.session.pageCount)
		req.session.pageCount++;
	else
		req.session.pageCount = 1;
	console.log("Page count: " + req.session.pageCount);
	next();
});

app.get('/', function (req, res) {
  res.send(req.session.id + "<br>")
})

app.listen(3000);
console.log("App server running on port 3000");
