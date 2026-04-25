use regex::Regex;

use crate::domain::profile::Profile;

pub struct ProbeDef {
    pub id: &'static str,
    pub expression: String,
    pub expected: Regex,
    pub expected_display: String,
}

pub struct ProbeResult {
    pub id: String,
    pub passed: bool,
    pub got: String,
    pub expected: String,
    pub error: Option<String>,
}

pub fn for_profile(p: &Profile) -> Vec<ProbeDef> {
    let lang0 = p.locale.languages.first().cloned().unwrap_or_default();
    let lang_array = format!(
        "[{}]",
        p.locale
            .languages
            .iter()
            .map(|l| format!("\"{l}\""))
            .collect::<Vec<_>>()
            .join(",")
    );
    let mut probes = vec![];
    probes.push(simple("webdriver", "String(navigator.webdriver)", "false"));
    probes.push(simple("platform", "navigator.platform", &p.identity.navigator_platform));
    probes.push(simple("language", "navigator.language", &lang0));
    probes.push(simple(
        "languages",
        "JSON.stringify(navigator.languages)",
        &lang_array,
    ));
    probes.push(simple(
        "hardware_concurrency",
        "String(navigator.hardwareConcurrency)",
        &p.hardware.hardware_concurrency.to_string(),
    ));
    probes.push(simple(
        "device_memory",
        "String(navigator.deviceMemory)",
        &p.hardware.device_memory_gb.to_string(),
    ));
    probes.push(simple(
        "color_depth",
        "String(screen.colorDepth)",
        &p.screen.color_depth.to_string(),
    ));
    probes.push(simple(
        "ua_data_platform",
        "(await navigator.userAgentData.getHighEntropyValues(['platform'])).platform",
        &p.identity.client_hints.platform,
    ));
    probes.push(simple(
        "webgl_vendor",
        "(()=>{const c=document.createElement('canvas').getContext('webgl');const e=c.getExtension('WEBGL_debug_renderer_info');return c.getParameter(e.UNMASKED_VENDOR_WEBGL);})()",
        &p.gpu.vendor,
    ));
    probes.push(simple(
        "webgl_renderer",
        "(()=>{const c=document.createElement('canvas').getContext('webgl');const e=c.getExtension('WEBGL_debug_renderer_info');return c.getParameter(e.UNMASKED_RENDERER_WEBGL);})()",
        &p.gpu.renderer,
    ));
    probes.push(ProbeDef {
        id: "no_swiftshader",
        expression: probes
            .last()
            .map(|p| p.expression.clone())
            .unwrap_or_default(),
        expected: Regex::new(r"^(?!.*SwiftShader).*$").expect("compile regex"),
        expected_display: "(no SwiftShader)".into(),
    });
    probes.push(simple(
        "timezone",
        "Intl.DateTimeFormat().resolvedOptions().timeZone",
        &p.locale.timezone,
    ));
    probes
}

fn simple(id: &'static str, expr: &str, expected: &str) -> ProbeDef {
    ProbeDef {
        id,
        expression: expr.to_owned(),
        expected: Regex::new(&format!("^{}$", regex::escape(expected))).expect("compile regex"),
        expected_display: expected.to_owned(),
    }
}

pub fn render_html(probes: &[ProbeDef]) -> String {
    let entries: String = probes
        .iter()
        .map(|p| {
            format!(
                "[{},{}]",
                serde_json::to_string(p.id).unwrap(),
                serde_json::to_string(&p.expression).unwrap()
            )
        })
        .collect::<Vec<_>>()
        .join(",");
    format!(
        "<!doctype html><pre id=\"out\">running…</pre><script>\
        async function run(){{const probes=[{entries}];const out=[];\
        for(const [id,expr] of probes){{\
        try{{const fn=new Function('return (async()=>('+expr+'))()');\
        const v=await fn();out.push(JSON.stringify({{id,value:String(v),error:null}}));}}\
        catch(e){{out.push(JSON.stringify({{id,value:null,error:String(e)}}));}}}}\
        document.getElementById('out').textContent=out.join('\\n');}}run();\
        </script>"
    )
}
