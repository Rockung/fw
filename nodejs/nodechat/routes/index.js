module.exports.index = index;
module.exports.login = login;
module.exports.loginProcess = loginProcess;
module.exports.chat = chat;

function index(req, res){
	// res.send('Index');
	// res.render('index', {layout: 'layout', title: 'Index'});
	console.log("index start...");
	
	res.cookie('IndexCookie', 'This was set from Index');
	res.render('index', {
		title: 'Index', 
		cookie: JSON.stringify(req.cookies), 
		session: JSON.stringify(req.session)
	});
	console.log("index end...");
};

function login(req, res){
	res.render('login', {layout: 'layout', title: 'Login'});
};

function loginProcess(req, res){
	res.redirect('/');
};

function chat(req, res){
	res.render('chat', {layout: 'layout', title: 'Chat'});
};
