pub mod keyboard;
pub mod mouse;
pub mod scroll;

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct BoundingBox {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

impl BoundingBox {
    pub fn center(&self) -> Point {
        Point {
            x: self.x + self.width / 2.0,
            y: self.y + self.height / 2.0,
        }
    }

    pub fn random_point(&self, rng: &mut impl rand::Rng) -> Point {
        let margin_x = self.width * 0.2;
        let margin_y = self.height * 0.2;
        Point {
            x: rng.random_range((self.x + margin_x)..(self.x + self.width - margin_x)),
            y: rng.random_range((self.y + margin_y)..(self.y + self.height - margin_y)),
        }
    }
}

pub async fn element_box(page: &chromiumoxide::Page, selector: &str) -> Option<BoundingBox> {
    let js = format!(
        r#"(() => {{
  const el = document.querySelector({});
  if (!el) return null;
  const r = el.getBoundingClientRect();
  return JSON.stringify({{ x: r.x, y: r.y, width: r.width, height: r.height }});
}})()"#,
        serde_json::to_string(selector).unwrap_or_default()
    );
    let val = page.evaluate(js).await.ok()?;
    let s: String = val.into_value().ok()?;
    let v: serde_json::Value = serde_json::from_str(&s).ok()?;
    Some(BoundingBox {
        x: v["x"].as_f64()?,
        y: v["y"].as_f64()?,
        width: v["width"].as_f64()?,
        height: v["height"].as_f64()?,
    })
}
