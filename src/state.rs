use std::collections::{HashSet, HashMap};

use rand::{thread_rng, Rng};

use crate::kingdom::{Kingdoms, KingdomName};
use crate::moon::MoonID;

pub struct State {
    current_kingdom: KingdomName,
    total_kingdom_moons: u16,
    total_moons: u16,
    moons_to_schedule: Vec<MoonID>,
    moons_ordered: Vec<MoonID>,
    moons_scheduled: HashSet<MoonID>,
    kingdoms_to_schedule: Vec<KingdomName>,
    kingdoms_ordered: Vec<KingdomName>,
    kingdoms_scheduled: HashMap<KingdomName, u8>,
}

impl State {
    pub fn new() -> Self {
        State {
            current_kingdom: KingdomName::Cap,
            total_kingdom_moons: 0,
            total_moons: 600,
            moons_to_schedule: Vec::new(),
            moons_ordered: Vec::new(),
            moons_scheduled: HashSet::new(),
            kingdoms_to_schedule: Vec::new(),
            kingdoms_ordered: Vec::new(),
            kingdoms_scheduled: HashMap::new(),
        }
    }

    pub fn print_schedule(&self, kingdoms: &Kingdoms) {
        for k in &self.kingdoms_ordered {
            println!("{}", kingdoms.kingdom(*k).name());
        }
    }

    pub fn add_kingdom_to_schedule(&mut self, id: KingdomName) {
        self.kingdoms_to_schedule.push(id);
    }

    pub fn schedule_kingdom(&mut self) -> bool {
        // if there are no kingdoms to schedule, return false
        if self.kingdoms_to_schedule.is_empty() {
            return false;
        }
        // randomly pick an available kingdom and schedule it
        let random = thread_rng().gen_range(0, self.kingdoms_to_schedule.len());
        // remove it from the scheduled
        let id = self.kingdoms_to_schedule.remove(random);
        // schedule it
        self.kingdoms_ordered.push(id);
        // update how many times we scheduled this
        match self.kingdoms_scheduled.get_mut(&id) {
            Some(v) => *v += 1,
            None => { self.kingdoms_scheduled.insert(id, 1); }
        }
        true
    }

    pub fn current_kingdom(&self) -> KingdomName {
        self.current_kingdom
    }

    pub fn total_kingdom_moons(&self) -> u16 {
        self.total_kingdom_moons
    }

    pub fn total_moons(&self) -> u16 {
        self.total_moons
    }

    pub fn moon_scheduled(&self, moon: MoonID) -> bool {
        self.moons_scheduled.contains(&moon)
    }

    pub fn kingdom_scheduled(&self, kingdom: KingdomName, visited: u8) -> bool {
        match self.kingdoms_scheduled.get(&kingdom) {
            Some(v) => *v >= visited,
            None => false,
        }
    }
}
