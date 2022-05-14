import init, {
	worker_thread_init,
	execute,
	query,
	test,
} from "/pkg/rusqlite_webworker.js"

let ready = false
let backlog = []

// Initialize everything, then replay any commands that were given before
init().then(()=>{
	worker_thread_init()
	ready = true
	backlog.forEach(process_message)
})


self.addEventListener('message',process_message)
function process_message(e) {

	if(!ready){
		backlog.push(e)
		return
	}

	// Depending a wasm function depending on the action specified
	// Pass the command data to it as an argument
	// Return the output back to the non-webworker code

	const wasm_function = {query,execute,test}[e.data.action]
	e.data.output = wasm_function(e.data.command)
	postMessage(e.data)
	
}