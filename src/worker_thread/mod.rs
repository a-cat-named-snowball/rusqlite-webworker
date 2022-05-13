use rusqlite::{params, Connection, Result};
use wasm_bindgen::prelude::*;
use wasm_bindgen::__rt::WasmRefCell;


struct SqlContext {
	conn:Connection,
	//output:Option<String>,
}

impl SqlContext {
	fn new() -> Self {
		Self {
			conn:Connection::open_in_memory().unwrap(),
			//output:Some("test output".to_owned()),
		}
	}

	fn to_pointer(self) -> u32 {
		Box::into_raw(Box::new(WasmRefCell::new(self))) as u32
	}
}


fn pointer_to_context(pointer:u32) -> &'static WasmRefCell<SqlContext> {
	let context = pointer as *mut WasmRefCell<SqlContext>;
	wasm_bindgen::__rt::assert_not_null(context);
	unsafe { &*context }
}


// Returns a ref to data that will need to be shared
#[wasm_bindgen]
pub fn worker_thread(name: &str) -> u32 {

	let context = SqlContext::new();
	context.to_pointer()
}

#[wasm_bindgen]
pub fn execute(context_pointer: u32, command: &str) -> usize {
	let context = pointer_to_context(context_pointer);
	let conn = &context.borrow().conn;
	conn.execute(command,[]).unwrap()
}

//TODO: Return some representation of rows

#[wasm_bindgen]
pub fn query(context_pointer:u32, command: &str) -> String {
	// let context = pointer_to_context(context_pointer);
	// let conn = &context.borrow().conn;
	// let mut stmt = conn.prepare(command).unwrap();
	// let rows = stmt.query([]).unwrap();

	//Box::into_raw(Box::new(WasmRefCell::new(self))) as u32
	//let test_out = vec![vec!["test value".to_owned();5];10];
	//test_out.iter().map(|v|v.join("\t")).collect::<Vec<String>>().iter().join(|r|r.join("\n"));

	return "a\tb\tc\na\tb\tc\nd\te\tf\nd\te\tf".to_owned()
}
