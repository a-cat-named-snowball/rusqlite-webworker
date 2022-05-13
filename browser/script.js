import init, {main_thread,callback_query} from "/pkg/rusqlite_webworker.js"

const worker = new Worker('worker.js', { type: "module" })

window.sqlite_execute = (pointer,command) => {
	console.log("EXECUTE:",pointer,command)
	worker.postMessage({
		action:"execute",
		pointer,
		command,
	})
}
window.sqlite_query = (pointer,command) => {
	console.log("QUERY:",pointer,command)
	worker.postMessage({
		action:"query",
		pointer,
		command,
	})
}

window.browser_dbg = (a) => {
	console.log(a)
}


async function start() {
	await init()
	console.log("starting")
	
	worker.addEventListener("message", e=> {
		console.log(e)
		if(e.data.action==="query"){
			callback_query(e.data.pointer,e.data.output)
		}
		// else if(e.data.action==="execute"){
		// 	callback_query(e.data.output)
		// }
	})

	main_thread()


	//Testing, will be moved to webworker
	// const context = worker_thread('some sql command')
	// console.log("context",context)
	// const out = execute(context,"asd123")
	// console.log("out",out)

}

start().then()