use wasm_bindgen::prelude::*;
use bytecode_vm::vm::VM;

#[wasm_bindgen]
pub struct WasmVm {
    inner: VM,
}

#[wasm_bindgen]
impl WasmVm {
    #[wasm_bindgen(constructor)]
    pub fn new() -> WasmVm {
        WasmVm { inner: VM::new() }
    }

    pub fn interpret(&mut self, source: &str) -> String {
        match self.inner.interpret(source) {
            true => "good".to_owned(),
            false => "bad".to_owned(),
        }
    }
}
