use crate::moon::MoonID;
use crate::state::State;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum KingdomName {
    Cap,
    Cascade,
    Sand,
    Lake,
    Wooded,
    Cloud,
    Lost,
    Metro,
    Snow,
    Seaside,
    Luncheon,
    Ruined,
    Bowser,
    Moon,
    Mushroom,
    Dark,
    Darker,
}

pub struct Kingdom {
    name: String,
    moons_to_leave: u16,
    moons_to_unlock: u16,
    prerequisite_kingdoms: Vec<KingdomName>,
    next_kingdoms: Vec<KingdomName>,
    exit_moon: Option<MoonID>,
}

impl Kingdom {
    fn new(name: &str, moons_to_leave: u16,
               moons_to_unlock: u16) -> Self {
        Kingdom {
            name: String::from(name),
            moons_to_leave,
            moons_to_unlock,
            prerequisite_kingdoms: Vec::new(),
            next_kingdoms: Vec::new(),
            exit_moon: None,
        }
    }

    fn add_prerequisite(&mut self, id: KingdomName) {
        self.prerequisite_kingdoms.push(id);
    }

    fn link_next(&mut self, id: KingdomName) {
        self.next_kingdoms.push(id);
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn next(&self) -> &Vec<KingdomName> {
        &self.next_kingdoms
    }

    pub fn moons_to_leave(&self) -> u16 {
        self.moons_to_leave
    }

    pub fn set_exit_moon(&mut self, moon: MoonID) {
        self.exit_moon = Some(moon);
    }

    pub fn can_leave(&self, state: &State) -> bool {
        // can leave if the total kingdom moons are enough
        if state.total_kingdom_moons() < self.moons_to_leave {
            return false;
        }
        // can leave if the required moon ID has been scheduled
        match self.exit_moon {
            Some(m) => {
                if !state.moon_scheduled(m) {
                    return false;
                }
            }
            None => {}
        }
        true
    }

    pub fn available(&self, state: &State) -> bool {
        // available if all prerequisites are scheduled
        for p in &self.prerequisite_kingdoms {
            if !state.kingdom_scheduled(*p, 1) {
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
    pub fn kingdom(&self, id: KingdomName) -> &Kingdom {
        &self.kingdoms[id as usize]
    }

    pub fn kingdom_mut(&mut self, id: KingdomName) -> &mut Kingdom {
        &mut self.kingdoms[id as usize]
    }

    pub fn new() -> Self {
        let mut kingdoms = Vec::new();

        // main game
        kingdoms.push(Kingdom::new("Cap Kingdom", 0, 0));
        kingdoms.push(Kingdom::new("Cascade Kingdom", 5, 0));
        kingdoms.push(Kingdom::new("Sand Kingdom", 16, 0));
        kingdoms.push(Kingdom::new("Lake Kingdom", 8, 0));
        kingdoms.push(Kingdom::new("Wooded Kingdom", 16, 0));
        kingdoms.push(Kingdom::new("Cloud Kingdom", 0, 0));
        kingdoms.push(Kingdom::new("Lost Kingdom", 10, 0));
        kingdoms.push(Kingdom::new("Metro Kingdom", 20, 0));
        kingdoms.push(Kingdom::new("Snow Kingdom", 10, 0));
        kingdoms.push(Kingdom::new("Seaside Kingdom", 10, 0));
        kingdoms.push(Kingdom::new("Luncheon Kingdom", 18, 0));
        kingdoms.push(Kingdom::new("Ruined Kingdom", 3, 0));
        kingdoms.push(Kingdom::new("Bowser's Kingdom", 8, 0));
        kingdoms.push(Kingdom::new("Moon Kingdom", 1, 0));
        kingdoms.push(Kingdom::new("Mushroom Kingdom", 1, 0));
        kingdoms.push(Kingdom::new("Dark Side", 4, 250));
        kingdoms.push(Kingdom::new("Darker Side", 3, 500));

        // link up main game
        kingdoms[KingdomName::Cap as usize].link_next(KingdomName::Cascade);
        kingdoms[KingdomName::Cascade as usize].link_next(KingdomName::Sand);
        kingdoms[KingdomName::Cascade as usize].add_prerequisite(KingdomName::Cap);
        kingdoms[KingdomName::Sand as usize].link_next(KingdomName::Lake);
        kingdoms[KingdomName::Sand as usize].link_next(KingdomName::Wooded);
        kingdoms[KingdomName::Sand as usize].add_prerequisite(KingdomName::Cascade);
        kingdoms[KingdomName::Lake as usize].link_next(KingdomName::Cloud);
        kingdoms[KingdomName::Lake as usize].add_prerequisite(KingdomName::Sand);
        kingdoms[KingdomName::Wooded as usize].link_next(KingdomName::Cloud);
        kingdoms[KingdomName::Wooded as usize].add_prerequisite(KingdomName::Sand);
        kingdoms[KingdomName::Cloud as usize].link_next(KingdomName::Lost);
        kingdoms[KingdomName::Cloud as usize].add_prerequisite(KingdomName::Lake);
        kingdoms[KingdomName::Cloud as usize].add_prerequisite(KingdomName::Wooded);
        kingdoms[KingdomName::Lost as usize].link_next(KingdomName::Metro);
        kingdoms[KingdomName::Lost as usize].add_prerequisite(KingdomName::Cloud);
        kingdoms[KingdomName::Metro as usize].link_next(KingdomName::Snow);
        kingdoms[KingdomName::Metro as usize].link_next(KingdomName::Seaside);
        kingdoms[KingdomName::Metro as usize].add_prerequisite(KingdomName::Lost);
        kingdoms[KingdomName::Snow as usize].link_next(KingdomName::Luncheon);
        kingdoms[KingdomName::Snow as usize].add_prerequisite(KingdomName::Metro);
        kingdoms[KingdomName::Seaside as usize].link_next(KingdomName::Luncheon);
        kingdoms[KingdomName::Seaside as usize].add_prerequisite(KingdomName::Metro);
        kingdoms[KingdomName::Luncheon as usize].link_next(KingdomName::Ruined);
        kingdoms[KingdomName::Luncheon as usize].add_prerequisite(KingdomName::Snow);
        kingdoms[KingdomName::Luncheon as usize].add_prerequisite(KingdomName::Seaside);
        kingdoms[KingdomName::Ruined as usize].link_next(KingdomName::Bowser);
        kingdoms[KingdomName::Ruined as usize].add_prerequisite(KingdomName::Metro);
        kingdoms[KingdomName::Bowser as usize].link_next(KingdomName::Moon);
        kingdoms[KingdomName::Bowser as usize].add_prerequisite(KingdomName::Ruined);
        kingdoms[KingdomName::Moon as usize].link_next(KingdomName::Mushroom);
        kingdoms[KingdomName::Moon as usize].add_prerequisite(KingdomName::Bowser);
        kingdoms[KingdomName::Mushroom as usize].add_prerequisite(KingdomName::Moon);

        Kingdoms {
            kingdoms
        }
    }
}
