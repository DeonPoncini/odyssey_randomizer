use rand::{thread_rng, Rng};

use crate::state::State;
use crate::kingdom::{KingdomName, Kingdoms};
use crate::moon::Moons;

mod kingdom;
mod moon;
mod state;

fn main() {
    let mut state = State::new();
    let mut kingdoms = Kingdoms::new();
    let mut moons = Moons::new(&mut kingdoms);
    let mut leave_chance = 1;

    // start up the first kingdom
    state.add_kingdom_to_schedule(KingdomName::Cap);
    state.schedule_kingdom();

    loop {
        // first, find all moons that can be scheduled
        let available = moons.return_available(&mut state);
        for a in &available {
            state.add_moon_to_schedule(*a);
        }
        // schedule a random count trying to be enough to leave
        let exit_count;
        if state.completed_main_game() {
            exit_count = 1;
        } else {
            exit_count = kingdoms.kingdom(state.current_kingdom())
                .moons_to_leave();
        }
        let scheduleable = state.moons_to_schedule();
        let exit_count = std::cmp::min(exit_count as usize, scheduleable);
        let scheduled;
        if exit_count == scheduleable {
            scheduled = exit_count;
        } else {
            scheduled = thread_rng().gen_range(exit_count, scheduleable);
        }
        if scheduled == 0 {
            state.next_kingdom(&kingdoms);
            // schedule the next kingdom
            if !state.schedule_kingdom() {
                // no more moons and no more kingdoms, we are done
                break;
            }
        } else {
            // schedule the moons
            for _ in 0..scheduled {
               state.schedule_moon(&moons);
            }
            // lets only leave with a 10% chance that increases 10% each time
            let chance = thread_rng().gen_range(0, 10);
            if chance < leave_chance {
                leave_chance = 1;
                // leave for the next kingdom
                if state.next_kingdom(&kingdoms) {
                    state.schedule_kingdom();
                }
            } else {
                leave_chance += 1;
            }
        }
    }

    // print out the moons
    state.print_moons(&kingdoms, &moons);
}

