use crate::kingdom::KingdomName;
use crate::state::State;

pub type MoonID = usize;

pub struct Moon {
    name: String,
    count: u16,
    kingdom: KingdomName,
    prerequisite_kingdoms: Vec<(KingdomName, u8)>,
    prerequisite_moons: Vec<MoonID>,
}

impl Moon {
    fn new(name: &str, kingdom: KingdomName) -> Self {
        Moon {
            name: String::from(name),
            count: 1,
            kingdom: kingdom,
            prerequisite_kingdoms: Vec::new(),
            prerequisite_moons: Vec::new(),
        }
    }

    fn new_multi(name: &str, kingdom: KingdomName) -> Self {
        Moon {
            name: String::from(name),
            count: 3,
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

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn kingdom(&self) -> KingdomName {
        self.kingdom
    }

    pub fn count(&self) -> u16 {
        self.count
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
    offset: Vec<(usize, usize)>,
    ids: Vec<MoonID>,
}

impl Moons {
    pub fn moon(&self, id: MoonID) -> &Moon {
        &self.moons[id]
    }

    pub fn return_available(&mut self, state: &mut State) -> Vec<MoonID> {
        let kingdom = state.current_kingdom();
        let (s, e) = self.offset[kingdom as usize];
        if s == e {
            // no moons left in kingdom
            state.complete_kingdom(kingdom);
            return Vec::new();
        }

        // go from start to end, check availability
        let mut ret = Vec::new();
        let mut swap_point = s;
        for x in s..e {
            let id = self.ids[x];
            if self.moons[id].available(&state) {
                ret.push(id);
                // swap x with start
                self.ids[x] = self.ids[swap_point];
                self.ids[swap_point] = id;
                swap_point += 1;
            }
        }

        // finally update start by the amount copied out
        let new_s = swap_point;
        self.offset[kingdom as usize] = (new_s, e);
        ret
    }

    pub fn new() -> Self {
        let mut moons = Vec::new();
        let mut offset = Vec::new();
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
        offset.push((cap1, moons.len()));

        // cascade kingdom
        let cascade1 = moons.len();
        moons.push(Moon::new("Our First Power Moon", KingdomName::Cascade));
        let cascade2 = moons.len();
        moons.push(Moon::new_multi("Multi Moon Atop the Falls", KingdomName::Cascade));
        moons[cascade2].add_prereq_moon(cascade1);
        let cascade3 = moons.len();
        moons.push(Moon::new("Chomp Through the Rocks", KingdomName::Cascade));
        moons[cascade3].add_prereq_moon(cascade1);
        let cascade4 = moons.len();
        moons.push(Moon::new("Behind the Waterfall", KingdomName::Cascade));
        moons[cascade4].add_prereq_moon(cascade1);
        let cascade5 = moons.len();
        moons.push(Moon::new("On Top of the Rubble", KingdomName::Cascade));
        moons[cascade5].add_prereq_moon(cascade2);
        let cascade6 = moons.len();
        moons.push(Moon::new("Treasure of the Waterfall Basin", KingdomName::Cascade));
        moons[cascade6].add_prereq_moon(cascade2);
        let cascade7 = moons.len();
        moons.push(Moon::new("Above a High Cliff", KingdomName::Cascade));
        moons[cascade7].add_prereq_moon(cascade2);
        let cascade8 = moons.len();
        moons.push(Moon::new("Across the Floating Isles", KingdomName::Cascade));
        moons[cascade8].add_prereq_moon(cascade2);
        let cascade9 = moons.len();
        moons.push(Moon::new("Cascade Kingdom Timer Challenge 1", KingdomName::Cascade));
        moons[cascade9].add_prereq_moon(cascade2);
        let cascade10 = moons.len();
        moons.push(Moon::new("Cascade Kingdom Timer Challenge 2", KingdomName::Cascade));
        moons[cascade10].add_prereq_moon(cascade2);
        let cascade11 = moons.len();
        moons.push(Moon::new("Good Morning, Captain Toad!", KingdomName::Cascade));
        moons[cascade11].add_prereq_moon(cascade2);
        let cascade12 = moons.len();
        moons.push(Moon::new("Dinosaur Nest: Big Cleanup!", KingdomName::Cascade));
        moons[cascade12].add_prereq_moon(cascade2);
        let cascade13 = moons.len();
        moons.push(Moon::new("Dinosaur Nest: Running Wild!", KingdomName::Cascade));
        moons[cascade13].add_prereq_moon(cascade2);
        let cascade14 = moons.len();
        moons.push(Moon::new("Nice Shot with the Chain Chomp!", KingdomName::Cascade));
        moons[cascade14].add_prereq_moon(cascade2);
        let cascade15 = moons.len();
        moons.push(Moon::new("Very Nice Shot with the Chain Chomp!", KingdomName::Cascade));
        moons[cascade15].add_prereq_moon(cascade2);
        let cascade16 = moons.len();
        moons.push(Moon::new("Past the Chasm Lifts", KingdomName::Cascade));
        moons[cascade16].add_prereq_moon(cascade2);
        let cascade17 = moons.len();
        moons.push(Moon::new("Hidden Chasm Passage", KingdomName::Cascade));
        moons[cascade17].add_prereq_moon(cascade2);
        let cascade18 = moons.len();
        moons.push(Moon::new("Secret Path to Fossil Falls", KingdomName::Cascade));
        // TODO: depends on snow and seaside moons - add that constraint
        let cascade19 = moons.len();
        moons.push(Moon::new("A Tourist in the Cascade Kingdom", KingdomName::Cascade));
        // TODO: depends on metro kingdom tourist moon
        let cascade20 = moons.len();
        moons.push(Moon::new("Rolling Rock by the Falls", KingdomName::Cascade));
        moons[cascade20].add_prereq_kingdom_count(KingdomName::Cascade, 2);
        let cascade21 = moons.len();
        moons.push(Moon::new("Peach in the Cascade Kingdom", KingdomName::Cascade));
        moons[cascade21].add_prereq_kingdom(KingdomName::Mushroom);
        let cascade22 = moons.len();
        moons.push(Moon::new("Cascade Kingdom Regular Cup", KingdomName::Cascade));
        moons[cascade22].add_prereq_kingdom(KingdomName::Mushroom);
        let cascade23 = moons.len();
        moons.push(Moon::new("Caveman Cave-Fan", KingdomName::Cascade));
        moons[cascade23].add_prereq_kingdom(KingdomName::Mushroom);
        let cascade24 = moons.len();
        moons.push(Moon::new("Shopping in Fossil Falls", KingdomName::Cascade));
        moons[cascade24].add_prereq_kingdom_count(KingdomName::Cascade, 2);
        let cascade25 = moons.len();
        moons.push(Moon::new("Sphynx Traveling to the Waterfall", KingdomName::Cascade));
        moons[cascade25].add_prereq_kingdom(KingdomName::Mushroom);
        let cascade26 = moons.len();
        moons.push(Moon::new("Bottom of the Waterfall Basin", KingdomName::Cascade));
        moons[cascade26].add_prereq_kingdom(KingdomName::Mushroom);
        let cascade27 = moons.len();
        moons.push(Moon::new("Just a Hat, Skip, and a Jump", KingdomName::Cascade));
        moons[cascade27].add_prereq_kingdom(KingdomName::Mushroom);
        let cascade28 = moons.len();
        moons.push(Moon::new("Treasure Under the Cliff", KingdomName::Cascade));
        moons[cascade28].add_prereq_kingdom(KingdomName::Mushroom);
        let cascade29 = moons.len();
        moons.push(Moon::new("Next to the Stone Arch", KingdomName::Cascade));
        moons[cascade29].add_prereq_kingdom(KingdomName::Mushroom);
        let cascade30 = moons.len();
        moons.push(Moon::new("Guarded by a Colossal Fossil", KingdomName::Cascade));
        moons[cascade30].add_prereq_kingdom(KingdomName::Mushroom);
        let cascade31 = moons.len();
        moons.push(Moon::new("Under the Old Electrical Pole", KingdomName::Cascade));
        moons[cascade31].add_prereq_kingdom(KingdomName::Mushroom);
        let cascade32 = moons.len();
        moons.push(Moon::new("Under the Ground", KingdomName::Cascade));
        moons[cascade32].add_prereq_kingdom(KingdomName::Mushroom);
        let cascade33 = moons.len();
        moons.push(Moon::new("Inside the Busted Fossil", KingdomName::Cascade));
        moons[cascade33].add_prereq_kingdom(KingdomName::Mushroom);
        let cascade34 = moons.len();
        moons.push(Moon::new("Caught Hopping at the Waterfall", KingdomName::Cascade));
        moons[cascade34].add_prereq_kingdom(KingdomName::Mushroom);
        let cascade35 = moons.len();
        moons.push(Moon::new("Taking Notes: Hurry Upward", KingdomName::Cascade));
        moons[cascade35].add_prereq_kingdom(KingdomName::Mushroom);
        let cascade36 = moons.len();
        moons.push(Moon::new("Cascade Kingdom Master Cup", KingdomName::Cascade));
        moons[cascade36].add_prereq_kingdom(KingdomName::Mushroom);
        moons[cascade22].add_prereq_moon(cascade22);
        let cascade37 = moons.len();
        moons.push(Moon::new("Across the Mysterious Clouds", KingdomName::Cascade));
        moons[cascade37].add_prereq_kingdom(KingdomName::Mushroom);
        let cascade38 = moons.len();
        moons.push(Moon::new("Atop a Wall Among the Clouds", KingdomName::Cascade));
        moons[cascade38].add_prereq_kingdom(KingdomName::Mushroom);
        let cascade39 = moons.len();
        moons.push(Moon::new("Across the Gusty Bridges", KingdomName::Cascade));
        moons[cascade39].add_prereq_kingdom(KingdomName::Mushroom);
        let cascade40 = moons.len();
        moons.push(Moon::new("Flying Far Away from Gusty Bridges", KingdomName::Cascade));
        moons[cascade40].add_prereq_kingdom(KingdomName::Mushroom);
        offset.push((cascade1, moons.len()));

        // sand kingdom
        offset.push((0, 0));

        // lake kingdom
        offset.push((0, 0));

        // wooded kingdom
        offset.push((0, 0));

        // cloud kingdom
        offset.push((0, 0));

        // lost kingdom
        offset.push((0, 0));

        // metro kingdom
        offset.push((0, 0));

        // snow kingdom
        offset.push((0, 0));

        // seaside kingdom
        offset.push((0, 0));

        // luncheon kingdom
        offset.push((0, 0));

        // ruined kingdom
        offset.push((0, 0));

        // bowser's kingdom
        offset.push((0, 0));

        // moon kingdom
        let cap17 = moons.len();
        moons.push(Moon::new("Found with Cap Kingdom Art", KingdomName::Moon));
        moons[cap17].add_prereq_kingdom(KingdomName::Mushroom);
        moons[cap17].add_prereq_kingdom_count(KingdomName::Cap, 2);
        offset.push((cap17, moons.len()));

        // dark side
        offset.push((0, 0));

        // darker side
        offset.push((0, 0));

        // setup the ids - initially monotonic
        let mut ids = Vec::new();
        for x in 0..moons.len() {
            ids.push(x);
        }

        Moons {
            moons: moons,
            offset: offset,
            ids: ids,
        }
    }

}
