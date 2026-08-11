use super::probe::ProbeDef;

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
