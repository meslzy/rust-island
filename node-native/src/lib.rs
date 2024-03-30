use neon::prelude::*;

fn hello_world(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string("hello world"))
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("helloWorld", hello_world)?;
    Ok(())
}
