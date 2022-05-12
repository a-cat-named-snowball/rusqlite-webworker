import init, {main_thread,worker_thread,execute} from "/pkg/rusqlite_webworker.js"


const worker = new Worker('worker.js', { type: "module" })


window.sqlite = (data) => {
	worker.postMessage({
		command:data,
		params:[data]
	})
}


async function start() {
	await init()
	
	worker.addEventListener("message", e=> {
		console.log(e)
	})

	main_thread()

	const context = worker_thread('some sql command')
	console.log("context",context)
	const out = execute(context,"asd123")
	console.log("out",out)
}

start().then()