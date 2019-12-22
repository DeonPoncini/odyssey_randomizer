---
Title: Odyssey Randomizer
Description: All Moons Randomizer for Super Mario Odyssey
Author: Deon Poncini

---
Odyssey Randomizer
==================

Description
-----------
This creates a randomized list of moons in the Super Mario Odyssey game to allow
you to complete the game with all moons in a random order.

Build and Execute
-----------------
Odyssey Randomizer is written in Rust, so just install Rust (I recommend doing
this through https://rustup.rs), then to build and run just type

    cargo run

This will create the random list. Currently there are no options to configure.

Current Moon Routing Assumptions
--------------------------------
This randomizer makes the following assumptions:

* Before leaving each kingdom, achieve world peace
* There is no use of sequence breaks (IP Clip, Snow Dram, Lake Clip etc)
* Hint art moons require a visit to the kingdom with the art before hand
* Secret path moons just involve a visit to the kingdom that starts the secret
path, however its not the best as it doesn't require the visit to precede the
kingdom with the secret path moon, so you might have to travel
* In post game, you only need to collect one moon before potentially leaving
* Mushroom Achievement moons are collected in numerical order, although not
strictly necessary I wanted to sort them mostly to the end
* In the first playthrough of the kingdoms, there is no backtracking to previous
kingdoms, just simply visit each in sequence (with divergent routes randomly
chosen) until reaching Mushroom. In post game, travel from one kingdom to the
next is completely randomized
* Sub area moons are not generally tied together, so repeated visits to sub areas
are required

I Want to Re-route the Moons
----------------------------
Maybe one day I will do what should be done and have all the kingdom and moon
dependencies in a configuration file, but for now they are hardcoded in the
following places:

For Kingdom dependencies:

    src/kingdom.rs:148 (Kingdoms::new())

For Moon dependencies:

    src/moon.rs:139 (Moons::new())

I will describe how the routing works in the next sections and you can just alter
the code to make it route like you want removing or adding dependencies.

How does Kingdom Routing work?
------------------------------
Each kingdom has two functions that control routing, however this is only active
in pre-game (before the first visit to Mushroom). In post game, routing is
ignored and kingdom selection is random

    fn add_prerequisite(&mut self, id: KingdomName);
    fn link_next(&mut self, id: KingdomName);

The `add_prerequisite` function adds any kingdoms that must be visited before
this kingdom can be scheduled for routing.

The `link_next` function adds any kingdoms that can be visited next after this
current kingdom, and is used to select the next kingdoms that can be traveled to.
If you want to for example allow backtracking after a first kingdom visit, you
can alter this to add all previous kingdoms as potential next destinations.

When deciding whether to leave a kingdom, two functions are consulted

    pub fn can_leave(&self, state: &State) -> bool;
    pub fn available(&self, state: &State) -> bool;

The available function checks that the Kingdom is currently available to be
scheduled, which it does through checking the prerequisites are scheduled and
enough moons have been scheduled to unlock this kingdom (used for Dark Side and
Darker Side).

The `can_leave` function figures out if the we can leave the kingdom and visit
the next one. This is determined to see if the moon required to exit has been
scheduled, and also if we have enough moons scheduled to leave. During pre-game,
this is the amount of moons required by the game to go to the next kingdom. During
post game, this is currently set to one.

The concept of an exit moon is really only required once in the game, the Mecha
Broodal fight in Bowser's Kingdom. All other Kingdoms can simply be left after
getting the right number of moons. However, for aestetic reasons I wanted to
achieve World Peace in pre-game, so I set the exit moon for each kingdom to be
the World Peace conclusion moon.

How does Moon Routing work?
---------------------------
The moons in each kingdom are organized as a dependency graph. The moons are
scheduled according to these dependencies by getting a list of available moons
in each kingdom, scheduling them, and repeating until the Kingdom is exited.
The dependencies are controlled with the following functions:

    fn add_prereq_kingdom(&mut self, kingdom: KingdomName);
    fn add_prereq_kingdom_count(&mut self, kingdom: KingdomName, visited: u8);
    fn add_prereq_moon(&mut self, moon: MoonID);
    fn set_prereq_moon_count(&mut self, count: u16);

The `add_prereq_kingdom` function allows to put a dependency on visiting a
certain Kingdom before this moon is able to be scheduled. This is useful for
things like painting moons, as well as postgame moons that only unlock after
Mushroom is visited.

The `add_prereq_kingdom_count` function determines how many times a previous
kingdom needs to be visited before this moon will be available to schedule. This
is useful for things like hint art where the hint art picture is only available
post game, so requires two visits to the kingdom with the picture before the
moon is available.

The `add_prereq_moon` function lists a moon is a direct dependency. Many moons
require a previous moon to be received first before unlocking. If you want to
make it so sub area moons are tied together so you only visit a sub area once,
then you can make one a pre-req of another, for example.

The `set_prereq_moon_count` function sets how many moons must be scheduled before
this moon is available. Currently just used for the Mushroom achievement moons
for 100, 300 and 600 moons.

To determine what moons are available, the state machine executes the following

    pub fn return_available(&mut self, state: &mut State) -> Vec<MoonID>;

Which determines what moons are available by checking if each moon is available
based on the criteria set by the above functions. These are then returned, and
a subset of them are randomly sorted and scheduled. This continues until the
kingdom is exited, and a new batch of available moons are dispatched.
