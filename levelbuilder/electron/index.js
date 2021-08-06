const {app, BrowserWindow} = require('electron');
const defaultWidth = 1920;
const defaultHeight = 1080;
let mainWindow = null;

function spawnWindow(){
	mainWindow = new BrowserWindow({width: defaultWidth, height: defaultHeight});
	mainWindow.loadFile('index.html');
	mainWindow.on('closed', function (){
		mainWindow = null;
	});
}

app.on('ready', spawnWindow);

app.on('window-all-closed', function (){
	if (process.platform !== 'darwin'){
		app.quit();
	}
});

app.on('activate', function (){
	if (mainWindow === null) {
		spawnWindow();
	}
});