import init, {execute,query} from "/pkg/rusqlite_webworker.js"


//TODO: clean up and implement init and exec commands

self.addEventListener('message', e => {

	init().then(()=>{
		// Run a test command to make sure everything is working
		if(e.data.action === "query") {
			let output = query(e.data.pointer,e.data.command)
			postMessage(output);
		}
		else if(e.data.action === "execute") {
			let output = execute(e.data.pointer,e.data.command)
			postMessage(output);
		}
	})
})
