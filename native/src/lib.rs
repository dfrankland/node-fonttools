#[macro_use]
extern crate neon;
extern crate pyo3;

mod prelude;
mod python;
mod js;
mod logging;

use neon::{
    mem::Handle,
    js::{
        Object,
        JsObject, JsBoolean, JsFunction,
        class::{Class, JsClass},
    },
};
use js::JsFontTools;

register_module!(
    module,
    {
        // Register as an ES Module
        let es_module = JsObject::new(module.scope);
        es_module.set("value", JsBoolean::new(module.scope, true))?;
        module.exports.set("__esModule", es_module)?;

        // Export default
        let class: Handle<JsClass<JsFontTools>> = JsFontTools::class(module.scope)?;
        let constructor: Handle<JsFunction<JsFontTools>> = class.constructor(module.scope)?;
        module.exports.set("default", constructor)?;

        Ok(())
    }
);
