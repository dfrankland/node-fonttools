#[macro_use]
extern crate neon;

use neon::{
    vm::{Call, JsResult},
    js::{
        Object,
        JsObject, JsBoolean, JsString
    },
};

#[allow(unknown_lints, needless_pass_by_value)]
fn default(call: Call) -> JsResult<JsString> {
    JsString::new_or_throw(call.scope, "Hello World")
}

register_module!(
    module,
    {
        // Register as an ES Module
        let es_module = JsObject::new(module.scope);
        es_module.set("value", JsBoolean::new(module.scope, true))?;
        module.exports.set("__esModule", es_module)?;

        // Export default function
        module.export("default", default)
    }
);
