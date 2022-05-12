
use wasm_bindgen::prelude::*;
use wasm_bindgen::__rt::WasmRefCell;

#[wasm_bindgen(js_namespace = window, js_name = sqlite)]
extern { pub fn sqlite(s: &str); }


// Main thread in browser will insert here
#[wasm_bindgen]
pub fn main_thread() {
	//Dummy use case
	sqlite("insert a into table");

	WebWorker::connect(|conn|{
		conn.execute("SELECT * from data;",|rows|{
			println!("{:}",rows.get(0)?);
		});
	});
	
}


struct WebWorker {
	callback:Box<dyn Fn(u32)>,
}

impl WebWorker {
	fn connect(
		f: Box<dyn Fn(u32)>
	) -> Self {
		Self {
			callback:f,
		}
	}
} 