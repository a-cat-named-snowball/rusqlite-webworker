import init, {greet,execute} from "/pkg/rusqlite_webworker.js"


self.addEventListener('message', e => {
	init().then(()=>{
		let output = execute(55,e.data.command)
		postMessage(output);
	})
})
