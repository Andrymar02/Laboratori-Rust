pub trait Aggregator {
    fn update(&mut self, value: f64);
    fn result(&self) -> String;
    fn mode_name(&self) -> &'static str;
}

fn format_f64(value: f64) -> String {
    if value.fract() == 0.0 {
        format!("{:.1}", value)
    } else {
        value.to_string()
    }
}

pub struct Count {
    count: u64,
}

impl Count {
    pub fn new() -> Self {
        Count { count: 0 }
    }
}

impl Aggregator for Count {
    fn update(&mut self, _value: f64) {
        self.count += 1;
    }

    fn result(&self) -> String {
        self.count.to_string()
    }

    fn mode_name(&self) -> &'static str {
        "count"
    }
}

pub struct Sum {
    sum: f64,
}

impl Sum {
    pub fn new() -> Self {
        Sum { sum: 0.0 }
    }
}

impl Aggregator for Sum {
    fn update(&mut self, value: f64) {
        self.sum += value;
    }

    fn result(&self) -> String {
    format_f64(self.sum)
    }

    fn mode_name(&self) -> &'static str {
        "sum"
    }
}

pub struct Average {
    sum: f64,
    count: u64,
}

impl Average {
    pub fn new() -> Self {
        Average { sum: 0.0, count: 0 }
    }
}

impl Aggregator for Average {
    fn update(&mut self, value: f64) {
        self.sum += value;
        self.count += 1;
    }

    fn result(&self) -> String {
        if self.count == 0 {
            "NaN".to_string()
        } else {
            format_f64(self.sum / self.count as f64)
        }
    }

    fn mode_name(&self) -> &'static str {
        "avg"
    }
}


pub struct Min {
    current: Option<f64>,
}

impl Min {
    pub fn new() -> Self {
        Min { current: None }
    }
}

impl Aggregator for Min {
    fn update(&mut self, value: f64) {
        match self.current {
            None => {
                self.current = Some(value);
            }
            Some(v) => {
                if value < v {
                    self.current = Some(value);
                }
            }
        }
    }

    fn result(&self) -> String {
        match self.current {
            None => "NaN".to_string(),
            Some(v) => format_f64(v),
        }
    }

    fn mode_name(&self) -> &'static str {
        "min"
    }
}

pub struct Max {
    current: Option<f64>,
}

impl Max {
    pub fn new() -> Self {
        Max { current: None }
    }
}

impl Aggregator for Max {
    fn update(&mut self, value: f64) {
        match self.current {
            None => {
                self.current = Some(value);
            }
            Some(v) => {
                if value > v {
                    self.current = Some(value);
                }
            }
        }
    }

    fn result(&self) -> String {
        match self.current {
            None => "NaN".to_string(),
            Some(v) => format_f64(v),
        }
    }

    fn mode_name(&self) -> &'static str {
        "max"
    }
}