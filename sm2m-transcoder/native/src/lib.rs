use neon::prelude::*;
use sm2m_transcoder_driver::driver::Driver;

fn version(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string(Driver::version()))
}

fn libusb_version(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string(Driver::libusb_version()))
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("version", version)?;
    cx.export_function("libusb_version", libusb_version)?;
    Ok(())
}
