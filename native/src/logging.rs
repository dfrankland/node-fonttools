use neon::{
    vm::{
        JsResult, Throw, This,
    },
    js::{
        Value, Object,
        JsFunction, JsObject, JsString, JsUndefined,
    },
    scope::Scope,
    mem::Handle,
};

pub fn is_debug<'a, T: Scope<'a>>(scope: &mut T) -> Result<bool, Throw> {
    let global = scope.global().check::<JsObject>()?;
    Ok({
        global
            .get(scope, "process")?.check::<JsObject>()?
            .get(scope, "env")?.check::<JsObject>()?
            .get(scope, "DEBUG")?.is_a::<JsString>()
    })
}

#[allow(dead_code)]
pub enum ConsoleMethod {
    Assert,
    Clear,
    Count,
    Dir,
    Dirxml,
    Error,
    Group,
    GroupCollapsed,
    GroupEnd,
    Info,
    Log,
    Profile,
    ProfileEnd,
    Table,
    Time,
    TimeEnd,
    TimeStamp,
    Trace,
    Warn,
}

fn get_console_method(console_method: &ConsoleMethod) -> &'static str {
    match console_method {
        ConsoleMethod::Assert => "assert",
        ConsoleMethod::Clear => "clear",
        ConsoleMethod::Count => "count",
        ConsoleMethod::Dir => "dir",
        ConsoleMethod::Dirxml => "dirxml",
        ConsoleMethod::Error => "error",
        ConsoleMethod::Group => "group",
        ConsoleMethod::GroupCollapsed => "groupCollapsed",
        ConsoleMethod::GroupEnd => "groupEnd",
        ConsoleMethod::Info => "info",
        ConsoleMethod::Log => "log",
        ConsoleMethod::Profile => "profile",
        ConsoleMethod::ProfileEnd => "profileEnd",
        ConsoleMethod::Table => "table",
        ConsoleMethod::Time => "time",
        ConsoleMethod::TimeEnd => "timeEnd",
        ConsoleMethod::TimeStamp => "timeStamp",
        ConsoleMethod::Trace => "trace",
        ConsoleMethod::Warn => "warn",
    }
}

pub fn console
    <'a, 'b, T: Scope<'a>, U: This + Value + 'a, A: Value + 'b, AS: IntoIterator<Item=Handle<'b, A>>>
    (scope: &mut T, this: Handle<'a, U>, method: Option<&ConsoleMethod>, arguments: AS) -> JsResult<'a, JsUndefined>
    where 'a: 'b
{
    let global = scope.global().check::<JsObject>()?;

    let method = match method {
        Some(ref m) => get_console_method(m),
        None => get_console_method(&ConsoleMethod::Log),
    };

    Ok({
        global
            .get(scope, "console")?.check::<JsObject>()?
            .get(scope, method)?.check::<JsFunction>()?
            .call(scope, this, arguments)?
            .check::<JsUndefined>()?
    })
}
