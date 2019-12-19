use std::collections::{HashSet, HashMap};

use rand::{thread_rng, Rng};

use crate::kingdom::{Kingdoms, KingdomName};
use crate::moon::{Moons, MoonID};

pub struct State {
    current_kingdom: KingdomName,
    total_kingdom_moons: u16,
    total_moons: u16,
    moons_to_schedule: Vec<MoonID>,
    moons_stored_queue: HashMap<KingdomName, Vec<MoonID>>,
    moons_ordered: Vec<MoonID>,
    moons_scheduled: HashSet<MoonID>,
    kingdoms_to_schedule: Vec<KingdomName>,
    kingdoms_ordered: Vec<KingdomName>,
    kingdoms_scheduled: HashMap<KingdomName, u8>,
    kingdoms_completed: HashSet<KingdomName>,
    completed_main_game: bool,
}

impl State {
    pub fn new() -> Self {
        State {
            current_kingdom: KingdomName::Darker,
            total_kingdom_moons: 0,
            total_moons: 0,
            moons_to_schedule: Vec::new(),
            moons_stored_queue: HashMap::new(),
            moons_ordered: Vec::new(),
            moons_scheduled: HashSet::new(),
            kingdoms_to_schedule: Vec::new(),
            kingdoms_ordered: Vec::new(),
            kingdoms_scheduled: HashMap::new(),
            kingdoms_completed: HashSet::new(),
            completed_main_game: false,
        }
    }

    pub fn print_moons(&self, kingdoms: &Kingdoms, moons: &Moons) {
        let mut x = 1;
        let mut current_kingdom = KingdomName::Cap;
        for m in &self.moons_ordered {
            if moons.moon(*m).kingdom() != current_kingdom {
                println!("==={}===",
                     kingdoms.kingdom(moons.moon(*m).kingdom()).name());
                current_kingdom = moons.moon(*m).kingdom();
            }
            let count = moons.moon(*m).count();
            if count > 1 {
                println!("{}.\t{} ({})", x, moons.moon(*m).name(), count);
            } else {
                println!("{}.\t{}", x, moons.moon(*m).name());
            }
            x += count;
        }
    }

    pub fn add_kingdom_to_schedule(&mut self, id: KingdomName) {
        if id == self.current_kingdom {
            return; // don't reschedule yourself
        }
        if self.kingdoms_completed.contains(&id) {
            // check if there are any pending moons to schedule
            match self.moons_stored_queue.get(&id) {
                Some(v) => {
                    if v.is_empty() {
                        return; // no stored moons
                    }
                    // otherwise, reschedule the kingdom
                }
                None => return,
            }
        }
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
        // if we have moons in the queue, back them up
        match self.moons_stored_queue.get_mut(&self.current_kingdom) {
            Some(v) => {
                v.extend(&self.moons_to_schedule);
                self.moons_to_schedule.clear();
            }
            None => {
                self.moons_stored_queue.insert(self.current_kingdom,
                                               self.moons_to_schedule.clone());
                self.moons_to_schedule.clear();
            }
        }

        // set the current schedule
        self.current_kingdom = id;
        self.total_kingdom_moons = 0;
        // check if we beat the game
        if self.current_kingdom == KingdomName::Mushroom {
            self.completed_main_game = true;
        }
        // if we have backed up moons, move them to the queue
        match self.moons_stored_queue.get_mut(&self.current_kingdom) {
            Some(v) => {
                for vv in v {
                    self.moons_to_schedule.push(*vv);
                }
            }
            None => {}
        }
        self.moons_stored_queue.insert(self.current_kingdom, Vec::new());
        true
    }

    pub fn complete_kingdom(&mut self, id: KingdomName) {
        // before we complete a kingdom
        self.kingdoms_completed.insert(id);
    }

    pub fn next_kingdom(&mut self, kingdoms: &Kingdoms) {
        // move to the next kingdom
        if self.completed_main_game {
            if self.total_kingdom_moons() < 1 {
                // we can't leave yet
                return;
            }
            // add every kingdom that isn't this one
            self.add_kingdom_to_schedule(KingdomName::Cap);
            self.add_kingdom_to_schedule(KingdomName::Cascade);
            self.add_kingdom_to_schedule(KingdomName::Sand);
            self.add_kingdom_to_schedule(KingdomName::Lake);
            self.add_kingdom_to_schedule(KingdomName::Wooded);
            self.add_kingdom_to_schedule(KingdomName::Cloud);
            self.add_kingdom_to_schedule(KingdomName::Lost);
            self.add_kingdom_to_schedule(KingdomName::Metro);
            self.add_kingdom_to_schedule(KingdomName::Snow);
            self.add_kingdom_to_schedule(KingdomName::Seaside);
            self.add_kingdom_to_schedule(KingdomName::Luncheon);
            self.add_kingdom_to_schedule(KingdomName::Ruined);
            self.add_kingdom_to_schedule(KingdomName::Bowser);
            self.add_kingdom_to_schedule(KingdomName::Moon);
            self.add_kingdom_to_schedule(KingdomName::Mushroom);
            if kingdoms.kingdom(KingdomName::Dark).available(&self) {
                self.add_kingdom_to_schedule(KingdomName::Dark);
            }
            if kingdoms.kingdom(KingdomName::Darker).available(&self) {
                self.add_kingdom_to_schedule(KingdomName::Darker);
            }
        } else {
            if !kingdoms.kingdom(self.current_kingdom).can_leave(&self) {
                // can't leave yet
                return;
            }
            for k in kingdoms.kingdom(self.current_kingdom).next() {
                if kingdoms.kingdom(*k).available(&self) {
                    self.add_kingdom_to_schedule(*k);
                }
            }
        }
    }

    pub fn add_moon_to_schedule(&mut self, id: MoonID) {
        self.moons_to_schedule.push(id);
    }

    pub fn moons_to_schedule(&self) -> usize {
        self.moons_to_schedule.len()
    }

    pub fn schedule_moon(&mut self, moons: &Moons) -> bool {
        // if there are no moons to schedule, return false
        if self.moons_to_schedule.is_empty() {
            return false;
        }
        // randomly pick a moon and schedule it
        let random = thread_rng().gen_range(0, self.moons_to_schedule.len());
        let id = self.moons_to_schedule.remove(random);
        let count = moons.moon(id).count();
        // schedule it
        self.moons_ordered.push(id);
        self.moons_scheduled.insert(id);
        self.total_kingdom_moons += count;
        self.total_moons += count;
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

    pub fn completed_main_game(&self) -> bool {
        self.completed_main_game
    }

    pub fn kingdom_scheduled(&self, kingdom: KingdomName, visited: u8) -> bool {
        match self.kingdoms_scheduled.get(&kingdom) {
            Some(v) => *v >= visited,
            None => false,
        }
    }
}
