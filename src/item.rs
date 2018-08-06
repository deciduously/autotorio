//item.rs handles the item types
use building::Building;
use factory::Rate;

#[derive(Debug, PartialEq)]
pub enum Item {
    CopperOre,
    CopperPlate,
    IronOre,
    IronPlate,
}

impl Item {
    // Returns a list of ingredients at rates, and the buildings required.  Raw ingredients return empty vectors
    pub fn recipe(&self, rate: Rate) -> (Vec<(Item, Rate)>, Vec<(Building, u32)>) {
        use self::Item::*;
        match self {
            CopperOre => (Vec::new(), Vec::new()),
            CopperPlate => (
                vec![(CopperPlate, rate)],
                vec![(Building::Drill, rate), (Building::Furnace, rate)],
            ),
            IronOre => (Vec::new(), Vec::new()),
            IronPlate => (
                vec![(IronOre, rate)],
                vec![(Building::Drill, rate), (Building::Furnace, rate)],
            ),
        }
    }
}
