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
	let callback_type = {
		query:callback_query,
		execute:callback_execute,
		test:callback_test
	}[e.data.action]

	callback_type(e.data.output)
})


// Used by wasm code to log things to console.log
window.browser_dbg = (a) => {
	console.log(a)
}

// Wasm code will call this it wants to perform an sql query
window.sqlite = (action,command) => {
	worker.postMessage({action,command})
}

// Start executing wasm code
init().then(() => main_thread())

