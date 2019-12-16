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
        let lake27 = moons.len();
        moons.push(Moon::new("Found with Lake Kingdom Art", KingdomName::Cascade));
        moons[lake27].add_prereq_kingdom(KingdomName::Lake);
        moons[lake27].add_prereq_kingdom_count(KingdomName::Cascade, 2);
        offset.push((cascade1, moons.len()));

        // sand kingdom
        let sand1 = moons.len();
        moons.push(Moon::new("Atop the Highest Tower", KingdomName::Sand));
        let sand2 = moons.len();
        moons.push(Moon::new("Moon Shards in the Sand", KingdomName::Sand));
        moons[sand2].add_prereq_moon(sand1);
        let sand3 = moons.len();
        moons.push(Moon::new_multi("Showdown on the Inverted Pyramid", KingdomName::Sand));
        moons[sand3].add_prereq_moon(sand2);
        let sand4 = moons.len();
        moons.push(Moon::new_multi("The Hole in the Desert", KingdomName::Sand));
        moons[sand4].add_prereq_moon(sand3);
        moons.push(Moon::new("Overlooking the Desert Town", KingdomName::Sand));
        moons.push(Moon::new("Alcove in the Ruins", KingdomName::Sand));
        moons.push(Moon::new("On the Leaning Pillar", KingdomName::Sand));
        moons.push(Moon::new("Hidden Room in the Flowing Sands", KingdomName::Sand));
        moons.push(Moon::new("Secret of the Mural", KingdomName::Sand));
        let sand10 = moons.len();
        moons.push(Moon::new("Secret of the Inverted Mural", KingdomName::Sand));
        moons[sand10].add_prereq_moon(sand2);
        moons.push(Moon::new("On Top of the Stone Archway", KingdomName::Sand));
        moons.push(Moon::new("From a Crate in the Ruins", KingdomName::Sand));
        moons.push(Moon::new("On the Lone Pillar", KingdomName::Sand));
        let sand14 = moons.len();
        moons.push(Moon::new("On the Statue's Tail", KingdomName::Sand));
        moons[sand14].add_prereq_moon(sand2);
        let sand15 = moons.len();
        moons.push(Moon::new("Hang Your Hat on the Fountain", KingdomName::Sand));
        moons[sand15].add_prereq_moon(sand4);
        moons.push(Moon::new("Where the Birds Gather", KingdomName::Sand));
        moons.push(Moon::new("Top of a Dune", KingdomName::Sand));
        moons.push(Moon::new("Lost in the Luggage", KingdomName::Sand));
        let sand19 = moons.len();
        moons.push(Moon::new("Bullet Bill Breakthrough", KingdomName::Sand));
        moons[sand19].add_prereq_moon(sand4);
        moons.push(Moon::new("Inside a Block is a Hard Place", KingdomName::Sand));
        moons.push(Moon::new("Bird Traveling the Desert", KingdomName::Sand));
        let sand22 = moons.len();
        moons.push(Moon::new("Bird Traveling the Wastes", KingdomName::Sand));
        moons[sand22].add_prereq_moon(sand4);
        let sand23 = moons.len();
        moons.push(Moon::new("The Lurker Under the Stone", KingdomName::Sand));
        moons[sand23].add_prereq_moon(sand4);
        moons.push(Moon::new("The Treasure of Jaxi Ruins", KingdomName::Sand));
        moons.push(Moon::new("Desert Gardening: Plaza Seed", KingdomName::Sand));
        moons.push(Moon::new("Desert Gardening: Ruins Seed", KingdomName::Sand));
        moons.push(Moon::new("Desert Gardening: Seed on the Cliff", KingdomName::Sand));
        let sand28 = moons.len();
        moons.push(Moon::new("Sand Kingdom Timer Challenge 1", KingdomName::Sand));
        moons[sand28].add_prereq_moon(sand4);
        let sand29 = moons.len();
        moons.push(Moon::new("Sand Kingdom Timer Challenge 2", KingdomName::Sand));
        moons[sand29].add_prereq_moon(sand4);
        let sand30 = moons.len();
        moons.push(Moon::new("Sand Kingdom Timer Challenge 3", KingdomName::Sand));
        moons[sand30].add_prereq_moon(sand4);
        let sand31 = moons.len();
        moons.push(Moon::new("Found in the Sand! Good Dog!", KingdomName::Sand));
        moons[sand31].add_prereq_moon(sand4);
        moons.push(Moon::new("Taking Notes: Jump on the Palm", KingdomName::Sand));
        moons.push(Moon::new("Herding Sheep in the Dunes", KingdomName::Sand));
        let sand34 = moons.len();
        moons.push(Moon::new("Fishing in the Oasis", KingdomName::Sand));
        moons[sand34].add_prereq_moon(sand4);
        let sand35 = moons.len();
        moons.push(Moon::new("Love in the Heart of the Desert", KingdomName::Sand));
        moons[sand35].add_prereq_moon(sand4);
        moons.push(Moon::new("Among the Five Cactuses", KingdomName::Sand));
        let sand37 = moons.len();
        moons.push(Moon::new("You're Quite a Catch, Captain Toad!", KingdomName::Sand));
        moons[sand37].add_prereq_moon(sand4);
        let sand38 = moons.len();
        moons.push(Moon::new("Jaxi Reunion!", KingdomName::Sand));
        moons[sand38].add_prereq_moon(sand4);
        let sand39 = moons.len();
        moons.push(Moon::new("Welcome Back, Jaxi!", KingdomName::Sand));
        moons[sand39].add_prereq_moon(sand4);
        moons.push(Moon::new("Wandering Cactus", KingdomName::Sand));
        moons.push(Moon::new("Sand Quiz: Wonderful!", KingdomName::Sand));
        moons.push(Moon::new("Shopping in Tostarena", KingdomName::Sand));
        moons.push(Moon::new("Employees Only", KingdomName::Sand));
        moons.push(Moon::new("Sand Kingdom Slots", KingdomName::Sand));
        let sand45 = moons.len();
        moons.push(Moon::new("Walking the Desert", KingdomName::Sand));
        let sand46 = moons.len();
        moons.push(Moon::new("Hidden Room in the Inverted Pyramid", KingdomName::Sand));
        moons[sand46].add_prereq_moon(sand2);
        let sand47 = moons.len();
        moons.push(Moon::new("Underground Treasure Chest", KingdomName::Sand));
        moons[sand47].add_prereq_moon(sand3);
        let sand48 = moons.len();
        moons.push(Moon::new("Goomba Tower Assembly", KingdomName::Sand));
        moons[sand48].add_prereq_moon(sand3);
        let sand49 = moons.len();
        moons.push(Moon::new("Under the Mummy's Curse", KingdomName::Sand));
        moons[sand49].add_prereq_moon(sand4);
        moons.push(Moon::new("Ice Cave Treasure", KingdomName::Sand));
        moons.push(Moon::new("Sphynx's Treasure Vault", KingdomName::Sand));
        moons.push(Moon::new("A Rumble from the Sandy Floor", KingdomName::Sand));
        moons.push(Moon::new("Dancing with New Friends", KingdomName::Sand));
        moons.push(Moon::new("The Invisible Maze", KingdomName::Sand));
        moons.push(Moon::new("Skull Sign in the Transparent Maze", KingdomName::Sand));
        moons.push(Moon::new("The Bullet Bill Maze: Break Through!", KingdomName::Sand));
        moons.push(Moon::new("The Bullet Bill Maze: Side Path", KingdomName::Sand));
        moons.push(Moon::new("Jaxi Driver", KingdomName::Sand));
        moons.push(Moon::new("Jaxi Stunt Driving", KingdomName::Sand));
        let sand60 = moons.len();
        moons.push(Moon::new("Strange Neighborhood", KingdomName::Sand));
        moons[sand60].add_prereq_moon(sand4);
        let sand61 = moons.len();
        moons.push(Moon::new("Above a Strange Neighborhood", KingdomName::Sand));
        moons[sand61].add_prereq_moon(sand4);
        let sand62 = moons.len();
        moons.push(Moon::new("Secret Path to Tostarena!", KingdomName::Sand));
        moons[sand62].add_prereq_kingdom(KingdomName::Lake);
        // TODO: requires wooded kingdom story moon
        let sand64 = moons.len();
        moons.push(Moon::new("Jammin' in the Sand Kingdom", KingdomName::Sand));
        moons[sand64].add_prereq_kingdom(KingdomName::Mushroom);
        let sand65 = moons.len();
        moons.push(Moon::new("Hat-and-Seek: In the Sand", KingdomName::Sand));
        moons[sand65].add_prereq_kingdom(KingdomName::Mushroom);
        let sand66 = moons.len();
        moons.push(Moon::new("Sand Kingdom Regular Cup", KingdomName::Sand));
        moons[sand66].add_prereq_kingdom(KingdomName::Mushroom);
        let sand67 = moons.len();
        moons.push(Moon::new("Binding Band Returned", KingdomName::Sand));
        moons[sand67].add_prereq_kingdom(KingdomName::Mushroom);
        let sand68 = moons.len();
        moons.push(Moon::new("Round-the-World Tourist", KingdomName::Sand));
        // TODO: requires mushroom kingdom tourist
        let sand69 = moons.len();
        moons.push(Moon::new("Peach in the Sand Kingdom", KingdomName::Sand));
        moons[sand69].add_prereq_kingdom(KingdomName::Mushroom);
        let sand70 = moons.len();
        moons.push(Moon::new("Mighty Leap from the Palm Tree!", KingdomName::Sand));
        moons[sand70].add_prereq_kingdom(KingdomName::Mushroom);
        let sand71 = moons.len();
        moons.push(Moon::new("On the North Pillar", KingdomName::Sand));
        moons[sand71].add_prereq_kingdom(KingdomName::Mushroom);
        let sand72 = moons.len();
        moons.push(Moon::new("Into the Flowing Sands", KingdomName::Sand));
        moons[sand72].add_prereq_kingdom(KingdomName::Mushroom);
        let sand73 = moons.len();
        moons.push(Moon::new("In the Skies Above the Canyon", KingdomName::Sand));
        moons[sand73].add_prereq_kingdom(KingdomName::Mushroom);
        let sand74 = moons.len();
        moons.push(Moon::new("Island in the Poison Swamp", KingdomName::Sand));
        moons[sand74].add_prereq_kingdom(KingdomName::Mushroom);
        let sand75 = moons.len();
        moons.push(Moon::new("An Invisible Gleam", KingdomName::Sand));
        moons[sand75].add_prereq_kingdom(KingdomName::Mushroom);
        let sand76 = moons.len();
        moons.push(Moon::new("On the Eastern Pillar", KingdomName::Sand));
        moons[sand76].add_prereq_kingdom(KingdomName::Mushroom);
        let sand77 = moons.len();
        moons.push(Moon::new("Caught Hopping in the Desert!", KingdomName::Sand));
        moons[sand77].add_prereq_kingdom(KingdomName::Mushroom);
        let sand78 = moons.len();
        moons.push(Moon::new("Poster Cleanup", KingdomName::Sand));
        moons[sand78].add_prereq_kingdom(KingdomName::Mushroom);
        let sand79 = moons.len();
        moons.push(Moon::new("Taking Notes: Running Down", KingdomName::Sand));
        moons[sand79].add_prereq_kingdom(KingdomName::Mushroom);
        let sand80 = moons.len();
        moons.push(Moon::new("Taking Notes: In the Wall Painting", KingdomName::Sand));
        moons[sand80].add_prereq_kingdom(KingdomName::Mushroom);
        let sand81 = moons.len();
        moons.push(Moon::new("Love at the Edge of the Desert", KingdomName::Sand));
        moons[sand81].add_prereq_kingdom(KingdomName::Mushroom);
        let sand82 = moons.len();
        moons.push(Moon::new("More Walking in the Desert!", KingdomName::Sand));
        moons[sand82].add_prereq_kingdom(KingdomName::Mushroom);
        moons[sand82].add_prereq_moon(sand45);
        let sand83 = moons.len();
        moons.push(Moon::new("Sand Kingdom Master Cup", KingdomName::Sand));
        moons[sand83].add_prereq_kingdom(KingdomName::Mushroom);
        moons[sand83].add_prereq_moon(sand66);
        let sand84 = moons.len();
        moons.push(Moon::new("Where the Transparent Platforms End", KingdomName::Sand));
        moons[sand84].add_prereq_kingdom(KingdomName::Mushroom);
        let sand85 = moons.len();
        moons.push(Moon::new("Jump Onto the Transparent Lift", KingdomName::Sand));
        moons[sand85].add_prereq_kingdom(KingdomName::Mushroom);
        let sand86 = moons.len();
        moons.push(Moon::new("Colossal Ruins: Dash! Jump!", KingdomName::Sand));
        moons[sand86].add_prereq_kingdom(KingdomName::Mushroom);
        let sand87 = moons.len();
        moons.push(Moon::new("Sinking Colossal Ruins: Hurry!", KingdomName::Sand));
        moons[sand87].add_prereq_kingdom(KingdomName::Mushroom);
        let sand88 = moons.len();
        moons.push(Moon::new("Through the Freezing Waterway", KingdomName::Sand));
        moons[sand88].add_prereq_kingdom(KingdomName::Mushroom);
        let sand89 = moons.len();
        moons.push(Moon::new("Freezing Waterway: Hidden Room", KingdomName::Sand));
        moons[sand89].add_prereq_kingdom(KingdomName::Mushroom);
        let wooded50 = moons.len();
        moons.push(Moon::new("Found with Wooded Kingdom Art", KingdomName::Sand));
        moons[wooded50].add_prereq_kingdom(KingdomName::Wooded);
        moons[wooded50].add_prereq_kingdom_count(KingdomName::Sand, 2);
        offset.push((sand1, moons.len()));

        // lake kingdom
        let lake1 = moons.len();
        moons.push(Moon::new_multi("Broodals Over the Lake", KingdomName::Lake));
        moons.push(Moon::new("Dorrie-Back Rider", KingdomName::Lake));
        moons.push(Moon::new("Cheep Cheep Crossing", KingdomName::Lake));
        moons.push(Moon::new("End of the Hidden Passage", KingdomName::Lake));
        moons.push(Moon::new("What's in the Box?", KingdomName::Lake));
        moons.push(Moon::new("On the Lakeshore", KingdomName::Lake));
        moons.push(Moon::new("From the Broken Pillar", KingdomName::Lake));
        moons.push(Moon::new("Treasure in the Spiky Waterway", KingdomName::Lake));
        let lake9 = moons.len();
        moons.push(Moon::new("Lake Gardening: Spiky Passage Seed", KingdomName::Lake));
        moons[lake9].add_prereq_moon(lake1);
        let lake10 = moons.len();
        moons.push(Moon::new("Lake Kingdom Timer Challenge 1", KingdomName::Lake));
        moons[lake10].add_prereq_moon(lake1);
        let lake11 = moons.len();
        moons.push(Moon::new("Lake Kingdom Timer Challenge 2", KingdomName::Lake));
        moons[lake11].add_prereq_moon(lake1);
        moons.push(Moon::new("Moon Shards in the Lake", KingdomName::Lake));
        moons.push(Moon::new("Taking Notes: Dive and Swim", KingdomName::Lake));
        moons.push(Moon::new("Taking Notes: In the Cliffside", KingdomName::Lake));
        let lake15 = moons.len();
        moons.push(Moon::new("Lake Fishing", KingdomName::Lake));
        moons[lake15].add_prereq_moon(lake1);
        let lake16 = moons.len();
        moons.push(Moon::new("I Met a Lake Cheep Cheep!", KingdomName::Lake));
        moons[lake16].add_prereq_moon(lake1);
        moons.push(Moon::new("Our Secret Little Room", KingdomName::Lake));
        moons.push(Moon::new("Let's Go Swimming, Captain Toad!", KingdomName::Lake));
        moons.push(Moon::new("Shopping in Lake Lamode", KingdomName::Lake));
        let lake20 = moons.len();
        moons.push(Moon::new("A Successful Repair Job", KingdomName::Lake));
        moons[lake20].add_prereq_moon(lake1);
        moons.push(Moon::new("I Feel Underdressed", KingdomName::Lake));
        moons.push(Moon::new("Unzip the Chasm", KingdomName::Lake));
        moons.push(Moon::new("Super-Secret Zipper", KingdomName::Lake));
        moons.push(Moon::new("Jump, Grab, Cling, and Climb", KingdomName::Lake));
        let lake26 = moons.len();
        moons.push(Moon::new("Secret Path to Lake Lamode!", KingdomName::Lake));
        // TODO: add dependency on snow, seaside, metro moons
        moons[lake27].add_prereq_moon(lake1);
        let lake28 = moons.len();
        moons.push(Moon::new("Taxi Flying Through Lake Lamode", KingdomName::Lake));
        moons[lake28].add_prereq_kingdom(KingdomName::Mushroom);
        let lake29 = moons.len();
        moons.push(Moon::new("That Trendy \"Pirate\" Look", KingdomName::Lake));
        moons[lake29].add_prereq_kingdom(KingdomName::Mushroom);
        let lake30 = moons.len();
        moons.push(Moon::new("Space Is \"In\" Right Now", KingdomName::Lake));
        moons[lake30].add_prereq_kingdom(KingdomName::Mushroom);
        let lake31 = moons.len();
        moons.push(Moon::new("That \"Old West\" Style", KingdomName::Lake));
        moons[lake31].add_prereq_kingdom(KingdomName::Mushroom);
        let lake32 = moons.len();
        moons.push(Moon::new("Lake Kingdom Regular Cup", KingdomName::Lake));
        moons[lake32].add_prereq_kingdom(KingdomName::Mushroom);
        let lake33 = moons.len();
        moons.push(Moon::new("Peach in the Lake Kingdom", KingdomName::Lake));
        moons[lake33].add_prereq_kingdom(KingdomName::Mushroom);
        let lake34 = moons.len();
        moons.push(Moon::new("Behind the Floodgate", KingdomName::Lake));
        moons[lake34].add_prereq_kingdom(KingdomName::Mushroom);
        let lake35 = moons.len();
        moons.push(Moon::new("High-Flying Leap", KingdomName::Lake));
        moons[lake35].add_prereq_kingdom(KingdomName::Mushroom);
        let lake36 = moons.len();
        moons.push(Moon::new("Deep, Deep Down", KingdomName::Lake));
        moons[lake36].add_prereq_kingdom(KingdomName::Mushroom);
        let lake37 = moons.len();
        moons.push(Moon::new("Rooftop of the Water Plaza", KingdomName::Lake));
        moons[lake37].add_prereq_kingdom(KingdomName::Mushroom);
        let lake38 = moons.len();
        moons.push(Moon::new("Bird Traveling Over the Lake", KingdomName::Lake));
        moons[lake38].add_prereq_kingdom(KingdomName::Mushroom);
        let lake39 = moons.len();
        moons.push(Moon::new("Love by the Lake", KingdomName::Lake));
        moons[lake39].add_prereq_kingdom(KingdomName::Mushroom);
        let lake40 = moons.len();
        moons.push(Moon::new("Lake Kingdom Master Cup", KingdomName::Lake));
        moons[lake40].add_prereq_kingdom(KingdomName::Mushroom);
        moons[lake40].add_prereq_moon(lake32);
        let lake41 = moons.len();
        moons.push(Moon::new("Waves of Poison: Hoppin' Over", KingdomName::Lake));
        moons[lake41].add_prereq_kingdom(KingdomName::Mushroom);
        let lake42 = moons.len();
        moons.push(Moon::new("Waves of Poison: Hop to It!", KingdomName::Lake));
        moons[lake42].add_prereq_kingdom(KingdomName::Mushroom);
        offset.push((lake1, moons.len()));

        // wooded kingdom
        let wooded1 = moons.len();
        moons.push(Moon::new("Road to Sky Garden", KingdomName::Wooded));
        let wooded2 = moons.len();
        moons.push(Moon::new_multi("Flower Thieves of Sky Garden", KingdomName::Wooded));
        moons[wooded2].add_prereq_moon(wooded1);
        let wooded3 = moons.len();
        moons.push(Moon::new("Path to the Secret Flower Field", KingdomName::Wooded));
        moons[wooded3].add_prereq_moon(wooded2);
        let wooded4 = moons.len();
        moons.push(Moon::new_multi("Defend the Secret Flower Field!", KingdomName::Wooded));
        moons[wooded4].add_prereq_moon(wooded3);
        let wooded5 = moons.len();
        moons.push(Moon::new("Behind the Rock Wall", KingdomName::Wooded));
        moons[wooded5].add_prereq_moon(wooded2);
        let wooded6 = moons.len();
        moons.push(Moon::new("Back Way Up the Mountain", KingdomName::Wooded));
        moons[wooded6].add_prereq_moon(wooded2);
        moons.push(Moon::new("Rolling Rock in the Woods", KingdomName::Wooded));
        moons.push(Moon::new("Caught Hopping in the Forest!", KingdomName::Wooded));
        let wooded9 = moons.len();
        moons.push(Moon::new("Thanks for the Charge!", KingdomName::Wooded));
        moons[wooded9].add_prereq_moon(wooded2);
        moons.push(Moon::new("Atop the Tall Tree", KingdomName::Wooded));
        moons.push(Moon::new("Tucked Way Inside the Tunnel", KingdomName::Wooded));
        let wooded12 = moons.len();
        moons.push(Moon::new("Over the Cliff's Edge", KingdomName::Wooded));
        moons[wooded12].add_prereq_moon(wooded2);
        moons.push(Moon::new("The Nut Round the Corner", KingdomName::Wooded));
        moons.push(Moon::new("Climb the Cliff to Get the Nut", KingdomName::Wooded));
        moons.push(Moon::new("The Nut in the Red Maze", KingdomName::Wooded));
        moons.push(Moon::new("The Nut at the Dead End", KingdomName::Wooded));
        let wooded17 = moons.len();
        moons.push(Moon::new("Cracked Nut on a Crumbling Tower", KingdomName::Wooded));
        moons[wooded17].add_prereq_moon(wooded1);
        let wooded18 = moons.len();
        moons.push(Moon::new("The Nut that Grew on the Tall Fence", KingdomName::Wooded));
        moons[wooded18].add_prereq_moon(wooded2);
        moons.push(Moon::new("Fire in the Cave", KingdomName::Wooded));
        let wooded20 = moons.len();
        moons.push(Moon::new("Hey Out There, Captain Toad!", KingdomName::Wooded));
        moons[wooded20].add_prereq_moon(wooded4);
        let wooded21 = moons.len();
        moons.push(Moon::new("Love in the Forest Ruins", KingdomName::Wooded));
        moons[wooded21].add_prereq_moon(wooded2);
        let wooded22 = moons.len();
        moons.push(Moon::new("Inside a Rock in the Forest", KingdomName::Wooded));
        moons[wooded22].add_prereq_moon(wooded4);
        moons.push(Moon::new("Shopping in Steam Gardens", KingdomName::Wooded));
        let wooded24 = moons.len();
        moons.push(Moon::new("Nut Planted in the Tower", KingdomName::Wooded));
        moons[wooded24].add_prereq_moon(wooded1);
        let wooded25 = moons.len();
        moons.push(Moon::new("Stretching Your Legs", KingdomName::Wooded));
        moons[wooded25].add_prereq_moon(wooded1);
        let wooded26 = moons.len();
        moons.push(Moon::new("Spinning-Platforms Treasure", KingdomName::Wooded));
        moons[wooded26].add_prereq_moon(wooded2);
        let wooded27 = moons.len();
        moons.push(Moon::new("Make the Secret Flower Field Bloom", KingdomName::Wooded));
        moons[wooded27].add_prereq_moon(wooded4);
        moons.push(Moon::new("Rolling Rock in the Deep Woods", KingdomName::Wooded));
        moons.push(Moon::new("Glowing in the Deep Woods", KingdomName::Wooded));
        moons.push(Moon::new("Past the Peculiar Pipes", KingdomName::Wooded));
        moons.push(Moon::new("By the Babbling Brook in Deep Woods", KingdomName::Wooded));
        moons.push(Moon::new("The Hard Rock in Deep Woods", KingdomName::Wooded));
        moons.push(Moon::new("A Treasure Made from Coins", KingdomName::Wooded));
        moons.push(Moon::new("Beneath the Roots of the Moving Tree", KingdomName::Wooded));
        moons.push(Moon::new("Deep Woods Treasure Trap", KingdomName::Wooded));
        moons.push(Moon::new("Exploring for Treasure", KingdomName::Wooded));
        let wooded37 = moons.len();
        moons.push(Moon::new("Wooded Kingdom Timer Challenge 1", KingdomName::Wooded));
        moons[wooded37].add_prereq_moon(wooded4);
        let wooded38 = moons.len();
        moons.push(Moon::new("Wooded Kingdom Timer Challenge 2", KingdomName::Wooded));
        moons[wooded38].add_prereq_moon(wooded4);
        moons.push(Moon::new("Flooding Pipeway", KingdomName::Wooded));
        moons.push(Moon::new("Flooding Pipeway Ceiling Secret", KingdomName::Wooded));
        let wooded41 = moons.len();
        moons.push(Moon::new("Wandering in the Fog", KingdomName::Wooded));
        moons[wooded41].add_prereq_moon(wooded2);
        let wooded42 = moons.len();
        moons.push(Moon::new("Nut Hidden in the Fog", KingdomName::Wooded));
        moons[wooded42].add_prereq_moon(wooded2);
        let wooded43 = moons.len();
        moons.push(Moon::new("Flower Road Run", KingdomName::Wooded));
        moons[wooded43].add_prereq_moon(wooded2);
        let wooded44 = moons.len();
        moons.push(Moon::new("Flower Road Reach", KingdomName::Wooded));
        moons[wooded44].add_prereq_moon(wooded2);
        let wooded45 = moons.len();
        moons.push(Moon::new("Elevator Escalation", KingdomName::Wooded));
        moons[wooded45].add_prereq_moon(wooded2);
        let wooded46 = moons.len();
        moons.push(Moon::new("Elevator Blind Spot", KingdomName::Wooded));
        moons[wooded46].add_prereq_moon(wooded2);
        let wooded47 = moons.len();
        moons.push(Moon::new("Walking on Clouds", KingdomName::Wooded));
        moons[wooded47].add_prereq_moon(wooded4);
        let wooded48 = moons.len();
        moons.push(Moon::new("Above the Clouds", KingdomName::Wooded));
        moons[wooded48].add_prereq_moon(wooded4);
        let wooded49 = moons.len();
        moons.push(Moon::new("Secret Path to Steam Gardens!", KingdomName::Wooded));
        // TODO: add dependencies on snow, seaside, metro
        let wooded51 = moons.len();
        moons.push(Moon::new("Swing Around Secret Flower Field", KingdomName::Wooded));
        moons[wooded51].add_prereq_kingdom(KingdomName::Mushroom);
        let wooded52 = moons.len();
        moons.push(Moon::new("Jammin' in the Wooded Kingdom", KingdomName::Wooded));
        moons[wooded52].add_prereq_kingdom(KingdomName::Mushroom);
        let wooded53 = moons.len();
        moons.push(Moon::new("Wooded Kingdom Regular Cup", KingdomName::Wooded));
        moons[wooded53].add_prereq_kingdom(KingdomName::Mushroom);
        let wooded54 = moons.len();
        moons.push(Moon::new("Peach in the Wooded Kingdom", KingdomName::Wooded));
        moons[wooded54].add_prereq_kingdom(KingdomName::Mushroom);
        let wooded55 = moons.len();
        moons.push(Moon::new("High Up in the Cave", KingdomName::Wooded));
        moons[wooded55].add_prereq_kingdom(KingdomName::Mushroom);
        let wooded56 = moons.len();
        moons.push(Moon::new("Lost in the Tall Trees", KingdomName::Wooded));
        moons[wooded56].add_prereq_kingdom(KingdomName::Mushroom);
        let wooded57 = moons.len();
        moons.push(Moon::new("Looking Down on the Goombas", KingdomName::Wooded));
        moons[wooded57].add_prereq_kingdom(KingdomName::Mushroom);
        let wooded58 = moons.len();
        moons.push(Moon::new("High Up on a Rock Wall", KingdomName::Wooded));
        moons[wooded58].add_prereq_kingdom(KingdomName::Mushroom);
        let wooded59 = moons.len();
        moons.push(Moon::new("The Nut in the Robot Storeroom", KingdomName::Wooded));
        moons[wooded59].add_prereq_kingdom(KingdomName::Mushroom);
        let wooded60 = moons.len();
        moons.push(Moon::new("Above the Iron Mountain Path", KingdomName::Wooded));
        moons[wooded60].add_prereq_kingdom(KingdomName::Mushroom);
        let wooded61 = moons.len();
        moons.push(Moon::new("The Nut Under the Observation Deck", KingdomName::Wooded));
        moons[wooded61].add_prereq_kingdom(KingdomName::Mushroom);
        let wooded62 = moons.len();
        moons.push(Moon::new("Bird Traveling the Forest", KingdomName::Wooded));
        moons[wooded62].add_prereq_kingdom(KingdomName::Mushroom);
        let wooded63 = moons.len();
        moons.push(Moon::new("Invader in the Sky Garden", KingdomName::Wooded));
        moons[wooded63].add_prereq_kingdom(KingdomName::Mushroom);
        let wooded64 = moons.len();
        moons.push(Moon::new("Hot, Hot, Hot from the Campfire", KingdomName::Wooded));
        moons[wooded64].add_prereq_kingdom(KingdomName::Mushroom);
        let wooded65 = moons.len();
        moons.push(Moon::new("Wooded Kingdom Timer Challenge 3", KingdomName::Wooded));
        moons[wooded65].add_prereq_kingdom(KingdomName::Mushroom);
        let wooded66 = moons.len();
        moons.push(Moon::new("Moon Shards in the Forest", KingdomName::Wooded));
        moons[wooded66].add_prereq_kingdom(KingdomName::Mushroom);
        let wooded67 = moons.len();
        moons.push(Moon::new("Taking Notes: On Top of the Wall", KingdomName::Wooded));
        moons[wooded67].add_prereq_kingdom(KingdomName::Mushroom);
        let wooded68 = moons.len();
        moons.push(Moon::new("Taking Notes: Stretching", KingdomName::Wooded));
        moons[wooded68].add_prereq_kingdom(KingdomName::Mushroom);
        let wooded69 = moons.len();
        moons.push(Moon::new("Wooded Kingdom Master Cup", KingdomName::Wooded));
        moons[wooded69].add_prereq_kingdom(KingdomName::Mushroom);
        moons[wooded69].add_prereq_moon(wooded53);
        let wooded70 = moons.len();
        moons.push(Moon::new("I Met an Uproot!", KingdomName::Wooded));
        moons[wooded70].add_prereq_kingdom(KingdomName::Mushroom);
        let wooded71 = moons.len();
        moons.push(Moon::new("Invisible Road: Danger!", KingdomName::Wooded));
        moons[wooded71].add_prereq_kingdom(KingdomName::Mushroom);
        let wooded72 = moons.len();
        moons.push(Moon::new("Invisible Road: Hidden Room", KingdomName::Wooded));
        moons[wooded72].add_prereq_kingdom(KingdomName::Mushroom);
        let wooded73 = moons.len();
        moons.push(Moon::new("Herding Sheep Above the Forest Fog", KingdomName::Wooded));
        moons[wooded73].add_prereq_kingdom(KingdomName::Mushroom);
        let wooded74 = moons.len();
        moons.push(Moon::new("Herding Sheep on the Iron Bridge", KingdomName::Wooded));
        moons[wooded74].add_prereq_kingdom(KingdomName::Mushroom);
        let wooded75 = moons.len();
        moons.push(Moon::new("Down and Back Breakdown Road", KingdomName::Wooded));
        moons[wooded75].add_prereq_kingdom(KingdomName::Mushroom);
        let wooded76 = moons.len();
        moons.push(Moon::new("Below Breakdown Road", KingdomName::Wooded));
        moons[wooded76].add_prereq_kingdom(KingdomName::Mushroom);
        offset.push((wooded1, moons.len()));

        // cloud kingdom
        let cloud1 = moons.len();
        moons.push(Moon::new("Picture Match: Basically a Goomba", KingdomName::Cloud));
        moons[cloud1].add_prereq_kingdom(KingdomName::Lost);
        moons[cloud1].add_prereq_kingdom_count(KingdomName::Cloud, 2);
        let cloud2 = moons.len();
        moons.push(Moon::new("Peach in the Cloud Kingdom", KingdomName::Cloud));
        moons[cloud2].add_prereq_kingdom(KingdomName::Mushroom);
        let cloud3 = moons.len();
        moons.push(Moon::new("Digging in the...Cloud?", KingdomName::Cloud));
        moons[cloud3].add_prereq_kingdom(KingdomName::Mushroom);
        let cloud4 = moons.len();
        moons.push(Moon::new("High, High Above the Clouds", KingdomName::Cloud));
        moons[cloud4].add_prereq_kingdom(KingdomName::Mushroom);
        let cloud5 = moons.len();
        moons.push(Moon::new("Crossing the Cloud Sea", KingdomName::Cloud));
        moons[cloud5].add_prereq_kingdom(KingdomName::Mushroom);
        let cloud6 = moons.len();
        moons.push(Moon::new("Taking Notes: Up and Down", KingdomName::Cloud));
        moons[cloud6].add_prereq_kingdom(KingdomName::Mushroom);
        let cloud7 = moons.len();
        moons.push(Moon::new("Picture Match: A Stellar Goomba", KingdomName::Cloud));
        moons[cloud7].add_prereq_kingdom(KingdomName::Mushroom);
        moons[cloud7].add_prereq_moon(cloud1);
        let cloud8 = moons.len();
        moons.push(Moon::new("King of the Cube!", KingdomName::Cloud));
        moons[cloud8].add_prereq_kingdom(KingdomName::Mushroom);
        let cloud9 = moons.len();
        moons.push(Moon::new("The Sixth Face", KingdomName::Cloud));
        moons[cloud9].add_prereq_kingdom(KingdomName::Mushroom);
        offset.push((cloud1, moons.len()));

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
        let sand63 = moons.len();
        moons.push(Moon::new("Found with Sand Kingdom Art", KingdomName::Bowser));
        moons[sand63].add_prereq_kingdom(KingdomName::Mushroom);
        moons[sand63].add_prereq_kingdom_count(KingdomName::Sand, 2);
        offset.push((sand63, moons.len()));

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
