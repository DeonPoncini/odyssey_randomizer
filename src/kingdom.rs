use std::str::FromStr;

use crate::moon::MoonID;
use crate::state::State;

pub type KingdomID = usize;

pub struct Kingdom {
    name: String,
    moons_to_leave: u16,
    moons_to_unlock: u16,
    prerequisite_kingdoms: Vec<KingdomID>,
    next_kingdoms: Vec<KingdomID>,
}

impl Kingdom {
    fn new(name: &str, moons_to_leave: u16,
               moons_to_unlock: u16) -> Self {
        Kingdom {
            name: String::from_str(name).unwrap(),
            moons_to_leave,
            moons_to_unlock,
            prerequisite_kingdoms: Vec::new(),
            next_kingdoms: Vec::new(),
        }
    }

    fn add_prerequisite(&mut self, id: KingdomID) {
        self.prerequisite_kingdoms.push(id);
    }

    fn link_next(&mut self, id: KingdomID) {
        self.next_kingdoms.push(id);
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn next(&self) -> &Vec<KingdomID> {
        &self.next_kingdoms
    }

    pub fn can_leave(&self, state: &State) -> bool {
        // available if kingdom moons is enough
        if state.total_kingdom_moons() < self.moons_to_leave {
            return false;
        }
        true
    }

    pub fn available(&self, state: &State) -> bool {
        // available if all prerequisites are scheduled
        for p in &self.prerequisite_kingdoms {
            if !state.kingdom_scheduled(*p) {
                return false;
            }
        }

        // available if total moons is enough
        if state.total_moons() < self.moons_to_unlock {
            return false;
        }

        true
    }
}

pub struct Kingdoms {
    kingdoms: Vec<Kingdom>,
}

impl Kingdoms {
    pub fn kingdom(&self, id: KingdomID) -> &Kingdom {
        &self.kingdoms[id]
    }

    pub fn new() -> Self {
        let mut kingdoms = Vec::new();

        // main game
        let cap = kingdoms.len();
        kingdoms.push(Kingdom::new("Cap Kingdom", 0, 0));
        let cascade = kingdoms.len();
        kingdoms.push(Kingdom::new("Cascade Kingdom", 5, 0));
        let sand = kingdoms.len();
        kingdoms.push(Kingdom::new("Sand Kingdom", 16, 0));
        let lake = kingdoms.len();
        kingdoms.push(Kingdom::new("Lake Kingdom", 8, 0));
        let wooded = kingdoms.len();
        kingdoms.push(Kingdom::new("Wooded Kingdom", 16, 0));
        let cloud = kingdoms.len();
        kingdoms.push(Kingdom::new("Cloud Kingdom", 0, 0));
        let lost = kingdoms.len();
        kingdoms.push(Kingdom::new("Lost Kingdom", 10, 0));
        let metro = kingdoms.len();
        kingdoms.push(Kingdom::new("Metro Kingdom", 20, 0));
        let snow = kingdoms.len();
        kingdoms.push(Kingdom::new("Snow Kingdom", 10, 0));
        let seaside = kingdoms.len();
        kingdoms.push(Kingdom::new("Seaside Kingdom", 10, 0));
        let luncheon = kingdoms.len();
        kingdoms.push(Kingdom::new("Luncheon Kingdom", 18, 0));
        let ruined = kingdoms.len();
        kingdoms.push(Kingdom::new("Ruined Kingdom", 3, 0));
        let bowser = kingdoms.len();
        kingdoms.push(Kingdom::new("Bowser's Kingdom", 8, 0));
        let moon = kingdoms.len();
        kingdoms.push(Kingdom::new("Moon Kingdom", 0, 0));
        let mushroom = kingdoms.len();
        kingdoms.push(Kingdom::new("Mushroom Kingdom", 1, 0));
        let dark = kingdoms.len();
        kingdoms.push(Kingdom::new("Dark Side", 4, 250));
        let darker = kingdoms.len();
        kingdoms.push(Kingdom::new("Darker Side", 3, 500));

        // link up main game
        kingdoms[cap].link_next(cascade);
        kingdoms[cascade].link_next(sand);
        kingdoms[cascade].add_prerequisite(cap);
        kingdoms[sand].link_next(lake);
        kingdoms[sand].link_next(wooded);
        kingdoms[sand].add_prerequisite(cascade);
        kingdoms[lake].link_next(cloud);
        kingdoms[lake].add_prerequisite(sand);
        kingdoms[wooded].link_next(cloud);
        kingdoms[wooded].add_prerequisite(sand);
        kingdoms[cloud].link_next(lost);
        kingdoms[cloud].add_prerequisite(lake);
        kingdoms[cloud].add_prerequisite(wooded);
        kingdoms[lost].link_next(metro);
        kingdoms[lost].add_prerequisite(cloud);
        kingdoms[metro].link_next(snow);
        kingdoms[metro].link_next(seaside);
        kingdoms[metro].add_prerequisite(lost);
        kingdoms[snow].link_next(luncheon);
        kingdoms[snow].add_prerequisite(metro);
        kingdoms[seaside].link_next(luncheon);
        kingdoms[seaside].add_prerequisite(metro);
        kingdoms[luncheon].link_next(ruined);
        kingdoms[luncheon].add_prerequisite(snow);
        kingdoms[luncheon].add_prerequisite(seaside);
        kingdoms[ruined].link_next(bowser);
        kingdoms[ruined].add_prerequisite(metro);
        kingdoms[bowser].link_next(moon);
        kingdoms[bowser].add_prerequisite(ruined);
        kingdoms[moon].link_next(mushroom);
        kingdoms[moon].add_prerequisite(bowser);
        kingdoms[mushroom].add_prerequisite(moon);

        // post game
        let cap2 = kingdoms.len();
        kingdoms.push(Kingdom::new("Cap Kingdom", 1, 0));
        let cascade2 = kingdoms.len();
        kingdoms.push(Kingdom::new("Cascade Kingdom", 1, 0));
        let sand2 = kingdoms.len();
        kingdoms.push(Kingdom::new("Sand Kingdom", 1, 0));
        let lake2 = kingdoms.len();
        kingdoms.push(Kingdom::new("Lake Kingdom", 1, 0));
        let wooded2 = kingdoms.len();
        kingdoms.push(Kingdom::new("Wooded Kingdom", 1, 0));
        let cloud2 = kingdoms.len();
        kingdoms.push(Kingdom::new("Cloud Kingdom", 1, 0));
        let lost2 = kingdoms.len();
        kingdoms.push(Kingdom::new("Lost Kingdom", 1, 0));
        let metro2 = kingdoms.len();
        kingdoms.push(Kingdom::new("Metro Kingdom", 1, 0));
        let snow2 = kingdoms.len();
        kingdoms.push(Kingdom::new("Snow Kingdom", 1, 0));
        let seaside2 = kingdoms.len();
        kingdoms.push(Kingdom::new("Seaside Kingdom", 1, 0));
        let luncheon2 = kingdoms.len();
        kingdoms.push(Kingdom::new("Luncheon Kingdom", 1, 0));
        let ruined2 = kingdoms.len();
        kingdoms.push(Kingdom::new("Ruined Kingdom", 1, 0));
        let bowser2 = kingdoms.len();
        kingdoms.push(Kingdom::new("Bowser's Kingdom", 1, 0));
        let moon2 = kingdoms.len();
        kingdoms.push(Kingdom::new("Moon Kingdom", 1, 0));

        // link up post game - everyone can go to everyone
        kingdoms[mushroom].link_next(dark);
        kingdoms[mushroom].link_next(darker);
        kingdoms[mushroom].link_next(cap2);
        kingdoms[mushroom].link_next(cascade2);
        kingdoms[mushroom].link_next(sand2);
        kingdoms[mushroom].link_next(lake2);
        kingdoms[mushroom].link_next(wooded2);
        kingdoms[mushroom].link_next(cloud2);
        kingdoms[mushroom].link_next(lost2);
        kingdoms[mushroom].link_next(metro2);
        kingdoms[mushroom].link_next(snow2);
        kingdoms[mushroom].link_next(seaside2);
        kingdoms[mushroom].link_next(luncheon2);
        kingdoms[mushroom].link_next(ruined2);
        kingdoms[mushroom].link_next(bowser2);
        kingdoms[mushroom].link_next(moon2);

        kingdoms[dark].link_next(mushroom);
        kingdoms[dark].link_next(darker);
        kingdoms[dark].link_next(cap2);
        kingdoms[dark].link_next(cascade2);
        kingdoms[dark].link_next(sand2);
        kingdoms[dark].link_next(lake2);
        kingdoms[dark].link_next(wooded2);
        kingdoms[dark].link_next(cloud2);
        kingdoms[dark].link_next(lost2);
        kingdoms[dark].link_next(metro2);
        kingdoms[dark].link_next(snow2);
        kingdoms[dark].link_next(seaside2);
        kingdoms[dark].link_next(luncheon2);
        kingdoms[dark].link_next(ruined2);
        kingdoms[dark].link_next(bowser2);

        kingdoms[darker].link_next(mushroom);
        kingdoms[darker].link_next(dark);
        kingdoms[darker].link_next(cap2);
        kingdoms[darker].link_next(cascade2);
        kingdoms[darker].link_next(sand2);
        kingdoms[darker].link_next(lake2);
        kingdoms[darker].link_next(wooded2);
        kingdoms[darker].link_next(cloud2);
        kingdoms[darker].link_next(lost2);
        kingdoms[darker].link_next(metro2);
        kingdoms[darker].link_next(snow2);
        kingdoms[darker].link_next(seaside2);
        kingdoms[darker].link_next(luncheon2);
        kingdoms[darker].link_next(ruined2);
        kingdoms[darker].link_next(bowser2);

        kingdoms[cap2].link_next(mushroom);
        kingdoms[cap2].link_next(dark);
        kingdoms[cap2].link_next(darker);
        kingdoms[cap2].link_next(cascade2);
        kingdoms[cap2].link_next(sand2);
        kingdoms[cap2].link_next(lake2);
        kingdoms[cap2].link_next(wooded2);
        kingdoms[cap2].link_next(cloud2);
        kingdoms[cap2].link_next(lost2);
        kingdoms[cap2].link_next(metro2);
        kingdoms[cap2].link_next(snow2);
        kingdoms[cap2].link_next(seaside2);
        kingdoms[cap2].link_next(luncheon2);
        kingdoms[cap2].link_next(ruined2);
        kingdoms[cap2].link_next(bowser2);
        kingdoms[cap2].link_next(moon2);

        kingdoms[cascade2].link_next(mushroom);
        kingdoms[cascade2].link_next(dark);
        kingdoms[cascade2].link_next(darker);
        kingdoms[cascade2].link_next(cap2);
        kingdoms[cascade2].link_next(sand2);
        kingdoms[cascade2].link_next(lake2);
        kingdoms[cascade2].link_next(wooded2);
        kingdoms[cascade2].link_next(cloud2);
        kingdoms[cascade2].link_next(lost2);
        kingdoms[cascade2].link_next(metro2);
        kingdoms[cascade2].link_next(snow2);
        kingdoms[cascade2].link_next(seaside2);
        kingdoms[cascade2].link_next(luncheon2);
        kingdoms[cascade2].link_next(ruined2);
        kingdoms[cascade2].link_next(bowser2);
        kingdoms[cascade2].link_next(moon2);

        kingdoms[sand2].link_next(mushroom);
        kingdoms[sand2].link_next(dark);
        kingdoms[sand2].link_next(darker);
        kingdoms[sand2].link_next(cap2);
        kingdoms[sand2].link_next(cascade2);
        kingdoms[sand2].link_next(lake2);
        kingdoms[sand2].link_next(wooded2);
        kingdoms[sand2].link_next(cloud2);
        kingdoms[sand2].link_next(lost2);
        kingdoms[sand2].link_next(metro2);
        kingdoms[sand2].link_next(snow2);
        kingdoms[sand2].link_next(seaside2);
        kingdoms[sand2].link_next(luncheon2);
        kingdoms[sand2].link_next(ruined2);
        kingdoms[sand2].link_next(bowser2);
        kingdoms[sand2].link_next(moon2);

        kingdoms[sand2].link_next(mushroom);
        kingdoms[sand2].link_next(dark);
        kingdoms[sand2].link_next(darker);
        kingdoms[sand2].link_next(cap2);
        kingdoms[sand2].link_next(cascade2);
        kingdoms[sand2].link_next(lake2);
        kingdoms[sand2].link_next(wooded2);
        kingdoms[sand2].link_next(cloud2);
        kingdoms[sand2].link_next(lost2);
        kingdoms[sand2].link_next(metro2);
        kingdoms[sand2].link_next(snow2);
        kingdoms[sand2].link_next(seaside2);
        kingdoms[sand2].link_next(luncheon2);
        kingdoms[sand2].link_next(ruined2);
        kingdoms[sand2].link_next(bowser2);
        kingdoms[sand2].link_next(moon2);

        kingdoms[lake2].link_next(mushroom);
        kingdoms[lake2].link_next(dark);
        kingdoms[lake2].link_next(darker);
        kingdoms[lake2].link_next(cap2);
        kingdoms[lake2].link_next(cascade2);
        kingdoms[lake2].link_next(sand2);
        kingdoms[lake2].link_next(wooded2);
        kingdoms[lake2].link_next(cloud2);
        kingdoms[lake2].link_next(lost2);
        kingdoms[lake2].link_next(metro2);
        kingdoms[lake2].link_next(snow2);
        kingdoms[lake2].link_next(seaside2);
        kingdoms[lake2].link_next(luncheon2);
        kingdoms[lake2].link_next(ruined2);
        kingdoms[lake2].link_next(bowser2);
        kingdoms[lake2].link_next(moon2);

        kingdoms[wooded2].link_next(mushroom);
        kingdoms[wooded2].link_next(dark);
        kingdoms[wooded2].link_next(darker);
        kingdoms[wooded2].link_next(cap2);
        kingdoms[wooded2].link_next(cascade2);
        kingdoms[wooded2].link_next(sand2);
        kingdoms[wooded2].link_next(lake2);
        kingdoms[wooded2].link_next(cloud2);
        kingdoms[wooded2].link_next(lost2);
        kingdoms[wooded2].link_next(metro2);
        kingdoms[wooded2].link_next(snow2);
        kingdoms[wooded2].link_next(seaside2);
        kingdoms[wooded2].link_next(luncheon2);
        kingdoms[wooded2].link_next(ruined2);
        kingdoms[wooded2].link_next(bowser2);
        kingdoms[wooded2].link_next(moon2);

        kingdoms[cloud2].link_next(mushroom);
        kingdoms[cloud2].link_next(dark);
        kingdoms[cloud2].link_next(darker);
        kingdoms[cloud2].link_next(cap2);
        kingdoms[cloud2].link_next(cascade2);
        kingdoms[cloud2].link_next(sand2);
        kingdoms[cloud2].link_next(lake2);
        kingdoms[cloud2].link_next(wooded2);
        kingdoms[cloud2].link_next(lost2);
        kingdoms[cloud2].link_next(metro2);
        kingdoms[cloud2].link_next(snow2);
        kingdoms[cloud2].link_next(seaside2);
        kingdoms[cloud2].link_next(luncheon2);
        kingdoms[cloud2].link_next(ruined2);
        kingdoms[cloud2].link_next(bowser2);
        kingdoms[cloud2].link_next(moon2);

        kingdoms[lost2].link_next(mushroom);
        kingdoms[lost2].link_next(dark);
        kingdoms[lost2].link_next(darker);
        kingdoms[lost2].link_next(cap2);
        kingdoms[lost2].link_next(cascade2);
        kingdoms[lost2].link_next(sand2);
        kingdoms[lost2].link_next(lake2);
        kingdoms[lost2].link_next(wooded2);
        kingdoms[lost2].link_next(cloud2);
        kingdoms[lost2].link_next(metro2);
        kingdoms[lost2].link_next(snow2);
        kingdoms[lost2].link_next(seaside2);
        kingdoms[lost2].link_next(luncheon2);
        kingdoms[lost2].link_next(ruined2);
        kingdoms[lost2].link_next(bowser2);
        kingdoms[lost2].link_next(moon2);

        kingdoms[metro2].link_next(mushroom);
        kingdoms[metro2].link_next(dark);
        kingdoms[metro2].link_next(darker);
        kingdoms[metro2].link_next(cap2);
        kingdoms[metro2].link_next(cascade2);
        kingdoms[metro2].link_next(sand2);
        kingdoms[metro2].link_next(lake2);
        kingdoms[metro2].link_next(wooded2);
        kingdoms[metro2].link_next(cloud2);
        kingdoms[metro2].link_next(lost2);
        kingdoms[metro2].link_next(snow2);
        kingdoms[metro2].link_next(seaside2);
        kingdoms[metro2].link_next(luncheon2);
        kingdoms[metro2].link_next(ruined2);
        kingdoms[metro2].link_next(bowser2);
        kingdoms[metro2].link_next(moon2);

        kingdoms[snow2].link_next(mushroom);
        kingdoms[snow2].link_next(dark);
        kingdoms[snow2].link_next(darker);
        kingdoms[snow2].link_next(cap2);
        kingdoms[snow2].link_next(cascade2);
        kingdoms[snow2].link_next(sand2);
        kingdoms[snow2].link_next(lake2);
        kingdoms[snow2].link_next(wooded2);
        kingdoms[snow2].link_next(cloud2);
        kingdoms[snow2].link_next(lost2);
        kingdoms[snow2].link_next(metro2);
        kingdoms[snow2].link_next(seaside2);
        kingdoms[snow2].link_next(luncheon2);
        kingdoms[snow2].link_next(ruined2);
        kingdoms[snow2].link_next(bowser2);
        kingdoms[snow2].link_next(moon2);

        kingdoms[seaside2].link_next(mushroom);
        kingdoms[seaside2].link_next(dark);
        kingdoms[seaside2].link_next(darker);
        kingdoms[seaside2].link_next(cap2);
        kingdoms[seaside2].link_next(cascade2);
        kingdoms[seaside2].link_next(sand2);
        kingdoms[seaside2].link_next(lake2);
        kingdoms[seaside2].link_next(wooded2);
        kingdoms[seaside2].link_next(cloud2);
        kingdoms[seaside2].link_next(lost2);
        kingdoms[seaside2].link_next(metro2);
        kingdoms[seaside2].link_next(snow2);
        kingdoms[seaside2].link_next(luncheon2);
        kingdoms[seaside2].link_next(ruined2);
        kingdoms[seaside2].link_next(bowser2);
        kingdoms[seaside2].link_next(moon2);

        kingdoms[luncheon2].link_next(mushroom);
        kingdoms[luncheon2].link_next(dark);
        kingdoms[luncheon2].link_next(darker);
        kingdoms[luncheon2].link_next(cap2);
        kingdoms[luncheon2].link_next(cascade2);
        kingdoms[luncheon2].link_next(sand2);
        kingdoms[luncheon2].link_next(lake2);
        kingdoms[luncheon2].link_next(wooded2);
        kingdoms[luncheon2].link_next(cloud2);
        kingdoms[luncheon2].link_next(lost2);
        kingdoms[luncheon2].link_next(metro2);
        kingdoms[luncheon2].link_next(snow2);
        kingdoms[luncheon2].link_next(seaside2);
        kingdoms[luncheon2].link_next(ruined2);
        kingdoms[luncheon2].link_next(bowser2);
        kingdoms[luncheon2].link_next(moon2);

        kingdoms[ruined2].link_next(mushroom);
        kingdoms[ruined2].link_next(dark);
        kingdoms[ruined2].link_next(darker);
        kingdoms[ruined2].link_next(cap2);
        kingdoms[ruined2].link_next(cascade2);
        kingdoms[ruined2].link_next(sand2);
        kingdoms[ruined2].link_next(lake2);
        kingdoms[ruined2].link_next(wooded2);
        kingdoms[ruined2].link_next(cloud2);
        kingdoms[ruined2].link_next(lost2);
        kingdoms[ruined2].link_next(metro2);
        kingdoms[ruined2].link_next(snow2);
        kingdoms[ruined2].link_next(seaside2);
        kingdoms[ruined2].link_next(luncheon2);
        kingdoms[ruined2].link_next(bowser2);
        kingdoms[ruined2].link_next(moon2);

        kingdoms[bowser2].link_next(mushroom);
        kingdoms[bowser2].link_next(dark);
        kingdoms[bowser2].link_next(darker);
        kingdoms[bowser2].link_next(cap2);
        kingdoms[bowser2].link_next(cascade2);
        kingdoms[bowser2].link_next(sand2);
        kingdoms[bowser2].link_next(lake2);
        kingdoms[bowser2].link_next(wooded2);
        kingdoms[bowser2].link_next(cloud2);
        kingdoms[bowser2].link_next(lost2);
        kingdoms[bowser2].link_next(metro2);
        kingdoms[bowser2].link_next(snow2);
        kingdoms[bowser2].link_next(seaside2);
        kingdoms[bowser2].link_next(luncheon2);
        kingdoms[bowser2].link_next(ruined2);
        kingdoms[bowser2].link_next(moon2);

        kingdoms[moon2].link_next(mushroom);
        kingdoms[moon2].link_next(dark);
        kingdoms[moon2].link_next(darker);
        kingdoms[moon2].link_next(cap2);
        kingdoms[moon2].link_next(cascade2);
        kingdoms[moon2].link_next(sand2);
        kingdoms[moon2].link_next(lake2);
        kingdoms[moon2].link_next(wooded2);
        kingdoms[moon2].link_next(cloud2);
        kingdoms[moon2].link_next(lost2);
        kingdoms[moon2].link_next(metro2);
        kingdoms[moon2].link_next(snow2);
        kingdoms[moon2].link_next(seaside2);
        kingdoms[moon2].link_next(luncheon2);
        kingdoms[moon2].link_next(ruined2);
        kingdoms[moon2].link_next(bowser2);

        Kingdoms {
            kingdoms
        }
    }
}
