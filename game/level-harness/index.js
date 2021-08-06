const express = require('express');
const app = express();

app.get('/test.swf', function(req, res){
	res.sendFile(__dirname + '/test.swf');
});

app.get('/', function(req, res){
	res.sendFile(__dirname + '/index.html');
});

app.get('/platformer/customLevel.txt', function(req, res){
	res.sendFile(__dirname + '/customLevel.txt');
});

app.listen('80');