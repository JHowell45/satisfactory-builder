pub trait ModelTrait {
    fn output(&self) -> f32;

    fn power_usage(&self) -> i32;
}

pub trait GroupTrait {
    fn number_of_machines(&self) -> usize;
    fn total_output(&self) -> f32;
    fn total_power_usage(&self) -> i32;
}