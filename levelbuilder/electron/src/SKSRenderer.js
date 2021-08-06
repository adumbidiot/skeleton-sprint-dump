const BLOCK_ROW_SIZE = 32;
const BLOCK_COLUMN_SIZE = 18;
const BLOCK_NUM = BLOCK_COLUMN_SIZE * BLOCK_ROW_SIZE;
const BLOCK_SIZE_PX = 60;
const CANVAS_WIDTH = 1920;
const CANVAS_HEIGHT = 1080;
const CANVAS_FPS = 60;

export default class SKSRenderer{
	constructor({canvas}){
		this.canvas = canvas;
		this.ctx = this.canvas.getContext('2d');
		this.stateArray = new Array(BLOCK_NUM);
		this.blockDefs = new Map();
		this.history = [];
		this.start();
		this.defaultBG = '';
		this.backgroundList = [];
	}
	loop(){
		this.clearScreen();
		this.update();
		this.render();
		//requestAnimationFrame(this.loop.bind(this));
	}
	clearScreen(){
		this.ctx.clearRect(0, 0, this.canvas.width, this.canvas.height);
	}
	update(){
		
	}
	render(){
		let renderedBG = false;
		
		for(let i = 0; i != this.backgroundList.length && !renderedBG; i++){
			if(this.stateArray[i].includes(this.backgroundList[i]) && this.blockDefs.has(this.backgroundList[i])){
				let img = this.blockDefs.get(this.backgroundList[i]);
				this.ctx.drawImage(img, 0, 0, img.width, img.height);
				renderedBG = true;
			}
		}
		
		if(!renderedBG && this.blockDefs.has(this.defaultBG)){
			let img = this.blockDefs.get(this.defaultBG);
			this.ctx.drawImage(img, 0, 0, img.width, img.height);
		}
		
		for(let i = 0; i != BLOCK_NUM; i++){
			if(this.stateArray[i] === '00') continue;
			
			if(this.blockDefs.has(this.stateArray[i])){
				let x = (i % BLOCK_ROW_SIZE) * BLOCK_SIZE_PX;
				let y = ((i / BLOCK_ROW_SIZE) | 0) * BLOCK_SIZE_PX;
				let img = this.blockDefs.get(this.stateArray[i]);
				this.ctx.drawImage(img, x, y, img.width, img.height);
			}else{
				console.warn(`Key "${this.stateArray[i]}" is not recognized. Overwriting to "00"`);
				this.stateArray[i] = '00';
			}
		}
	}
	start(){
		//requestAnimationFrame(this.loop.bind(this));
		setInterval(this.loop.bind(this), 1000/CANVAS_FPS);
	}
	setDef(key, value, isBackground){
		const img = !isBackground ? (new Image(BLOCK_SIZE_PX, BLOCK_SIZE_PX)) : (new Image(CANVAS_WIDTH, CANVAS_HEIGHT));
		img.src = value;
		this.blockDefs.set(key, img);
		return new Promise((resolve, reject) => {
			img.onload = () => {
				if(isBackground) this.backgroundList.push(key);
				let resizedImg = document.createElement('canvas');
				resizedImg.width = img.width;
				resizedImg.height = img.height;
				resizedImg.getContext('2d').drawImage(img, 0, 0, img.width, img.height);
				this.blockDefs.set(key, resizedImg);
				resolve(img);
			}
		});
	}
	setDefaultbackground(key){
		this.defaultBG = key;
	}
	write(index, data){
		this.history.push({index, data: this.stateArray[index]});
		this.stateArray[index] = data;
	}
	writeChunk(index, arr){
		for(let i = 0; i < BLOCK_NUM && i < arr.length; i++){
			this.write(i + index, arr[i]);
		}
	}
}