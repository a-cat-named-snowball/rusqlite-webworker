import init, {worker_thread_init,execute,query,test} from "/pkg/rusqlite_webworker.js"

function log(s){
	console.log(s)
}

//TODO: clean up and implement init and exec commands

self.addEventListener('message', e => {

	init().then(()=>{
		console.log("Webworker stareted")
		worker_thread_init()

		// Run a test command to make sure everything is working
		if(e.data.action === "query") {
			e.data.output = query(e.data.command)
			postMessage(e.data);
		}
		else if(e.data.action === "execute") {
			e.data.output = execute(e.data.command)
			postMessage(e.data);
		}
		else if(e.data.action === "test") {
			e.data.output = test(e.data.command)
			postMessage(e.data);
		}
	})
})
