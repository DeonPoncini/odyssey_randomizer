use crate::kingdom::KingdomName;
use crate::state::State;

pub type MoonID = usize;

pub struct Moon {
    name: String,
    kingdom: KingdomName,
    prerequisite_kingdoms: Vec<(KingdomName, u8)>,
    prerequisite_moons: Vec<MoonID>,
}

impl Moon {
    fn new(name: &str, kingdom: KingdomName) -> Self {
        Moon {
            name: String::from(name),
            kingdom: kingdom,
            prerequisite_kingdoms: Vec::new(),
            prerequisite_moons: Vec::new(),
        }
    }

    fn add_prereq_kingdom(&mut self, kingdom: KingdomName) {
        self.prerequisite_kingdoms.push((kingdom, 1));
    }

    fn add_prereq_kingdom_count(&mut self, kingdom: KingdomName, visited: u8) {
        self.prerequisite_kingdoms.push((kingdom, visited));
    }

    fn add_prereq_moon(&mut self, moon: MoonID) {
        self.prerequisite_moons.push(moon);
    }

    pub fn available(&self, state: &State) -> bool {
        // current kingdom has to be the kingdom this moon is in
        if state.current_kingdom() != self.kingdom {
            return false;
        }

        // all prerequisite kingdoms must be scheduled
        for (p, c) in &self.prerequisite_kingdoms {
            if !state.kingdom_scheduled(*p, *c) {
                return false;
            }
        }

        // all prerequisite moons must be scheduled
        for p in &self.prerequisite_moons {
            if !state.moon_scheduled(*p) {
                return false;
            }
        }

        true
    }
}

pub struct Moons {
    moons: Vec<Moon>,
}

impl Moons {
    pub fn new() -> Self {
        let mut moons = Vec::new();
        // cap kingdom
        let cap1 = moons.len();
        moons.push(Moon::new("Frog-Jumping Above the Fog", KingdomName::Cap));
        moons[cap1].add_prereq_kingdom(KingdomName::Cascade);
        let cap2 = moons.len();
        moons.push(Moon::new("Frog-Jumping from the Top Deck", KingdomName::Cap));
        moons[cap2].add_prereq_kingdom(KingdomName::Cascade);
        let cap3 = moons.len();
        moons.push(Moon::new("Cap Kingdom Timer Challenge 1", KingdomName::Cap));
        moons[cap3].add_prereq_kingdom(KingdomName::Cascade);
        let cap4 = moons.len();
        moons.push(Moon::new("Good Evening, Captain Toad!", KingdomName::Cap));
        moons[cap4].add_prereq_kingdom(KingdomName::Cascade);
        let cap5 = moons.len();
        moons.push(Moon::new("Shopping in Bonneton", KingdomName::Cap));
        moons[cap5].add_prereq_kingdom(KingdomName::Cascade);
        let cap6 = moons.len();
        moons.push(Moon::new("Skimming the Poison Tide", KingdomName::Cap));
        moons[cap6].add_prereq_kingdom(KingdomName::Cascade);
        let cap7 = moons.len();
        moons.push(Moon::new("Slipping Through the Poison Tide", KingdomName::Cap));
        moons[cap7].add_prereq_kingdom(KingdomName::Cascade);
        let cap8 = moons.len();
        moons.push(Moon::new("Push-Block Peril", KingdomName::Cap));
        moons[cap8].add_prereq_kingdom(KingdomName::Cascade);
        let cap9 = moons.len();
        moons.push(Moon::new("Hidden Among the Push-Blocks", KingdomName::Cap));
        moons[cap9].add_prereq_kingdom(KingdomName::Cascade);
        let cap10 = moons.len();
        moons.push(Moon::new("Searching the Frog Pond", KingdomName::Cap));
        moons[cap10].add_prereq_kingdom(KingdomName::Cascade);
        let cap11 = moons.len();
        moons.push(Moon::new("Secrets of the Frog Pond", KingdomName::Cap));
        moons[cap11].add_prereq_kingdom(KingdomName::Cascade);
        let cap12 = moons.len();
        moons.push(Moon::new("The Forgotten Treasure", KingdomName::Cap));
        moons[cap12].add_prereq_kingdom(KingdomName::Mushroom);
        let cap13 = moons.len();
        moons.push(Moon::new("Taxi Flying Through Bonneton", KingdomName::Cap));
        moons[cap13].add_prereq_kingdom(KingdomName::Mushroom);
        let cap14 = moons.len();
        moons.push(Moon::new("Bonneter Blockade", KingdomName::Cap));
        moons[cap14].add_prereq_kingdom(KingdomName::Mushroom);
        let cap15 = moons.len();
        moons.push(Moon::new("Cap Kingdom Regular Cup", KingdomName::Cap));
        moons[cap15].add_prereq_kingdom(KingdomName::Mushroom);
        let cap16 = moons.len();
        moons.push(Moon::new("Peach in the Cap Kingdom", KingdomName::Cap));
        moons[cap16].add_prereq_kingdom(KingdomName::Mushroom);
        let cap17 = moons.len();
        moons.push(Moon::new("Found with Cap Kingdom Art", KingdomName::Moon));
        moons[cap17].add_prereq_kingdom(KingdomName::Mushroom);
        moons[cap17].add_prereq_kingdom_count(KingdomName::Cap, 2);
        let cap18 = moons.len();
        moons.push(Moon::new("Next to Glasses Bridge", KingdomName::Cap));
        moons[cap18].add_prereq_kingdom(KingdomName::Mushroom);
        let cap19 = moons.len();
        moons.push(Moon::new("Danger Sign", KingdomName::Cap));
        moons[cap19].add_prereq_kingdom(KingdomName::Mushroom);
        let cap20 = moons.len();
        moons.push(Moon::new("Under the Big One's Brim", KingdomName::Cap));
        moons[cap20].add_prereq_kingdom(KingdomName::Mushroom);
        let cap21 = moons.len();
        moons.push(Moon::new("Fly to the Edge of the Fog", KingdomName::Cap));
        moons[cap21].add_prereq_kingdom(KingdomName::Mushroom);
        let cap22 = moons.len();
        moons.push(Moon::new("Spin the Hat, Get a Prize", KingdomName::Cap));
        moons[cap22].add_prereq_kingdom(KingdomName::Mushroom);
        let cap23 = moons.len();
        moons.push(Moon::new("Hidden in a Sunken Hat", KingdomName::Cap));
        moons[cap23].add_prereq_kingdom(KingdomName::Mushroom);
        let cap24 = moons.len();
        moons.push(Moon::new("Fog-Shrouded Platform", KingdomName::Cap));
        moons[cap24].add_prereq_kingdom(KingdomName::Mushroom);
        let cap25 = moons.len();
        moons.push(Moon::new("Fog-Shrouded Platform", KingdomName::Cap));
        moons[cap25].add_prereq_kingdom(KingdomName::Mushroom);
        let cap26 = moons.len();
        moons.push(Moon::new("Caught Hopping Near the Ship!", KingdomName::Cap));
        moons[cap26].add_prereq_kingdom(KingdomName::Mushroom);
        let cap27 = moons.len();
        moons.push(Moon::new("Taking Notes: In the Fog", KingdomName::Cap));
        moons[cap27].add_prereq_kingdom(KingdomName::Mushroom);
        let cap28 = moons.len();
        moons.push(Moon::new("Cap Kingdom Timer Challenge 2", KingdomName::Cap));
        moons[cap28].add_prereq_kingdom(KingdomName::Mushroom);
        let cap29 = moons.len();
        moons.push(Moon::new("Cap Kingdom Master Cup", KingdomName::Cap));
        moons[cap29].add_prereq_kingdom(KingdomName::Mushroom);
        moons[cap29].add_prereq_moon(cap15);
        let cap30 = moons.len();
        moons.push(Moon::new("Roll On and On", KingdomName::Cap));
        moons[cap30].add_prereq_kingdom(KingdomName::Mushroom);
        let cap31 = moons.len();
        moons.push(Moon::new("Precision Rolling", KingdomName::Cap));
        moons[cap31].add_prereq_kingdom(KingdomName::Mushroom);

        Moons {
            moons: moons,
        }
    }

}
