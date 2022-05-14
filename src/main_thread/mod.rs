use std::sync::{Mutex};
use wasm_bindgen::prelude::*;
//use wasm_bindgen::__rt::WasmRefCell;

extern crate console_error_panic_hook;

// Needs to be a static mut because it's going to store references to callbacks
// These callbacks will be set by this thread, but executed by another.

// The alternative is passing function pointers to JS and back, but I think
// that would be less save, since ownership would be lost and data could
// be overwritten.

// BUG: Recursive mutex causes panic.
// Maybe switch to RwLock?
static mut WEB_WORKER:Option<Mutex<WebWorkerSqlite>> = None;


// The browser will run this function on its main thread.
#[wasm_bindgen]
pub fn main_thread() {

	// If Rust panics, show it as console.error 
	console_error_panic_hook::set_once();

	unsafe {

		// Initalize the web worker callback structure
		WEB_WORKER = Some(Mutex::new(WebWorkerSqlite::new()));

		// Make some test calls to the web worker.
		// This really needs wrapped in a macro or something -
		// can't expect anyone to write lots of code like this.
		let mut con = WEB_WORKER.as_ref().unwrap().lock().unwrap();
		con.execute("
		CREATE TABLE test (
			id INTEGER PRIMARY KEY,
			name TEXT NOT NULL
		);",sql_executed_cb);
		
	};
}

fn sql_executed_cb(_rows_changed:u32){
	unsafe {
		let mut con = WEB_WORKER.as_ref().unwrap().lock().unwrap();
		con.query("SELECT * from test",sql_query_cb);
	}
}

fn sql_query_cb(rows:Vec<Vec<&str>>){
	browser_dbg(format!("{:}",rows[0][0]));
}

// Temporary, just using to to console.log things.
#[wasm_bindgen(js_namespace = window, js_name = browser_dbg)]
extern { pub fn browser_dbg(s:String); }

// These functions will pass an action and a command to the web worker
#[wasm_bindgen(js_namespace = window, js_name = sqlite)]
extern { pub fn sqlite(action:&str,command:&str); }


// Storing seperate callbacks for each type of data that can be returned
struct WebWorkerSqlite {
	query_callback:Option<fn(Vec<Vec<&str>>)>,
	execute_callback:Option<fn(u32)>,
}

impl WebWorkerSqlite {
	fn new() -> Self {
		Self {
			query_callback:None,
			execute_callback:None,
		}
	}
	fn execute(
		&mut self,
		command:&str,
		f: fn(u32)
	){
		self.execute_callback = Some(f);
		// rust-analyzer thinks this is unsafe, but compiler says it's safe
		sqlite("execute",command);
	}
	fn query(
		&mut self,
		command:&str,
		f: fn(Vec<Vec<&str>>)
	){
		self.query_callback = Some(f);
		// rust-analyzer thinks this is unsafe, but compiler says it's safe
		sqlite("query",command);
	}
} 


#[wasm_bindgen]
pub fn callback_query(data:String) {

	let ww = unsafe { WEB_WORKER.as_ref().unwrap() };


	let ret:Vec<Vec<&str>> = data.split("\n")
		.map(|row|row.split("\t").collect()	)
	.collect();

	ww.lock().unwrap().query_callback.as_ref().unwrap()(
		ret
	);
}

#[wasm_bindgen]
pub fn callback_execute(data:u32) {

	let ww = unsafe { WEB_WORKER.as_ref().unwrap() };

	ww.lock().unwrap().execute_callback.as_ref().unwrap()(
		data.clone()
	);
}