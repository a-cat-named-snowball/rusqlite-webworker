use rusqlite::{params, Connection, Result};
use wasm_bindgen::prelude::*;
use wasm_bindgen::__rt::WasmRefCell;

#[wasm_bindgen(js_namespace = window, js_name = sqlite)]
extern { pub fn sqlite(s: &str); }


// Main thread in browser will insert here
#[wasm_bindgen]
pub fn main_thread() {
	//Dummy use case
	sqlite("insert a into table");
}


struct SqlContext {
	conn:Option<Connection>,
	output:Option<String>,
}


// TODO: merge worker_thread function with execute function

// Returns a ref to data that will need to be shared
#[wasm_bindgen]
pub fn worker_thread(name: &str) -> u32 {

	let context = SqlContext {
		conn:Some(Connection::open_in_memory().unwrap()),
		output:Some("test output".to_owned()),
	};

	Box::into_raw(Box::new(WasmRefCell::new(context))) as u32
}

//TODO: execute some sql command on the connection.
#[wasm_bindgen]
pub fn execute(context:u32, command :&str) -> String {
	let context = context as *mut WasmRefCell<SqlContext>;
	wasm_bindgen::__rt::assert_not_null(context);
	let context = unsafe { &*context };
	let out_string = &context.clone().borrow().output;
	return out_string.clone().unwrap();
}
