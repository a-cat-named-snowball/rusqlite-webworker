use rusqlite::{Connection};
use wasm_bindgen::prelude::*;
//use wasm_bindgen::__rt::WasmRefCell;
use std::sync::{Mutex};

extern crate console_error_panic_hook;


// Needs to be a static mut because we lose don't have ownership after
// worker_thread_init is done executing, but we need mutable access to it in the
// query and execute functions that are going to be called by javascript.
static mut CONTEXT:Option<Mutex<SqlContext>> = None;

// Using a struct so more information can added later if needed
struct SqlContext {
	conn:Connection,
}

#[wasm_bindgen]
pub fn worker_thread_init(){

	// If Rust panics, show it as console.error 
	console_error_panic_hook::set_once();

	unsafe {
		matches!(CONTEXT,None);
		CONTEXT = Some( Mutex::new( SqlContext {
			conn:Connection::open_in_memory().unwrap()
		}));
	}
}

//TODO: Handle sql errors in a better way than panicking
#[wasm_bindgen]
pub fn execute(command: &str) -> usize {
	unsafe {
		let lock = CONTEXT.as_ref()
			.expect("worker_thread_init() should have been called first")
			.lock().unwrap();

		let conn = &lock.conn;
		conn.execute(command,[]).unwrap()
	}
}

//TODO: Handle sql errors in a better way than panicking
#[wasm_bindgen]
pub fn query(command: &str) -> String {
	unsafe {
		let lock = CONTEXT.as_ref()
			.expect("worker_thread_init() should have been called first")
			.lock().unwrap();

		let conn = &lock.conn;

		let mut stmt = conn.prepare(command).unwrap();
		
		// Here we run the query and convert the result into a string.
		// Columns are separated with \t, rows separated with \n.
		stmt.query_map([],|row|{
			let mut elements:Vec<String> = Vec::new();
			let mut i = 0;
			loop {
				let column = row.get(i);
				match column {
					Err(_)  => break,
					Ok(col) => elements.push(col),
				}
				i += 1;
			}
			Ok(elements.join("\t"))
		})
		.unwrap()
		.flatten()
		.collect::<Vec<String>>()
		.join("\n")
	}

}


#[wasm_bindgen]
pub fn test(command: &str) -> String {
	let parsed_value = command.parse::<u32>().unwrap();
	return format!("{:}",parsed_value + 1);
}