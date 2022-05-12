import init, {execute} from "/pkg/rusqlite_webworker.js"


//TODO: clean up and implement init and exec commands

self.addEventListener('message', e => {
	init().then(()=>{
		// Run a test command to make sure everything is working
		let output = execute(55,e.data.command)
		postMessage(output);
	})
})
