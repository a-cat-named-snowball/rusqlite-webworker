import init, {
	main_thread,
	callback_query,
	callback_execute,
	callback_test,
} from "/pkg/rusqlite_webworker.js"


//Start the web worker
const worker = new Worker('worker.js', { type: "module" })


// Get messages back from the web worker and pass the data back to wasm code
worker.addEventListener("message", e=> {
	console.log(e)

	if(e.data.action==="query"){
		callback_query(e.data.output)
	}
	else if(e.data.action==="execute"){
		callback_execute(e.data.output)
	}
	else if(e.data.action==="test"){
		callback_test(e.data.output)
	}
})




window.browser_dbg = (a) => {
	console.log(a)
}

// Functions that wasm code will call when it wants to perform an sql query
window.sqlite = (action,command) => {
	worker.postMessage({action,command})
}

// Start executing wasm code
init().then(() => main_thread())

// async function start() {
// 	await init()
// 	main_thread()
// }
// start().then()

