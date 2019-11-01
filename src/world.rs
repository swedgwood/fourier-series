struct SVector {
    start_angle: f64,
    frequency: f64,
    magnitude: f64,
}

impl SVector {
    pub fn new(start_angle: f64, frequency: f64, magnitude: f64) -> Self {
        Self { start_angle, frequency, magnitude }
    }

    pub fn get_state(&self, t: f64) -> SVectorState {
        let angle = self.start_angle + 2 * PI * self.frequency * t;
        let x = self.magnitude * angle.cos();
        let y = self.magnitude * angle.sin();

        SVectorState { x, y }
    }
}

struct SVectorState {
    x: f64,
    y: f64
}

struct World {
    svectors: [SVector],
}

impl World {
    pub fn get_state() -> WorldState {

    }
}

struct WorldState {

}