use std::collections::{HashSet, VecDeque};

use crate::state::State;
use crate::kingdom::Kingdoms;

mod kingdom;
mod moon;
mod state;

fn main() {
    let mut state = State::new();
    let mut kingdoms = Kingdoms::new();

    // schedule all kingdoms
    let mut queue = VecDeque::new();
    let mut unscheduleable = HashSet::new();
    let mut scheduled = HashSet::new(); // TODO: this is temporary

    queue.push_back(0);
    while !queue.is_empty() {
        let id = queue.pop_front().unwrap();
        // if its available, add it to the kingdoms to be scheduled
        if kingdoms.kingdom(id).available(&state) {
            unscheduleable.remove(&id);
            if !scheduled.contains(&id) {
                scheduled.insert(id);
                state.add_kingdom_to_schedule(id);
            }
        } else {
            // if we tried to schedule this before and failed
            // we have returned to it again, so let's schedule a kingdom
            if unscheduleable.contains(&id) {
                if !state.schedule_kingdom() {
                    println!("This configuration cannot be scheduled");
                    return;
                }
            }
            // push it back to process again later
            queue.push_back(id);
            unscheduleable.insert(id);
        }
        // add all the next kingdoms to the list
        for k in kingdoms.kingdom(id).next() {
            if !scheduled.contains(k) {
                queue.push_back(*k);
            }
        }
    }

    // schedule everything that is unscheduled
    while state.schedule_kingdom() {}

    // print out the schedule
    state.print_schedule(&kingdoms);
}

