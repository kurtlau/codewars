struct Population {
    p0: i32,
    percent: f64,
    aug: i32,
}

impl Iterator for Population {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        let p0 = self.p0;
        let born = (self.p0 as f64 * self.percent * 0.01f64) as i32;
        self.p0 = self.p0 + born + self.aug;
        Some(p0)
    }
}

fn nb_year(p0: i32, percent: f64, aug: i32, p: i32) -> i32 {
    let mut population = Population {
        p0: p0,
        percent: percent,
        aug: aug,
    };

    for (i, v) in population.into_iter().enumerate() {
        match v >= p {
            true => return i as i32,
            false => {}
        }
    }

    -1
}
