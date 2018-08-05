//item.rs handles the item types
use building::Building;
use factory::Rate;

#[derive(Debug, PartialEq)]
pub enum Item {
    IronOre,
    IronPlate,
}

impl Item {
    // Returns a list of ingredients at rates, and the buildings required.  Raw ingredients return empty vectors
    pub fn recipe(&self, rate: Rate) -> (Vec<(Item, Rate)>, Vec<Building>) {
        use self::Item::*;
        match self {
            IronOre =>(Vec::new(), Vec::new()),
            IronPlate => (vec![(IronOre, 1 * rate)], vec![Building::Assembler]),
        }
   }
}
