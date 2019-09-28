use wasmer_runtime::{imports, instantiate, Instance};
use korrecte_shared::reporting::Finding;
use serde_json;
use crate::reporting::Reporter;
use crate::linters::LintList;
use crate::kube::ObjectRepository;

pub struct WasmEvaluator {
    instance: Instance,
}

impl WasmEvaluator {
    pub fn new() -> Self {
        let ini = std::time::SystemTime::now();

        // Instantiate the web assembly module
        let instance = instantiate(WASM, &imports!{}).expect("failed to instantiate wasm module");

        let elapsed = ini.elapsed().unwrap();
        println!("Instantation time: {:?}", elapsed);

        WasmEvaluator {
            instance,
        }
    }

    pub fn evaluate(
        &self,
        reporter: &dyn Reporter,
        object_repository: &dyn ObjectRepository,
    ) {
        for object in object_repository.all() {
            let ini = std::time::SystemTime::now();

            let finding = self.execute_wasm_function("_lint_something");
            reporter.report(finding);

            let elapsed = ini.elapsed().unwrap();
            println!("Elapsed: {:?}", elapsed);

        }
    }

    fn execute_wasm_function(&self, fn_name: &str) -> Finding {
        let context = self.instance.context();
        let memory = context.memory(0);
        // Now we can get a view of that memory
        let view = memory.view::<u8>();
        // Zero our the first 4 bytes of memory
        for cell in view[1..5].iter() {
            cell.set(0);
        }
        let bytes = "test".as_bytes();
        // Our length of bytes
        let len = bytes.len();
        // loop over the wasm memory view's bytes
        // and also the string bytes
        for (cell, byte) in view[5..len + 5].iter().zip(bytes.iter()) {
            // set each wasm memory byte to
            // be the value of the string byte
            cell.set(*byte)
        }
        // Bind our helper function
        let lint = self.instance.func::<(i32, u32), i32>(fn_name).expect("Failed to bind _lint_something");
        // Call the helper function an store the start of the returned string
        let start = lint.call(5 as i32, len as u32).expect("Failed to execute _multiply") as usize;
        // Get an updated view of memory
        let new_view = memory.view::<u8>();
        // Setup the 4 bytes that will be converted
        // into our new length
        let mut new_len_bytes = [0u8;4];
        for i in 0..4 {
            // attempt to get i+1 from the memory view (1,2,3,4)
            // If we can, return the value it contains, otherwise
            // default back to 0
            new_len_bytes[i] = new_view.get(i + 1).map(|c| c.get()).unwrap_or(0);
        }
        // Convert the 4 bytes into a u32 and cast to usize
        let new_len = u32::from_ne_bytes(new_len_bytes) as usize;
        // Calculate the end as the start + new length
        let end = start + new_len;
        // Capture the string as bytes
        // from the new view of the wasm memory
        let updated_bytes: Vec<u8> = new_view[start..end]
            .iter()
            .map(|c|c.get())
            .collect();
        // Convert the bytes to a string
        let updated: Finding = serde_json::from_slice(&updated_bytes)
            .expect("Failed to convert wasm memory to tuple");

        updated
    }
}

// For now we are going to use this to read in our wasm bytes
static WASM: &[u8] = include_bytes!("../../../target/wasm32-unknown-unknown/debug/korrecte_wasm.wasm");