pub struct FunctionNode {
    func: Vec<f32>,
}

impl FunctionNode {
    pub fn new() -> Self {
        Self {
            func: Vec::new(),
        }
    }

    pub fn add(&mut self, input: f32, output: f32) {
        self.func.push(output / input);
    }
}
