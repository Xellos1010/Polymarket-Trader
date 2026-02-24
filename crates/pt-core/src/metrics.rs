use parking_lot::RwLock;
use std::collections::BTreeMap;

#[derive(Debug, Default)]
pub struct MetricsRegistry {
    counters: RwLock<BTreeMap<String, f64>>,
    gauges: RwLock<BTreeMap<String, f64>>,
}

impl MetricsRegistry {
    pub fn inc_counter(&self, name: &str, delta: f64) {
        let mut map = self.counters.write();
        let entry = map.entry(name.to_string()).or_insert(0.0);
        *entry += delta;
    }

    pub fn set_gauge(&self, name: &str, value: f64) {
        self.gauges.write().insert(name.to_string(), value);
    }

    pub fn get_counter(&self, name: &str) -> f64 {
        *self.counters.read().get(name).unwrap_or(&0.0)
    }

    pub fn get_gauge(&self, name: &str) -> f64 {
        *self.gauges.read().get(name).unwrap_or(&0.0)
    }

    pub fn render_prometheus(&self) -> String {
        let mut out = String::new();
        for (k, v) in self.counters.read().iter() {
            out.push_str(&format!(
                "# TYPE {} counter\n{} {}\n",
                sanitize(k),
                sanitize(k),
                v
            ));
        }
        for (k, v) in self.gauges.read().iter() {
            out.push_str(&format!(
                "# TYPE {} gauge\n{} {}\n",
                sanitize(k),
                sanitize(k),
                v
            ));
        }
        out
    }
}

fn sanitize(name: &str) -> String {
    name.chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '_' {
                c
            } else {
                '_'
            }
        })
        .collect()
}
