use super::probe::ProbeDef;

const PROBE_TIMEOUT_MS: u32 = 5000;

pub(super) fn render_html(probes: &[ProbeDef]) -> String {
    let entries: String = probes
        .iter()
        .map(|p| {
            format!(
                "[{},{}]",
                serde_json::to_string(p.id).unwrap_or_default(),
                serde_json::to_string(&p.expression).unwrap_or_default()
            )
        })
        .collect::<Vec<_>>()
        .join(",");
    format!(
        "<!doctype html><html><body><iframe id=\"f\" src=\"about:blank\" style=\"display:none\"></iframe>\
        <pre id=\"out\">running</pre><script>\
        const timeout=(ms)=>new Promise((_,rej)=>setTimeout(()=>rej(new Error('timeout')),ms));\
        async function run(){{const probes=[{entries}];\
        const out=await Promise.all(probes.map(async([id,expr])=>{{\
        try{{const fn=new Function('return (async()=>('+expr+'))()');\
        const v=await Promise.race([fn(),timeout({PROBE_TIMEOUT_MS})]);return JSON.stringify({{id,value:String(v),error:null}});}}\
        catch(e){{return JSON.stringify({{id,value:null,error:String(e)}});}}}}));\
        const o=document.getElementById('out');o.textContent=out.join('\\n');o.dataset.done='1';}}\
        window.addEventListener('load',run);\
        </script></body></html>"
    )
}
