use pyo3::prelude::*;
use neon::{
    mem::Handle,
    vm::Lock,
    js::{
        Object,
        JsValue, JsFunction, JsObject, JsString,
        binary::JsBuffer,
    },
};

use prelude::*;
use python::{
    IMPORTS,
    import,
    eval,
    insert_sys_path,
};
use logging::{
    is_debug,
    console,
};

pub struct FontTools {
    path: String,
}

declare_types! {
    pub class JsFontTools for FontTools {
        init(call) {
            let scope = call.scope;
            let this = call.arguments.this(scope);

            let path = {
                let require = call.arguments.require(scope, 0)?.check::<JsFunction>()?;
                let module = call.arguments.require(scope, 1)?.check::<JsObject>()?;
                let path_module = {
                    let require_path_args = vec![JsString::new_or_throw(scope, "path")?];
                    require
                        .call(scope, this, require_path_args)?
                        .check::<JsObject>()?
                };
                let module_filename = module.get(scope, "filename")?.check::<JsString>()?;
                let path_resolve_args = vec![
                    module_filename,
                    JsString::new_or_throw(scope, "../native/fonttools/Lib")?,
                ];
                path_module
                    .get(scope, "resolve")?.check::<JsFunction>()?
                    .call(scope, this, path_resolve_args)?
                    .check::<JsString>()?
                    .value()
            };

            if is_debug(scope)? {
                let console_args = vec![
                    JsString::new_or_throw(scope, "Path to FontTools Python library:")?,
                    JsString::new_or_throw(scope, &path)?,
                ];
                console(scope, this, None, console_args)?;
            }

            Ok(FontTools{ path })
        }

        method decompile(call) {
            let scope = call.scope;
            let mut this = call.arguments.this(scope);

            let path = this.grab(|fonttools| fonttools.path.clone());
            let mut ttf_js_buffer = call.arguments.require(scope, 0)?.check::<JsBuffer>()?;
            let ttf_slice = ttf_js_buffer.grab(|x| x.as_slice());

            if is_debug(scope)? {
                let mut js_buffer = JsBuffer::new(scope, ttf_slice.len() as u32)?;
                js_buffer.grab(|mut x| x.as_mut_slice().copy_from_slice(ttf_slice));
                let console_args = vec![
                    JsString::new_or_throw(scope, "Got TTF Buffer:")?.upcast::<JsValue>(),
                    js_buffer.upcast::<JsValue>(),
                ];
                console(scope, this, None, console_args)?;
            }

            let gil = Python::acquire_gil();
            let py = gil.python();
            let locals = PyDict::new(py);

            // Setup to allow Python to import FontTools
            insert_sys_path(&py, &locals, &path).to_js_result(scope, &py)?;

            // Import modules to work with fonts
            import(&py, &locals, "fontTools.ttLib").to_js_result(scope, &py)?;
            import(&py, &locals, "io").to_js_result(scope, &py)?;

            // Add TTF buffer to `locals`
            locals.set_item("ttf", PyByteArray::new(py, ttf_slice)).to_js_result(scope, &py)?;

            // Create "file" from TTF buffer
            let py_ttf_file = eval(&py, &locals, &format!("{}['io'].BytesIO(ttf)", IMPORTS)).to_vm_result()?;
            locals.set_item("ttf_file", py_ttf_file).to_js_result(scope, &py)?;

            // Create empty "file" to write to
            let py_file = eval(&py, &locals, &format!("{}['io'].BytesIO()", IMPORTS)).to_vm_result()?;
            locals.set_item("file", py_file).to_js_result(scope, &py)?;

            // Convert TTF buffer "file" to XML and write to empty "file"
            eval(&py, &locals, &format!("{}['fontTools.ttLib'].TTFont(ttf_file).saveXML(file)", IMPORTS)).to_vm_result()?;

            // Convert Python bytearray to JS buffer
            let js_buffer_xml: Handle<JsBuffer> = {
                let py_bytearray: Result<&PyByteArray, PyDowncastError> = {
                    eval(&py, &locals, "bytearray(file.getvalue())")
                        .to_vm_result()?
                        .try_into()
                };
                Ok(py_bytearray.to_vm_result()?).to_js_result(scope, &py)?
            };

            Ok(js_buffer_xml.upcast())
        }
    }
}
