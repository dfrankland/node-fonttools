use pyo3::prelude::*;
use python::{
    IMPORTS,
    import, eval,
};
use neon::{
    vm::{
        Lock, Throw, VmResult, JsResult,
    },
    scope::Scope,
    mem::Managed,
    js::{
        JsString, JsUndefined,
        binary::JsBuffer,
        error::{
            Kind::{
                Error,
            },
            JsError,
        },
    },
};

fn py_err_to_js_err<'a, T>(e: &PyErr, py: &'a Python) -> VmResult<T> {
    if let Some(ref tb) = e.ptraceback {
        let locals = PyDict::new(*py);
        import(py, locals, "string").to_vm_result()?;
        import(py, locals, "traceback").to_vm_result()?;
        locals.set_item("tb", tb).to_vm_result()?;
        println!("{}", locals);
        let eval_string = format!("{}.string.join({}.traceback.format_tb(tb))", IMPORTS, IMPORTS);
        let py_traceback_string = eval(py, locals, &eval_string).to_vm_result()?;
        let traceback: String = py_traceback_string.extract().unwrap();
        return JsError::throw(Error, &format!("Python Error: {:?}\n{}", e.ptype, traceback));
    }

    JsError::throw(Error, &format!("Python Error: {:?}", e))
}

pub trait PyResultToJsResult<'a, 'b, T: Scope<'a>, U: Managed> {
    fn to_js_result(self, &mut T, &'b Python) -> JsResult<'a, U>;
}

pub trait PyResultToVmResult<'a, T> {
    fn to_vm_result(self) -> VmResult<&'a T>;
}

impl<'a, 'b, T: Scope<'a>, U> PyResultToJsResult<'a, 'b, T, JsUndefined> for PyResult<U> {
    fn to_js_result(self, _scope: &mut T, py: &'b Python) -> JsResult<'a, JsUndefined> {
        match self {
            Ok(_) => Ok(JsUndefined::new()),
            Err(ref e) => py_err_to_js_err(e, py),
        }
    }
}

impl<'a, 'b, T: Scope<'a>> PyResultToJsResult<'a, 'b, T, JsString> for PyResult<String> {
    fn to_js_result(self, scope: &mut T, py: &'b Python) -> JsResult<'a, JsString> {
        match self {
            Ok(v) => JsString::new_or_throw(scope, &v),
            Err(ref e) => py_err_to_js_err(e, py),
        }
    }
}

impl<'a, 'b, 'c, T: Scope<'a>> PyResultToJsResult<'a, 'b, T, JsBuffer> for PyResult<&'c PyByteArray> {
    fn to_js_result(self, scope: &mut T, py: &'b Python) -> JsResult<'a, JsBuffer> {
        match self {
            Ok(v) => {
                let data = v.data();
                let mut js_buffer = JsBuffer::new(scope, data.len() as u32)?;
                js_buffer.grab(|mut x| x.as_mut_slice().copy_from_slice(data));
                Ok(js_buffer)
            },
            Err(ref e) => py_err_to_js_err(e, py),
        }
    }
}

impl<'a, T> PyResultToVmResult<'a, T> for Result<&'a T, PyDowncastError> {
    fn to_vm_result(self) -> VmResult<&'a T> {
        match self {
            Ok(v) => Ok(v),
            Err(_) => Err(Throw),
        }
    }
}

impl<'a, T> PyResultToVmResult<'a, T> for Result<&'a T, PyErr> {
    fn to_vm_result(self) -> VmResult<&'a T> {
        match self {
            Ok(v) => Ok(v),
            Err(_) => Err(Throw),
        }
    }
}

impl<'a> PyResultToVmResult<'a, ()> for Result<(), PyErr> {
    fn to_vm_result(self) -> VmResult<&'a ()> {
        match self {
            Ok(_) => Ok(&()),
            Err(_) => Err(Throw),
        }
    }
}
