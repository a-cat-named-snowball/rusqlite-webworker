
use wasm_bindgen::prelude::*;
use wasm_bindgen::__rt::WasmRefCell;



#[wasm_bindgen(js_namespace = window, js_name = browser_dbg)]
extern { pub fn browser_dbg(s:String); }

// Main thread in browser will insert here
#[wasm_bindgen]
pub fn main_thread() {

	// Get a reference 
	let mut ww_sqlite = WebWorkerSqlite::new();

	// ww_sqlite.execute("SELECT * from data;",|rows:Vec<String>|{
	// 	println!("{:}",rows[0]);
	// });
	

	//When uncommented, browser gives error:
	// Uncaught TypeError: Failed to resolve module specifier "env". Relative references must start with either "/", "./", or "../".

	browser_dbg("test browser degub".to_owned());

	ww_sqlite.query("SELECT * from data",|rows:Vec<String>|{
		unsafe {
			browser_dbg(format!("{:}",rows[0]));
			browser_dbg("!!!".to_owned());
		}
	});
	
}


#[wasm_bindgen(js_namespace = window, js_name = sqlite_execute)]
extern { pub fn sqlite_execute(pointer:u32,command:&str); }
#[wasm_bindgen(js_namespace = window, js_name = sqlite_query)]
extern { pub fn sqlite_query(pointer:u32,command:&str); }

struct WebWorkerSqlite {
	query_callback:Option<Box<dyn Fn(Vec<String>)>>,
	execute_callback:Option<Box<dyn Fn(u32)>>,
	returned_rows:Vec<Vec<String>>,
}

impl WebWorkerSqlite {
	fn new() -> Self {
		Self {
			query_callback:None,
			execute_callback:None,
			returned_rows:Vec::new(),
		}
	}

	fn execute(&mut self,command:&str,f: fn(u32)){
		self.execute_callback = Some(Box::new(f));
		unsafe { sqlite_execute(self.get_pointer(),command); }
	}

	fn query(&mut self,command:&str,f: fn(Vec<String>)){
		self.query_callback = Some(Box::new(f));
		unsafe { sqlite_query(self.get_pointer(),command); }
	}

	fn get_pointer(&self) -> u32 {
		Box::into_raw(Box::new(WasmRefCell::new(self))) as u32
	}

} 


#[wasm_bindgen]
pub fn callback_query(pointer:u32,data:String) {

	browser_dbg("callback received".to_owned());

	let web_worker = pointer as *mut WasmRefCell<WebWorkerSqlite>;
	wasm_bindgen::__rt::assert_not_null(web_worker);
	let web_worker = unsafe { &*web_worker };

	browser_dbg(data.clone());

	web_worker.borrow().query_callback.as_ref().unwrap()(vec![data;1]);

}