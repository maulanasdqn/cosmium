use regex::Regex;

use crate::domain::profile::Profile;

pub struct ProbeDef {
    pub id: &'static str,
    pub expression: String,
    pub expected: Regex,
    pub expected_display: String,
    pub negate: bool,
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
    let webgl_renderer_expr = "(()=>{const c=document.createElement('canvas').getContext('webgl');const e=c.getExtension('WEBGL_debug_renderer_info');return c.getParameter(e.UNMASKED_RENDERER_WEBGL);})()";

    let mut probes = vec![
        simple("webdriver", "String(navigator.webdriver)", "false"),
        simple(
            "platform",
            "navigator.platform",
            &p.identity.navigator_platform,
        ),
        simple("language", "navigator.language", &lang0),
        simple(
            "languages",
            "JSON.stringify(navigator.languages)",
            &lang_array,
        ),
        simple(
            "hardware_concurrency",
            "String(navigator.hardwareConcurrency)",
            &p.hardware.hardware_concurrency.to_string(),
        ),
        simple(
            "device_memory",
            "String(navigator.deviceMemory)",
            &p.hardware.device_memory_gb.to_string(),
        ),
        simple(
            "color_depth",
            "String(screen.colorDepth)",
            &p.screen.color_depth.to_string(),
        ),
        simple(
            "ua_data_platform",
            "(await navigator.userAgentData.getHighEntropyValues(['platform'])).platform",
            &p.identity.client_hints.platform,
        ),
        simple(
            "webgl_vendor",
            "(()=>{const c=document.createElement('canvas').getContext('webgl');const e=c.getExtension('WEBGL_debug_renderer_info');return c.getParameter(e.UNMASKED_VENDOR_WEBGL);})()",
            &p.gpu.vendor,
        ),
        simple("webgl_renderer", webgl_renderer_expr, &p.gpu.renderer),
        ProbeDef {
            id: "no_swiftshader",
            expression: webgl_renderer_expr.to_owned(),
            expected: Regex::new("SwiftShader").expect("compile regex"),
            expected_display: "(no SwiftShader)".into(),
            negate: true,
        },
        simple(
            "timezone",
            "Intl.DateTimeFormat().resolvedOptions().timeZone",
            &p.locale.timezone,
        ),
    ];
    probes.shrink_to_fit();
    probes
}

fn simple(id: &'static str, expr: &str, expected: &str) -> ProbeDef {
    ProbeDef {
        id,
        expression: expr.to_owned(),
        expected: Regex::new(&format!("^{}$", regex::escape(expected))).expect("compile regex"),
        expected_display: expected.to_owned(),
        negate: false,
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
