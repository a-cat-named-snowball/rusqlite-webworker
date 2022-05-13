import init, {main_thread} from "/pkg/rusqlite_webworker.js"

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


async function start() {
	await init()
	console.log("starting")
	
	worker.addEventListener("message", e=> {
		console.log(e)
	})

	main_thread()


	//Testing, will be moved to webworker
	// const context = worker_thread('some sql command')
	// console.log("context",context)
	// const out = execute(context,"asd123")
	// console.log("out",out)

}

start().then()