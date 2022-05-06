struct Builder {
    name: String,
    base_power: f64,
    base_craft_time: f64,
    clock_speed: f64,
}

impl Builder {
    pub fn new(name: String, power_consumption: f64, base_craft_time: f64) -> Builder {
        Builder { name: name, base_power: power_consumption, base_craft_time: base_craft_time, clock_speed: 1.0 }
    }

    pub fn change_clock_speed(&mut self, new_clock_speed: f64) {
        if new_clock_speed < 0.0 {
            panic!("New speed '{}' is too low!", new_clock_speed);
        } else if new_clock_speed > 2.5 {
            panic!("New speed '{}' is too high!", new_clock_speed);
        } else {
            self.clock_speed = new_clock_speed;
        }
    }

    pub fn is_overclocked(&self) -> bool {
        return self.clock_speed > 1.0;
    }

    pub fn is_underclocked(&self) -> bool {
        return self.clock_speed < 1.0;
    }

    pub fn power_consumption(&self) -> f64 {
        return self.base_power * self.clock_speed.powf(1.6);
    }

    pub fn craft_time(&self) -> f64 {
        return self.base_craft_time * (1.0 / self.clock_speed);
        // return (craft_time * 100.0).round() / 100.0;
    }

    pub fn energy(&self) -> f64 {
        return self.power_consumption() * self.craft_time();
        // return (energy * 10.0).round() / 10.0;
    }

    pub fn energy_percentage(&self) -> f64 {
        let base_energy = self.base_craft_time * self.base_power;
        return self.energy() / base_energy;
        // return (energy_percentage * 1000.0).round() / 1000.0;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_power_consumption_1() {
        let mut builder = Builder::new(String::from("Constructor"), 4.0, 4.0);
        builder.change_clock_speed(0.1);
        let res = builder.power_consumption();
        assert_eq!((res * 10.0).round() / 10.0, 0.1);
    }

    #[test]
    fn test_builder_power_consumption_2() {
        let mut builder = Builder::new(String::from("Constructor"), 4.0, 4.0);
        builder.change_clock_speed(0.5);
        let res = builder.power_consumption();
        assert_eq!((res * 10.0).round() / 10.0, 1.3);
    }

    #[test]
    fn test_builder_power_consumption_3() {
        let mut builder = Builder::new(String::from("Constructor"), 4.0, 4.0);
        builder.change_clock_speed(1.0);
        let res = builder.power_consumption();
        assert_eq!((res * 10.0).round() / 10.0, 4.0);
    }

    #[test]
    fn test_builder_power_consumption_4() {
        let mut builder = Builder::new(String::from("Constructor"), 4.0, 4.0);
        builder.change_clock_speed(1.5);
        let res = builder.power_consumption();
        assert_eq!((res * 10.0).round() / 10.0, 7.7);
    }

    #[test]
    fn test_builder_power_consumption_5() {
        let mut builder = Builder::new(String::from("Constructor"), 4.0, 4.0);
        builder.change_clock_speed(2.0);
        let res = builder.power_consumption();
        assert_eq!((res * 10.0).round() / 10.0, 12.1);
    }

    #[test]
    fn test_builder_power_consumption_6() {
        let mut builder = Builder::new(String::from("Constructor"), 4.0, 4.0);
        builder.change_clock_speed(2.5);
        let res = builder.power_consumption();
        assert_eq!((res * 10.0).round() / 10.0, 17.3);
    }

    #[test]
    fn test_builder_craft_time_1() {
        let mut builder = Builder::new(String::from("Constructor"), 4.0, 4.0);
        builder.change_clock_speed(0.1);
        assert_eq!(builder.craft_time(), 40.0);
    }

    #[test]
    fn test_builder_craft_time_2() {
        let mut builder = Builder::new(String::from("Constructor"), 4.0, 4.0);
        builder.change_clock_speed(0.5);
        assert_eq!(builder.craft_time(), 8.0);
    }

    #[test]
    fn test_builder_craft_time_3() {
        let mut builder = Builder::new(String::from("Constructor"), 4.0, 4.0);
        builder.change_clock_speed(1.0);
        assert_eq!(builder.craft_time(), 4.0);
    }

    #[test]
    fn test_builder_craft_time_4() {
        let mut builder = Builder::new(String::from("Constructor"), 4.0, 4.0);
        builder.change_clock_speed(1.5);
        let res = builder.craft_time();
        assert_eq!((res * 100.0).round() / 100.0, 2.67);
    }

    #[test]
    fn test_builder_craft_time_5() {
        let mut builder = Builder::new(String::from("Constructor"), 4.0, 4.0);
        builder.change_clock_speed(2.0);
        assert_eq!(builder.craft_time(), 2.0);
    }

    #[test]
    fn test_builder_craft_time_6() {
        let mut builder = Builder::new(String::from("Constructor"), 4.0, 4.0);
        builder.change_clock_speed(2.5);
        assert_eq!(builder.craft_time(), 1.6);
    }

    #[test]
    fn test_builder_energy_1() {
        let mut builder = Builder::new(String::from("Constructor"), 4.0, 4.0);
        builder.change_clock_speed(0.1);
        let res = builder.energy();
        assert_eq!((res * 10.0).round() / 10.0, 4.0);
    }

    #[test]
    fn test_builder_energy_2() {
        let mut builder = Builder::new(String::from("Constructor"), 4.0, 4.0);
        builder.change_clock_speed(0.5);
        let res = builder.energy();
        assert_eq!(res, 10.4);
        assert_eq!((res * 10.0).round() / 10.0, 10.4);
    }

    #[test]
    fn test_builder_energy_3() {
        let mut builder = Builder::new(String::from("Constructor"), 4.0, 4.0);
        builder.change_clock_speed(1.0);
        let res = builder.energy();
        assert_eq!((res * 10.0).round() / 10.0, 16.0);
    }

    #[test]
    fn test_builder_energy_4() {
        let mut builder = Builder::new(String::from("Constructor"), 4.0, 4.0);
        builder.change_clock_speed(1.5);
        let res = builder.energy();
        assert_eq!((res * 10.0).round() / 10.0, 20.4);
    }

    #[test]
    fn test_builder_energy_5() {
        let mut builder = Builder::new(String::from("Constructor"), 4.0, 4.0);
        builder.change_clock_speed(2.0);
        let res = builder.energy();
        assert_eq!((res * 10.0).round() / 10.0, 24.2);
    }

    #[test]
    fn test_builder_energy_6() {
        let mut builder = Builder::new(String::from("Constructor"), 4.0, 4.0);
        builder.change_clock_speed(2.5);
        let res = builder.energy();
        assert_eq!((res * 10.0).round() / 10.0, 27.7);
    }

    #[test]
    fn test_builder_energy_percentage_1() {
        let mut builder = Builder::new(String::from("Constructor"), 4.0, 4.0);
        builder.change_clock_speed(0.1);
        let res = builder.energy_percentage();
        assert_eq!((res * 100.0).round() / 100.0, 0.25);
    }

    #[test]
    fn test_builder_energy_percentage_2() {
        let mut builder = Builder::new(String::from("Constructor"), 4.0, 4.0);
        builder.change_clock_speed(0.5);
        let res = builder.energy_percentage();
        assert_eq!((res * 100.0).round() / 100.0, 0.66);
    }

    #[test]
    fn test_builder_energy_percentage_3() {
        let mut builder = Builder::new(String::from("Constructor"), 4.0, 4.0);
        builder.change_clock_speed(1.0);
        let res = builder.energy_percentage();
        assert_eq!((res * 100.0).round() / 100.0, 1.0);
    }

    #[test]
    fn test_builder_energy_percentage_4() {
        let mut builder = Builder::new(String::from("Constructor"), 4.0, 4.0);
        builder.change_clock_speed(1.5);
        let res = builder.energy_percentage();
        assert_eq!((res * 100.0).round() / 100.0, 1.28);
    }

    #[test]
    fn test_builder_energy_percentage_5() {
        let mut builder = Builder::new(String::from("Constructor"), 4.0, 4.0);
        builder.change_clock_speed(2.0);
        let res = builder.energy_percentage();
        assert_eq!((res * 100.0).round() / 100.0, 1.51);
    }

    #[test]
    fn test_builder_energy_percentage_6() {
        let mut builder = Builder::new(String::from("Constructor"), 4.0, 4.0);
        builder.change_clock_speed(2.5);
        let res = builder.energy_percentage();
        assert_eq!((res * 100.0).round() / 100.0, 1.73);
    }
}
