//! Labyrinth
//!
//! Look at `labyrinth_grade.rs` below before you start.
//! HINT: <https://en.wikipedia.org/wiki/100_prisoners_problem>
//!
//! NOTE: You will have to implement a probabilistic algorithm, which means, the algorithm can fail
//! even if you have implemented the solution. We recommend running multiple times (at least 5
//! times) to check your solution works well.

use std::cell::RefCell;

/// Husband
#[derive(Debug)]
pub struct Husband {
    brain: RefCell<[usize; 100]>,
}

impl Husband {
    /// What might a husband, who is looking for his wife's ID my_wife, be thinking?
    pub fn seeking(my_wife: usize) -> Self {
        let mut brain = [0; 100];
        brain[0] = my_wife;
        Self {
            brain: RefCell::new(brain),
        }
    }

    #[allow(missing_docs)]
    pub fn has_devised_a_strategy(&self) -> Strategy<'_> {
        Strategy {
            husband: self,
            current: 0,
            steps: 0,
        }
    }

    /// Based on the information about currently visited room number and someone's wife ID trapped
    /// inside, what the husband should do next?
    pub fn carefully_checks_whos_inside(&self, room: usize, wife: usize) {
        self.brain.borrow_mut()[room] = wife;
    }
}

/// Strategy of husband
#[derive(Debug)]
pub struct Strategy<'a> {
    husband: &'a Husband,
    current: usize,
    steps: usize,
}

impl Iterator for Strategy<'_> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.steps >= 50 {
            return None;
        }

        let next_room = self.current;
        let brain = self.husband.brain.borrow();
        self.current = brain[next_room];
        self.steps += 1;
        Some(next_room)
    }
}
