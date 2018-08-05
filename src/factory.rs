// factory.rs is the top-level facotry input/output of the calculator
use building::*;
use item::*;

// this is Rate in Hz
pub type Rate = u32;

#[derive(Debug, PartialEq)]
pub struct Factory {
    pub output: (Item, Rate),
    pub buildings: Vec<(Building, u32)>,
    pub raw_usage: Vec<(Item, Rate)>,
}

impl Factory {
    pub fn new(target_item: Item, target_rate: Rate) -> Self {
        let (r, b) = target_item.recipe(target_rate);
        Self {
            output: (target_item, target_rate),
            buildings: b,
            raw_usage: r,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_factory() {
        assert_eq!(
            Factory::new(Item::IronPlate, 1),
            Factory {
                output: (Item::IronPlate, 1),
                buildings: vec![(Building::Drill, 1), (Building::Furnace, 1)],
                raw_usage: vec![(Item::IronOre, 1)],
            }
        )
    }

    #[test]
    fn test_double_basic_factory() {
        assert_eq!(
            Factory::new(Item::IronPlate, 2),
            Factory {
                output: (Item::IronPlate, 2),
                buildings: vec![(Building::Drill, 2), (Building::Furnace, 2)],
                raw_usage: vec![(Item::IronOre, 2)],
            }
        )
    }
}
