use std::sync::{Mutex,MutexGuard};
use wasm_bindgen::prelude::*;
use wasm_bindgen::__rt::WasmRefCell;

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

	// When Rust panics, show it as console.error 
	console_error_panic_hook::set_once();

	// Initalize the web worker callback structure
	unsafe { WEB_WORKER = Some(Mutex::new(WebWorkerSqlite::new())); }

	// Make some test calls to the web worker.
	// This really needs wrapped in a macro or something -
	// can't expect anyone to write lots of code like this.
	unsafe {
		let mut con = WEB_WORKER.as_ref().unwrap().lock().unwrap();
		con.execute("
		CREATE TABLE test (
			id INTEGER PRIMARY KEY,
			name TEXT NOT NULL
		);",Box::new(|_|{
			con.query("SELECT * from test",Box::new(|rows:Vec<String>|{
				browser_dbg(format!("{:}",rows[0]));
			}));
		}));

	};
}

// Temporary, just using to to console.log things.
#[wasm_bindgen(js_namespace = window, js_name = browser_dbg)]
extern { pub fn browser_dbg(s:String); }

// These functions will pass an action and a command to the web worker
#[wasm_bindgen(js_namespace = window, js_name = sqlite)]
extern { pub fn sqlite(action:&str,command:&str); }


// Storing seperate callbacks for each type of data that can be returned
struct WebWorkerSqlite {
	query_callback:Option<Box<dyn Fn(Vec<String>)>>,
	execute_callback:Option<Box<dyn Fn(u32)>>,
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
		f: Box<dyn Fn(u32)>
	){
		self.execute_callback = Some(f);
		unsafe {
			// rust-analyzer can't decide if this is safe or unsafe.
			// It compiles either way.
			sqlite("execute",command);
		}
	}
	fn query(
		&mut self,
		command:&str,
		f: Box<dyn Fn(Vec<String>)>
	){
		self.query_callback = Some(f);
		unsafe {
			// rust-analyzer can't decide if this is safe or unsafe.
			// It compiles either way.
			sqlite("query",command);
		}
	}
} 


#[wasm_bindgen]
pub fn callback_query(data:String) {

	let mut ww = unsafe { WEB_WORKER.as_ref().unwrap() };

	ww.lock().unwrap().query_callback.as_ref().unwrap()(
		vec![data;1]
	);
}

#[wasm_bindgen]
pub fn callback_execute(data:u32) {

	let mut ww = unsafe { WEB_WORKER.as_ref().unwrap() };

	ww.lock().unwrap().execute_callback.as_ref().unwrap()(
		data.clone()
	);
}