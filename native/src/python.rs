use pyo3::prelude::*;

pub const IMPORTS: &str = "imports";

pub fn import<'a>(py: &'a Python, locals: &'a PyDict, import_name: &str) -> PyResult<&'a PyModule> {
    let imports = match locals.get_item(IMPORTS) {
        Some(imports) => imports.try_into()?,
        None => {
            let imports = PyDict::new(*py);
            locals.set_item(IMPORTS, imports)?;
            imports
        }
    };

    let import_module = match imports.get_item(import_name) {
        Some(import) => import.try_into()?,
        None => {
            let new_import_module = py.import(import_name)?;
            imports.set_item(import_name, new_import_module)?;
            new_import_module
        },
    };

    Ok(import_module)
}

pub fn eval<'a>(py: &'a Python, locals: &'a PyDict, eval: &str) -> PyResult<&'a PyObjectRef> {
    py.eval(eval, None, Some(&locals))
}

pub fn insert_sys_path<'a>(py: &'a Python, locals: &'a PyDict, path: &str) -> PyResult<&'a PyObjectRef> {
    import(py, locals, "sys")?;
    eval(py, locals, &format!("{}['sys'].path.insert(0, \"{}\")", IMPORTS, path))
}
