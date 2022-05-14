use rusqlite::{params, Connection, Result};
use wasm_bindgen::prelude::*;
use wasm_bindgen::__rt::WasmRefCell;
use std::sync::{Mutex};

extern crate console_error_panic_hook;

// Using a struct here so more information can added later
struct SqlContext {
	conn:Option<Connection>,
}
impl SqlContext {
	const fn new() -> Self {
		Self {
			conn:None,
		}
	}
}


static mut context:Option<Mutex<SqlContext>> = None;

#[wasm_bindgen]
pub fn worker_thread_init() -> u32 {

	// When Rust panics, show it as console.error 
	console_error_panic_hook::set_once();

	unsafe {
		matches!(context,None);
		context = Some(Mutex::new(SqlContext::new()));
		let mut lock = context.as_ref().unwrap().lock().unwrap();
		lock.conn = Connection::open_in_memory().ok();
	}
	return 12;
}

#[wasm_bindgen]
pub fn execute(command: &str) -> usize {
	unsafe {
		let lock = context.as_ref().unwrap().lock().unwrap();
		let conn = lock.conn.as_ref().unwrap();
		conn.execute(command,[]).unwrap()
	}
}

//TODO: Return some representation of rows

#[wasm_bindgen]
pub fn query(command: &str) -> String {
	unsafe {
		let lock = context.as_ref().unwrap().lock().unwrap();
		let conn = lock.conn.as_ref().unwrap();

		let mut stmt = conn.prepare(command).unwrap();
		let rows = stmt.query([]).unwrap();
		//rows
	}

	"a\tb\tc\na\tb\tc\nd\te\tf\nd\te\tf".to_owned()
}
