import init, {execute,query} from "/pkg/rusqlite_webworker.js"


//TODO: clean up and implement init and exec commands

self.addEventListener('message', e => {

	init().then(()=>{
		// Run a test command to make sure everything is working
		if(e.data.action === "query") {
			e.data.output = query(e.data.pointer,e.data.command)
			postMessage(e.data);
		}
		else if(e.data.action === "execute") {
			e.data.output = execute(e.data.pointer,e.data.command)
			postMessage(e.data);
		}
	})
})
