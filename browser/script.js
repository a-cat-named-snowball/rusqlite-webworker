import init, {main_thread,callback_query,callback_execute} from "/pkg/rusqlite_webworker.js"

const worker = new Worker('worker.js', { type: "module" })
worker.addEventListener("message", e=> {
	console.log(e)

	if(e.data.action==="query"){
		callback_query(e.data.output)
	}
	else if(e.data.action==="execute"){
		callback_execute(e.data.output)
	}
})

let actions = ["execute","query"]

window.sqlite_execute = command => {
	console.log("EXECUTE:",command)
	worker.postMessage({
		action:"execute",
		command,
	})
}

window.sqlite_query = command => {
	console.log("QUERY:",command)
	worker.postMessage({
		action:"query",
		command,
	})
}

window.browser_dbg = (a) => {
	console.log(a)
}

init().then(() => main_thread())

// async function start() {
// 	await init()
// 	main_thread()
// }
// start().then()

