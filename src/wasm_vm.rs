use wasm_bindgen::prelude::*;
use bytecode_vm::interpreter::Interpreter;
use bytecode_vm::interpreter::{InterpretResult, CompilerError, RuntimeError};

#[wasm_bindgen]
pub struct WasmVm {

}

#[wasm_bindgen]
#[derive(Clone)]
pub struct CompilerErr {
    pub line: usize,
    pub start: usize,
    pub len: usize
}


#[wasm_bindgen]
pub struct Output {
    success: bool,
    compile_errors: Vec<CompilerErr>,
    runtime_error: String
}

#[wasm_bindgen]
impl Output {
    #[wasm_bindgen(getter)]
    pub fn success(&self) -> bool {
        self.success
    }

    #[wasm_bindgen(getter)]
    pub fn compile_errors(&self) -> Vec<CompilerErr> {
        self.compile_errors.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn runtime_error(&self) -> String {
        self.runtime_error.clone()
    }
}

impl Output {
    pub fn successful() -> Self {
        return Self {
            success: true,
            compile_errors: vec![],
            runtime_error: "".to_owned()
        };
    }
    pub fn runtime_err(err: RuntimeError) -> Self {
        return Self {
            success: false,
            compile_errors: vec![],
            runtime_error: err.message.to_owned()
        };
    }
    pub fn compile_err(errors: Vec<CompilerError>) -> Self {
        return Self {
            success: false,
            compile_errors: errors.iter().map(|i| { CompilerErr {
                line: i.line,
                start: i.start,
                len: i.len
            }}).collect(),
            runtime_error: "".to_owned()
        }
    }
}

#[wasm_bindgen]
impl WasmVm {
    #[wasm_bindgen(constructor)]
    pub fn new() -> WasmVm {
        WasmVm { }
    }
    #[wasm_bindgen]
    pub fn interpret(&mut self, source: &str) -> Output {
        let mut interpreter = Interpreter::new(source.to_owned());
        return match interpreter.interpret() {
            InterpretResult::Ok => Output::successful(),
            InterpretResult::CompileErr(compiler_errors) => Output::compile_err(compiler_errors),
            InterpretResult::RuntimeErr(runtime_error) => Output::runtime_err(runtime_error),        
        };
    }

}
