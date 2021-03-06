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
		
		con.perform_test("3",test_cb);
		fn test_cb(modified_value:&str){
			// Will output "4" to console.log
			browser_dbg(format!("{:}",modified_value));
		}



		// Call out to the web worker to create a table
		con.execute("
		CREATE TABLE person (
			id              INTEGER PRIMARY KEY,
			name            TEXT NOT NULL,
			data            TEXT
		);",sql_executed_cb);

		// Now insert a row
		con.execute(
			format!(
				"INSERT INTO person (id,name, data) VALUES ('{:}','{}', '{}')",
				0,
				"Steven",
				"d"
			).as_str(),
		sql_executed_cb);

		// Another row
		con.execute(
			format!(
				"INSERT INTO person (id,name, data) VALUES ('{:}','{}', '{}')",
				1,
				"Robert",
				"--some-encoded-data--"
			).as_str(),
		sql_executed_cb);
		
		fn sql_executed_cb(rows_changed:u32){
			browser_dbg(format!("rows_changed: {:}",rows_changed));
		}

		// Now select all the rows we inserted and console.log them.
		con.query("SELECT id, name, data FROM person",sql_query_cb);
		fn sql_query_cb(rows:Vec<Vec<&str>>){
			browser_dbg(
				format!("Rows returned:{:?}",rows)
			);
		}

	};
}



// Using this to console.log things.
#[wasm_bindgen(js_namespace = window, js_name = browser_dbg)]
extern { pub fn browser_dbg(s:String); }

// These functions will pass an action and a command to the web worker
#[wasm_bindgen(js_namespace = window, js_name = sqlite)]
extern { pub fn sqlite(action:&str,command:&str); }


// Storing seperate callbacks for each type of data that can be returned
struct WebWorkerSqlite {
	query_callback:Option<fn(Vec<Vec<&str>>)>,
	execute_callback:Option<fn(u32)>,
	test_callback:Option<fn(&str)>, // For the n+1 example
}

impl WebWorkerSqlite {
	fn new() -> Self {
		Self {
			query_callback:None,
			execute_callback:None,
			test_callback:None,
		}
	}
	fn execute(&mut self,command:&str,f: fn(u32)){
		self.execute_callback = Some(f);
		sqlite("execute",command);
	}
	fn query(&mut self,command:&str,f: fn(Vec<Vec<&str>>)){
		self.query_callback = Some(f);
		sqlite("query",command);
	}

	// For the n+1 example
	fn perform_test(&mut self,command:&str,f: fn(&str)){
		self.test_callback = Some(f);
		sqlite("test",command);
	}
} 


#[wasm_bindgen]
pub fn callback_query(data:String) {
	let ww = unsafe { WEB_WORKER.as_ref().unwrap() };

	// Transform data into Vec<Vec<String>> for better usability
	let ret:Vec<Vec<&str>> = data.split("\n")
		.map(|row|row.split("\t").collect()	)
	.collect();

	ww.lock().unwrap().query_callback.as_ref().unwrap()(ret);
}

#[wasm_bindgen]
pub fn callback_execute(data:u32) {
	let ww = unsafe { WEB_WORKER.as_ref().unwrap() };
	ww.lock().unwrap().execute_callback.as_ref().unwrap()(data);
}

#[wasm_bindgen]
pub fn callback_test(data:&str) {
	let ww = unsafe { WEB_WORKER.as_ref().unwrap() };
	ww.lock().unwrap().test_callback.as_ref().unwrap()(data.clone());
}