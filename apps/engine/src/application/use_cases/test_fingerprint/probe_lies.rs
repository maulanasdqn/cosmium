use super::probe::{ProbeDef, simple};

const WORKER: &str = "(src)=>new Promise((res)=>{const b=new Blob([`onmessage=async()=>{try{postMessage(String(await (async()=>(${src}))()))}catch(e){postMessage('ERR '+e)}}`],{type:'application/javascript'});const w=new Worker(URL.createObjectURL(b));w.onmessage=(e)=>{res(e.data);w.terminate();};w.postMessage(0);})";

fn in_worker(expr: &str) -> String {
    let src = serde_json::to_string(expr).unwrap_or_default();
    format!("await ({WORKER})({src})")
}

fn same_in_worker(id: &'static str, expr: &str) -> ProbeDef {
    let worker = in_worker(expr);
    simple(
        id,
        &format!("String(String(await (async()=>({expr}))()) === String({worker}))"),
        "true",
    )
}

fn same_in_iframe(id: &'static str, expr: &str) -> ProbeDef {
    let framed = expr.replace(
        "navigator",
        "document.getElementById('f').contentWindow.navigator",
    );
    simple(
        id,
        &format!("String(String({expr}) === String({framed}))"),
        "true",
    )
}

fn native_getter(id: &'static str, proto: &str, prop: &str) -> ProbeDef {
    simple(
        id,
        &format!(
            "(()=>{{const d=Object.getOwnPropertyDescriptor({proto}.prototype,'{prop}');return String(!!d&&!!d.get&&Function.prototype.toString.call(d.get).includes('[native code]'));}})()"
        ),
        "true",
    )
}

pub(super) fn lie_probes() -> Vec<ProbeDef> {
    let mut out = global_tells();
    out.extend(consistency_probes());
    out
}

fn global_tells() -> Vec<ProbeDef> {
    vec![
        simple(
            "lie_navigator_own_props",
            "JSON.stringify(Object.getOwnPropertyNames(navigator))",
            "[]",
        ),
        simple(
            "lie_screen_own_props",
            "JSON.stringify(Object.getOwnPropertyNames(screen))",
            "[]",
        ),
        simple(
            "lie_uadata_own_props",
            "JSON.stringify(Object.getOwnPropertyNames(navigator.userAgentData))",
            "[]",
        ),
        simple(
            "lie_tostring_native",
            "String(Function.prototype.toString.call(Function.prototype.toString) === 'function toString() { [native code] }')",
            "true",
        ),
        simple(
            "lie_document_own_props",
            "JSON.stringify(Object.getOwnPropertyNames(document))",
            "[\"location\"]",
        ),
        simple(
            "lie_tostring_own_props",
            "JSON.stringify(Object.getOwnPropertyNames(Function.prototype.toString))",
            "[\"length\",\"name\"]",
        ),
        simple(
            "lie_cosmium_global",
            "String('__cosmiumPf' in window)",
            "false",
        ),
        simple(
            "lie_prepare_stack",
            "typeof Error.prepareStackTrace",
            "undefined",
        ),
        simple(
            "lie_chrome_runtime",
            "typeof window.chrome.runtime",
            "undefined",
        ),
        simple(
            "lie_fetch_native",
            "String(Function.prototype.toString.call(window.fetch).includes('[native code]') && Object.getOwnPropertyNames(window.fetch).join() === 'length,name')",
            "true",
        ),
        simple(
            "localhost_blocked",
            "await fetch('http://127.0.0.1:9/').then(()=>'reachable',(e)=>e.name)",
            "TypeError",
        ),
    ]
}

fn consistency_probes() -> Vec<ProbeDef> {
    vec![
        native_getter("lie_getter_hwc", "Navigator", "hardwareConcurrency"),
        native_getter("lie_getter_platform", "Navigator", "platform"),
        native_getter("lie_getter_languages", "Navigator", "languages"),
        native_getter("lie_getter_screen_w", "Screen", "width"),
        simple(
            "lie_brands_match_ua",
            "String(navigator.userAgentData.brands.find(b=>b.brand==='Google Chrome')?.version === (navigator.userAgent.match(/Chrome\\/(\\d+)/)||[])[1])",
            "true",
        ),
        same_in_worker("worker_hwc", "navigator.hardwareConcurrency"),
        same_in_worker("worker_platform", "navigator.platform"),
        same_in_worker("worker_languages", "JSON.stringify(navigator.languages)"),
        same_in_worker("worker_ua", "navigator.userAgent"),
        same_in_worker(
            "worker_tz",
            "Intl.DateTimeFormat().resolvedOptions().timeZone",
        ),
        same_in_worker(
            "worker_webgl",
            "(()=>{const g=new OffscreenCanvas(1,1).getContext('webgl');const e=g.getExtension('WEBGL_debug_renderer_info');return g.getParameter(e.UNMASKED_RENDERER_WEBGL);})()",
        ),
        same_in_iframe("iframe_hwc", "navigator.hardwareConcurrency"),
        same_in_iframe("iframe_platform", "navigator.platform"),
        same_in_iframe("iframe_ua", "navigator.userAgent"),
    ]
}
