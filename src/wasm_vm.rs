use bytecode_vm::{NativeFunction, Value};
use wasm_bindgen::prelude::*;
use bytecode_vm::interpreter::Interpreter;
use bytecode_vm::interpreter::{InterpretResult, CompilerError, RuntimeError};

#[wasm_bindgen]
pub struct WasmVm {
    natives: Vec<JsNativeFn>
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct CompilerErr {
    pub line: usize,
    pub start: usize,
    pub len: usize
}

#[wasm_bindgen]
pub struct JsNativeFn {
    name: String,
    arity: u8,
    function: js_sys::Function
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
        WasmVm { natives: vec![] }
    }
    #[wasm_bindgen]
    pub fn interpret(&mut self, source: &str, natives: Vec<JsNativeFn>) -> Output {
        let mut interpreter = Interpreter::new(source.to_owned());

        
        let mut rust_natives: Vec::<NativeFunction> = vec![];
        for (i, native) in natives.into_iter().enumerate() {
            
            self.natives.push(native);

            rust_natives.push(NativeFunction { 
                name: native.name.clone(), 
                arity: native.arity, 
                function: {
                    fn my_func(vals: &[Value]) -> Value {
                        return self.natives[i].function.call0(&JsValue::NULL);
                    }
                    my_func
                } 
            });
        }


        let print = NativeFunction { 
            name: "print".to_owned(), 
            arity: 1, 
            function: {
                fn print(vals: &[Value]) -> Value {
                    println!("{}", vals[0]);
                    return Value::Null;
                }
                print            
            }
        };

        return match interpreter.interpret(vec![print]) {
            InterpretResult::Ok => Output::successful(),
            InterpretResult::CompileErr(compiler_errors) => Output::compile_err(compiler_errors),
            InterpretResult::RuntimeErr(runtime_error) => Output::runtime_err(runtime_error),        
        };
    }

}
