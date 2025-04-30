use wasmtime::component::*;
use wasmtime::{Config, Engine, Store};

bindgen!({
    path: "minimal.wit",
});

fn main() -> wasmtime::Result<()> {
    let mut config = Config::new();
    config.wasm_component_model(true);
    let engine = Engine::new(&config)?;
    let component = Component::from_file(&engine, "minimal.wasm")?;

    let linker = Linker::new(&engine);

    let mut store = Store::new(&engine, ());

    let bindings = Root::instantiate(&mut store, &component, &linker)?;

    let res = bindings.call_answer(&mut store)?;
    println!("{res}");
    Ok(())
}
