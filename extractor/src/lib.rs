use resource::{ResourceNode, ResourceType};

struct Extractor {
    mk_number: u8,
    resource: ResourceNode,
    clock_speed: f32
}

impl Extractor {
    pub fn new(mk_number: u8, resource_name: String,  purity: ResourceType, clock_speed: f32) -> Extractor {
        Extractor { mk_number: mk_number, resource: ResourceNode::new(resource_name, purity), clock_speed: Extractor::validate_clock_speed(clock_speed) }
    }

    pub fn extraction_rate(&self) -> f64 {
        let mut extraction_rate: f64 = self.resource.base_extraction();
        for _ in 1..self.mk_number {
            extraction_rate = extraction_rate * 2.0;
        }
        return extraction_rate * self.clock_speed as f64;
    }

    fn validate_clock_speed(clock_speed: f32) -> f32 {
        if clock_speed < 0.0 || clock_speed > 2.5 {
            panic!("Invalid clock speed, should be between 0 and 2.5 instead of {}!", clock_speed);
        } else {
            return clock_speed;
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_impure_mk_1_extraction_rate() {
        let extract = Extractor::new(1, String::from("Iron Ore"), ResourceType::Impure, 0.5);
        assert_eq!(extract.extraction_rate(), 15.0);
        let extract = Extractor::new(1, String::from("Iron Ore"), ResourceType::Impure, 1.0);
        assert_eq!(extract.extraction_rate(), 30.0);
        let extract = Extractor::new(1, String::from("Iron Ore"), ResourceType::Impure, 1.5);
        assert_eq!(extract.extraction_rate(), 45.0);
        let extract = Extractor::new(1, String::from("Iron Ore"), ResourceType::Impure, 2.0);
        assert_eq!(extract.extraction_rate(), 60.0);
        let extract = Extractor::new(1, String::from("Iron Ore"), ResourceType::Impure, 2.5);
        assert_eq!(extract.extraction_rate(), 75.0);
    }

    #[test]
    fn test_impure_mk_2_extraction_rate() {
        let extract = Extractor::new(2, String::from("Iron Ore"), ResourceType::Impure, 0.5);
        assert_eq!(extract.extraction_rate(), 30.0);
        let extract = Extractor::new(2, String::from("Iron Ore"), ResourceType::Impure, 1.0);
        assert_eq!(extract.extraction_rate(), 60.0);
        let extract = Extractor::new(2, String::from("Iron Ore"), ResourceType::Impure, 1.5);
        assert_eq!(extract.extraction_rate(), 90.0);
        let extract = Extractor::new(2, String::from("Iron Ore"), ResourceType::Impure, 2.0);
        assert_eq!(extract.extraction_rate(), 120.0);
        let extract = Extractor::new(2, String::from("Iron Ore"), ResourceType::Impure, 2.5);
        assert_eq!(extract.extraction_rate(), 150.0);
    }

    #[test]
    fn test_impure_mk_3_extraction_rate() {
        let extract = Extractor::new(3, String::from("Iron Ore"), ResourceType::Impure, 0.5);
        assert_eq!(extract.extraction_rate(), 60.0);
        let extract = Extractor::new(3, String::from("Iron Ore"), ResourceType::Impure, 1.0);
        assert_eq!(extract.extraction_rate(), 120.0);
        let extract = Extractor::new(3, String::from("Iron Ore"), ResourceType::Impure, 1.5);
        assert_eq!(extract.extraction_rate(), 180.0);
        let extract = Extractor::new(3, String::from("Iron Ore"), ResourceType::Impure, 2.0);
        assert_eq!(extract.extraction_rate(), 240.0);
        let extract = Extractor::new(3, String::from("Iron Ore"), ResourceType::Impure, 2.5);
        assert_eq!(extract.extraction_rate(), 300.0);
    }

    #[test]
    fn test_normal_mk_1_extraction_rate() {
        let extract = Extractor::new(1, String::from("Iron Ore"), ResourceType::Normal, 0.5);
        assert_eq!(extract.extraction_rate(), 30.0);
        let extract = Extractor::new(1, String::from("Iron Ore"), ResourceType::Normal, 1.0);
        assert_eq!(extract.extraction_rate(), 60.0);
        let extract = Extractor::new(1, String::from("Iron Ore"), ResourceType::Normal, 1.5);
        assert_eq!(extract.extraction_rate(), 90.0);
        let extract = Extractor::new(1, String::from("Iron Ore"), ResourceType::Normal, 2.0);
        assert_eq!(extract.extraction_rate(), 120.0);
        let extract = Extractor::new(1, String::from("Iron Ore"), ResourceType::Normal, 2.5);
        assert_eq!(extract.extraction_rate(), 150.0);
    }

    #[test]
    fn test_normal_mk_2_extraction_rate() {
        let extract = Extractor::new(2, String::from("Iron Ore"), ResourceType::Normal, 0.5);
        assert_eq!(extract.extraction_rate(), 60.0);
        let extract = Extractor::new(2, String::from("Iron Ore"), ResourceType::Normal, 1.0);
        assert_eq!(extract.extraction_rate(), 120.0);
        let extract = Extractor::new(2, String::from("Iron Ore"), ResourceType::Normal, 1.5);
        assert_eq!(extract.extraction_rate(), 180.0);
        let extract = Extractor::new(2, String::from("Iron Ore"), ResourceType::Normal, 2.0);
        assert_eq!(extract.extraction_rate(), 240.0);
        let extract = Extractor::new(2, String::from("Iron Ore"), ResourceType::Normal, 2.5);
        assert_eq!(extract.extraction_rate(), 300.0);
    }

    #[test]
    fn test_normal_mk_3_extraction_rate() {
        let extract = Extractor::new(3, String::from("Iron Ore"), ResourceType::Normal, 0.5);
        assert_eq!(extract.extraction_rate(), 120.0);
        let extract = Extractor::new(3, String::from("Iron Ore"), ResourceType::Normal, 1.0);
        assert_eq!(extract.extraction_rate(), 240.0);
        let extract = Extractor::new(3, String::from("Iron Ore"), ResourceType::Normal, 1.5);
        assert_eq!(extract.extraction_rate(), 360.0);
        let extract = Extractor::new(3, String::from("Iron Ore"), ResourceType::Normal, 2.0);
        assert_eq!(extract.extraction_rate(), 480.0);
        let extract = Extractor::new(3, String::from("Iron Ore"), ResourceType::Normal, 2.5);
        assert_eq!(extract.extraction_rate(), 600.0);
    }

    #[test]
    fn test_pure_mk_1_extraction_rate() {
        let extract = Extractor::new(1, String::from("Iron Ore"), ResourceType::Pure, 0.5);
        assert_eq!(extract.extraction_rate(), 60.0);
        let extract = Extractor::new(1, String::from("Iron Ore"), ResourceType::Pure, 1.0);
        assert_eq!(extract.extraction_rate(), 120.0);
        let extract = Extractor::new(1, String::from("Iron Ore"), ResourceType::Pure, 1.5);
        assert_eq!(extract.extraction_rate(), 180.0);
        let extract = Extractor::new(1, String::from("Iron Ore"), ResourceType::Pure, 2.0);
        assert_eq!(extract.extraction_rate(), 240.0);
        let extract = Extractor::new(1, String::from("Iron Ore"), ResourceType::Pure, 2.5);
        assert_eq!(extract.extraction_rate(), 300.0);
    }

    #[test]
    fn test_pure_mk_2_extraction_rate() {
        let extract = Extractor::new(2, String::from("Iron Ore"), ResourceType::Pure, 0.5);
        assert_eq!(extract.extraction_rate(), 120.0);
        let extract = Extractor::new(2, String::from("Iron Ore"), ResourceType::Pure, 1.0);
        assert_eq!(extract.extraction_rate(), 240.0);
        let extract = Extractor::new(2, String::from("Iron Ore"), ResourceType::Pure, 1.5);
        assert_eq!(extract.extraction_rate(), 360.0);
        let extract = Extractor::new(2, String::from("Iron Ore"), ResourceType::Pure, 2.0);
        assert_eq!(extract.extraction_rate(), 480.0);
        let extract = Extractor::new(2, String::from("Iron Ore"), ResourceType::Pure, 2.5);
        assert_eq!(extract.extraction_rate(), 600.0);
    }

    #[test]
    fn test_pure_mk_3_extraction_rate() {
        let extract = Extractor::new(3, String::from("Iron Ore"), ResourceType::Pure, 0.5);
        assert_eq!(extract.extraction_rate(), 240.0);
        let extract = Extractor::new(3, String::from("Iron Ore"), ResourceType::Pure, 1.0);
        assert_eq!(extract.extraction_rate(), 480.0);
        let extract = Extractor::new(3, String::from("Iron Ore"), ResourceType::Pure, 1.5);
        assert_eq!(extract.extraction_rate(), 720.0);
        let extract = Extractor::new(3, String::from("Iron Ore"), ResourceType::Pure, 2.0);
        assert_eq!(extract.extraction_rate(), 960.0);
        let extract = Extractor::new(3, String::from("Iron Ore"), ResourceType::Pure, 2.5);
        assert_eq!(extract.extraction_rate(), 1200.0);
    }
}
