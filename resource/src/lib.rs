struct Impure;
struct Normal;
struct Pure;


pub enum ResourceType {
    Impure,
    Normal,
    Pure,
}


pub struct ResourceNode {
    name: String,
    resource_type: ResourceType
}

impl ResourceNode {
    pub fn new(name: String, resource_type: ResourceType) -> ResourceNode {
        ResourceNode { name: name, resource_type: resource_type }
    }

    pub fn base_extraction(&self) ->f64 {
        match self.resource_type {
            ResourceType::Impure => 30.0,
            ResourceType::Normal => 60.0,
            ResourceType::Pure => 120.0,
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_impure_node() {
        let node = ResourceNode::new(String::from("Iron Ore"), ResourceType::Impure);
        assert_eq!(node.base_extraction(), 30.0);
    }

    #[test]
    fn test_normal_node() {
        let node = ResourceNode::new(String::from("Iron Ore"), ResourceType::Normal);
        assert_eq!(node.base_extraction(), 60.0);
    }

    #[test]
    fn test_pure_node() {
        let node = ResourceNode::new(String::from("Iron Ore"), ResourceType::Pure);
        assert_eq!(node.base_extraction(), 120.0);
    }
}
