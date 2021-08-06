'use strict';

function noop() {}

function assign(tar, src) {
	for (var k in src) tar[k] = src[k];
	return tar;
}

function assignTrue(tar, src) {
	for (var k in src) tar[k] = 1;
	return tar;
}

function appendNode(node, target) {
	target.appendChild(node);
}

function insertNode(node, target, anchor) {
	target.insertBefore(node, anchor);
}

function detachNode(node) {
	node.parentNode.removeChild(node);
}

function createElement(name) {
	return document.createElement(name);
}

function createText(data) {
	return document.createTextNode(data);
}

function addListener(node, event, handler) {
	node.addEventListener(event, handler, false);
}

function removeListener(node, event, handler) {
	node.removeEventListener(event, handler, false);
}

function setStyle(node, key, value) {
	node.style.setProperty(key, value);
}

function blankObject() {
	return Object.create(null);
}

function destroy(detach) {
	this.destroy = noop;
	this.fire('destroy');
	this.set = noop;

	this._fragment.d(detach !== false);
	this._fragment = null;
	this._state = {};
}

function _differs(a, b) {
	return a != a ? b == b : a !== b || ((a && typeof a === 'object') || typeof a === 'function');
}

function _differsImmutable(a, b) {
	return a != a ? b == b : a !== b;
}

function fire(eventName, data) {
	var handlers =
		eventName in this._handlers && this._handlers[eventName].slice();
	if (!handlers) return;

	for (var i = 0; i < handlers.length; i += 1) {
		var handler = handlers[i];

		if (!handler.__calling) {
			handler.__calling = true;
			handler.call(this, data);
			handler.__calling = false;
		}
	}
}

function get() {
	return this._state;
}

function init(component, options) {
	component._handlers = blankObject();
	component._bind = options._bind;

	component.options = options;
	component.root = options.root || component;
	component.store = component.root.store || options.store;
}

function on(eventName, handler) {
	var handlers = this._handlers[eventName] || (this._handlers[eventName] = []);
	handlers.push(handler);

	return {
		cancel: function() {
			var index = handlers.indexOf(handler);
			if (~index) handlers.splice(index, 1);
		}
	};
}

function set(newState) {
	this._set(assign({}, newState));
	if (this.root._lock) return;
	this.root._lock = true;
	callAll(this.root._beforecreate);
	callAll(this.root._oncreate);
	callAll(this.root._aftercreate);
	this.root._lock = false;
}

function _set(newState) {
	var oldState = this._state,
		changed = {},
		dirty = false;

	for (var key in newState) {
		if (this._differs(newState[key], oldState[key])) changed[key] = dirty = true;
	}
	if (!dirty) return;

	this._state = assign(assign({}, oldState), newState);
	this._recompute(changed, this._state);
	if (this._bind) this._bind(changed, this._state);

	if (this._fragment) {
		this.fire("state", { changed: changed, current: this._state, previous: oldState });
		this._fragment.p(changed, this._state);
		this.fire("update", { changed: changed, current: this._state, previous: oldState });
	}
}

function callAll(fns) {
	while (fns && fns.length) fns.shift()();
}

function _mount(target, anchor) {
	this._fragment[this._fragment.i ? 'i' : 'm'](target, anchor || null);
}

var proto = {
	destroy,
	get,
	fire,
	on,
	set,
	_recompute: noop,
	_set,
	_mount,
	_differs
};

function Store(state, options) {
	this._handlers = {};
	this._dependents = [];

	this._computed = blankObject();
	this._sortedComputedProperties = [];

	this._state = assign({}, state);
	this._differs = options && options.immutable ? _differsImmutable : _differs;
}

assign(Store.prototype, {
	_add(component, props) {
		this._dependents.push({
			component: component,
			props: props
		});
	},

	_init(props) {
		const state = {};
		for (let i = 0; i < props.length; i += 1) {
			const prop = props[i];
			state['$' + prop] = this._state[prop];
		}
		return state;
	},

	_remove(component) {
		let i = this._dependents.length;
		while (i--) {
			if (this._dependents[i].component === component) {
				this._dependents.splice(i, 1);
				return;
			}
		}
	},

	_set(newState, changed) {
		const previous = this._state;
		this._state = assign(assign({}, previous), newState);

		for (let i = 0; i < this._sortedComputedProperties.length; i += 1) {
			this._sortedComputedProperties[i].update(this._state, changed);
		}

		this.fire('state', {
			changed,
			previous,
			current: this._state
		});

		const dependents = this._dependents.slice(); // guard against mutations
		for (let i = 0; i < dependents.length; i += 1) {
			const dependent = dependents[i];
			const componentState = {};
			let dirty = false;

			for (let j = 0; j < dependent.props.length; j += 1) {
				const prop = dependent.props[j];
				if (prop in changed) {
					componentState['$' + prop] = this._state[prop];
					dirty = true;
				}
			}

			if (dirty) dependent.component.set(componentState);
		}

		this.fire('update', {
			changed,
			previous,
			current: this._state
		});
	},

	_sortComputedProperties() {
		const computed = this._computed;
		const sorted = this._sortedComputedProperties = [];
		const visited = blankObject();
		let currentKey;

		function visit(key) {
			const c = computed[key];

			if (c) {
				c.deps.forEach(dep => {
					if (dep === currentKey) {
						throw new Error(`Cyclical dependency detected between ${dep} <-> ${key}`);
					}

					visit(dep);
				});

				if (!visited[key]) {
					visited[key] = true;
					sorted.push(c);
				}
			}
		}

		for (const key in this._computed) {
			visit(currentKey = key);
		}
	},

	compute(key, deps, fn) {
		let value;

		const c = {
			deps,
			update: (state, changed, dirty) => {
				const values = deps.map(dep => {
					if (dep in changed) dirty = true;
					return state[dep];
				});

				if (dirty) {
					const newValue = fn.apply(null, values);
					if (this._differs(newValue, value)) {
						value = newValue;
						changed[key] = true;
						state[key] = value;
					}
				}
			}
		};

		this._computed[key] = c;
		this._sortComputedProperties();

		const state = assign({}, this._state);
		const changed = {};
		c.update(state, changed, true);
		this._set(state, changed);
	},

	fire,

	get,

	on,

	set(newState) {
		const oldState = this._state;
		const changed = this._changed = {};
		let dirty = false;

		for (const key in newState) {
			if (this._computed[key]) throw new Error(`'${key}' is a read-only property`);
			if (this._differs(newState[key], oldState[key])) changed[key] = dirty = true;
		}
		if (!dirty) return;

		this._set(newState, changed);
	}
});

var store = new Store();

/* src\sks-nav.html generated by Svelte v2.7.2 */

function add_css() {
	var style = createElement("style");
	style.id = 'svelte-1bf71pv-style';
	style.textContent = ".main.svelte-1bf71pv{height:3rem;background-color:#383838;width:100%;user-select:none;overflow:hidden;cursor:default}.logo.svelte-1bf71pv{width:2.5rem;font-size:2.5rem;margin:.25rem}.item.svelte-1bf71pv{color:white;text-align:center;height:2.5rem;position:relative;float:left;margin:.25rem;margin-left:0.4rem;display:flex;justify-content:center;align-content:center;flex-direction:column}button.svelte-1bf71pv{background-color:#383838;border:0px;outline:none;cursor:pointer}";
	appendNode(style, document.head);
}

function create_main_fragment(component, ctx) {
	var div, div_1, text_1, button, text_3, button_1, text_5, button_2, text_7, button_3;

	function click_handler(event) {
		component.fire('file', {});
	}

	return {
		c() {
			div = createElement("div");
			div_1 = createElement("div");
			div_1.textContent = "SS";
			text_1 = createText("\r\n\t");
			button = createElement("button");
			button.textContent = "File";
			text_3 = createText("\r\n\t");
			button_1 = createElement("button");
			button_1.textContent = "Edit";
			text_5 = createText("\r\n\t");
			button_2 = createElement("button");
			button_2.textContent = "Template";
			text_7 = createText("\r\n\t");
			button_3 = createElement("button");
			button_3.textContent = "Help";
			div_1.className = "logo item svelte-1bf71pv";
			addListener(button, "click", click_handler);
			button.className = "item svelte-1bf71pv";
			button_1.className = "item svelte-1bf71pv";
			button_2.className = "item svelte-1bf71pv";
			button_3.className = "item svelte-1bf71pv";
			div.className = "main svelte-1bf71pv";
		},

		m(target, anchor) {
			insertNode(div, target, anchor);
			appendNode(div_1, div);
			appendNode(text_1, div);
			appendNode(button, div);
			appendNode(text_3, div);
			appendNode(button_1, div);
			appendNode(text_5, div);
			appendNode(button_2, div);
			appendNode(text_7, div);
			appendNode(button_3, div);
		},

		p: noop,

		d(detach) {
			if (detach) {
				detachNode(div);
			}

			removeListener(button, "click", click_handler);
		}
	};
}

function Sks_nav(options) {
	init(this, options);
	this._state = assign({}, options.data);
	this._intro = true;

	if (!document.getElementById("svelte-1bf71pv-style")) add_css();

	this._fragment = create_main_fragment(this, this._state);

	if (options.target) {
		this._fragment.c();
		this._mount(options.target, options.anchor);
	}
}

assign(Sks_nav.prototype, proto);

const BLOCK_ROW_SIZE = 32;
const BLOCK_COLUMN_SIZE = 18;
const BLOCK_NUM = BLOCK_COLUMN_SIZE * BLOCK_ROW_SIZE;
const BLOCK_SIZE_PX = 60;
const CANVAS_WIDTH = 1920;
const CANVAS_HEIGHT = 1080;
const CANVAS_FPS = 20;

class SKSRenderer{
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
			};
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

/* src\sks-canvas.html generated by Svelte v2.7.2 */

var methods = {
	onmousedown(e){
		console.log('down');
	},
	onmouseup(e){
		console.log('up');
	},
	onmouseleave(){
		console.log('lev'); //Consider "up"
	},
	getRenderer(){
		const renderer = new SKSRenderer({
			canvas: this.refs.canvas
		});
		renderer.setDef('B0', 'assets/block.png');
		renderer.setDef('M0', 'assets/background.png', true);
		
		let arr = [];
		for(let i = 0; i != 32 * 18; i++){
			arr.push('00');
		}
		arr[0] = 'M0';
		renderer.writeChunk(0, arr);
		return renderer;
	}
};

function oncreate(){
	const renderer = this.getRenderer();
	
	
	
	this.set({renderer});
}
function add_css$1() {
	var style = createElement("style");
	style.id = 'svelte-12qbgvn-style';
	style.textContent = ".main.svelte-12qbgvn{background-color:#777777;height:100%;position:relative;display:inline-flex;flex-grow:9}";
	appendNode(style, document.head);
}

function create_main_fragment$1(component, ctx) {
	var div, canvas;

	function mousedown_handler(event) {
		component.onmousedown(event);
	}

	function mouseup_handler(event) {
		component.onmouseup(event);
	}

	function mouseleave_handler(event) {
		component.onmouseleave(event);
	}

	return {
		c() {
			div = createElement("div");
			canvas = createElement("canvas");
			canvas.width = "1920";
			canvas.height = "1080";
			setStyle(canvas, "width", "800px");
			setStyle(canvas, "height", "450px");
			addListener(div, "mousedown", mousedown_handler);
			addListener(div, "mouseup", mouseup_handler);
			addListener(div, "mouseleave", mouseleave_handler);
			div.className = "main svelte-12qbgvn";
		},

		m(target, anchor) {
			insertNode(div, target, anchor);
			appendNode(canvas, div);
			component.refs.canvas = canvas;
		},

		p: noop,

		d(detach) {
			if (detach) {
				detachNode(div);
			}

			if (component.refs.canvas === canvas) component.refs.canvas = null;
			removeListener(div, "mousedown", mousedown_handler);
			removeListener(div, "mouseup", mouseup_handler);
			removeListener(div, "mouseleave", mouseleave_handler);
		}
	};
}

function Sks_canvas(options) {
	init(this, options);
	this.refs = {};
	this._state = assign({}, options.data);
	this._intro = true;

	if (!document.getElementById("svelte-12qbgvn-style")) add_css$1();

	if (!options.root) {
		this._oncreate = [];
	}

	this._fragment = create_main_fragment$1(this, this._state);

	this.root._oncreate.push(() => {
		oncreate.call(this);
		this.fire("update", { changed: assignTrue({}, this._state), current: this._state });
	});

	if (options.target) {
		this._fragment.c();
		this._mount(options.target, options.anchor);

		callAll(this._oncreate);
	}
}

assign(Sks_canvas.prototype, proto);
assign(Sks_canvas.prototype, methods);

/* src\sks-toolbar.html generated by Svelte v2.7.2 */

function add_css$2() {
	var style = createElement("style");
	style.id = 'svelte-1w4hdg9-style';
	style.textContent = ".main.svelte-1w4hdg9{background-color:#777777;height:100%;width:150px;position:relative;float:left;display:inline-flex;margin-left:1rem}";
	appendNode(style, document.head);
}

function create_main_fragment$2(component, ctx) {
	var div;

	return {
		c() {
			div = createElement("div");
			div.className = "main svelte-1w4hdg9";
		},

		m(target, anchor) {
			insertNode(div, target, anchor);
		},

		p: noop,

		d(detach) {
			if (detach) {
				detachNode(div);
			}
		}
	};
}

function Sks_toolbar(options) {
	init(this, options);
	this._state = assign({}, options.data);
	this._intro = true;

	if (!document.getElementById("svelte-1w4hdg9-style")) add_css$2();

	this._fragment = create_main_fragment$2(this, this._state);

	if (options.target) {
		this._fragment.c();
		this._mount(options.target, options.anchor);
	}
}

assign(Sks_toolbar.prototype, proto);

/* src\sks-main.html generated by Svelte v2.7.2 */

var methods$1 = {
	fileHandler(e){
		console.log('a', e);
	}
};

function store_1() {
	return store;
}

function add_css$3() {
	var style = createElement("style");
	style.id = 'svelte-1j9q1ph-style';
	style.textContent = ".main.svelte-1j9q1ph{width:100%;height:100%;background-color:#000000}.wrapper.svelte-1j9q1ph{left:1rem;right:1rem;top:4rem;bottom:1rem;position:absolute;display:flex}";
	appendNode(style, document.head);
}

function create_main_fragment$3(component, ctx) {
	var div, text, div_1, text_1;

	var sksnav = new Sks_nav({
		root: component.root
	});

	sksnav.on("file", function(event) {
		component.fileHandler(ctx.e);
	});

	var skscanvas = new Sks_canvas({
		root: component.root
	});

	var skstoolbar = new Sks_toolbar({
		root: component.root
	});

	return {
		c() {
			div = createElement("div");
			sksnav._fragment.c();
			text = createText("\r\n\t");
			div_1 = createElement("div");
			skscanvas._fragment.c();
			text_1 = createText("\r\n\t\t");
			skstoolbar._fragment.c();
			div_1.className = "wrapper svelte-1j9q1ph";
			div.className = "main svelte-1j9q1ph";
		},

		m(target, anchor) {
			insertNode(div, target, anchor);
			sksnav._mount(div, null);
			appendNode(text, div);
			appendNode(div_1, div);
			skscanvas._mount(div_1, null);
			appendNode(text_1, div_1);
			skstoolbar._mount(div_1, null);
		},

		p(changed, _ctx) {
			ctx = _ctx;

		},

		d(detach) {
			if (detach) {
				detachNode(div);
			}

			sksnav.destroy();
			skscanvas.destroy();
			skstoolbar.destroy();
		}
	};
}

function Sks_main(options) {
	init(this, options);
	this.store = store_1();
	this._state = assign({}, options.data);
	this._intro = true;

	if (!document.getElementById("svelte-1j9q1ph-style")) add_css$3();

	if (!options.root) {
		this._oncreate = [];
		this._beforecreate = [];
		this._aftercreate = [];
	}

	this._fragment = create_main_fragment$3(this, this._state);

	if (options.target) {
		this._fragment.c();
		this._mount(options.target, options.anchor);

		this._lock = true;
		callAll(this._beforecreate);
		callAll(this._oncreate);
		callAll(this._aftercreate);
		this._lock = false;
	}
}

assign(Sks_main.prototype, proto);
assign(Sks_main.prototype, methods$1);

module.exports = Sks_main;
