use std::collections::VecDeque;

/// Represents a single monkey.
#[derive(Clone)]
pub struct Monkey {
    items: VecDeque<u64>,
    op: Operation,
    divisor: u64,
    true_monkey: usize,
    false_monkey: usize,
    items_inspected: u64,
}

impl Monkey {
    /// Creates a new monkey, with items inspected set to 0.
    pub fn new(
        items: VecDeque<u64>,
        op: Operation,
        divisor: u64,
        true_monkey: usize,
        false_monkey: usize,
    ) -> Self {
        Self {
            items,
            op,
            divisor,
            true_monkey,
            false_monkey,
            items_inspected: 0,
        }
    }

    /// Adds the item to the end of the monkey's current items.
    pub fn give_item(&mut self, item: u64) {
        self.items.push_back(item);
    }

    /// Gets the divisor of the monkey used to check item worry levels.
    pub fn get_divisor(&self) -> u64 {
        self.divisor
    }

    /// Gets the number of items the monkey has inspected.
    pub fn get_items_inspected(&self) -> u64 {
        self.items_inspected
    }

    /// Monkey inspects and throws each of its items in order.
    pub fn inspect_and_throw(&mut self, reduce_worry: bool, supermodulo: u64) -> Vec<(usize, u64)> {
        let mut thrown_items: Vec<(usize, u64)> = vec![];
        loop {
            if self.items.is_empty() {
                break;
            }
            // Inspect item
            self.items_inspected += 1;
            match self.op {
                Operation::Add { value } => self.items[0] += value,
                Operation::Mult { value } => self.items[0] *= value,
                Operation::Pow { value } => self.items[0] = self.items[0].pow(value),
            }
            // Reduce the worry
            if reduce_worry {
                self.items[0] /= 3;
            }
            // Apply the supermodulo to reduce the item worry to stop it becoming too large
            self.items[0] %= supermodulo;
            if self.items[0] == 0 {
                self.items[0] = supermodulo;
            }
            // Check for throw
            let new_monkey = {
                if self.items[0] % self.divisor == 0 {
                    self.true_monkey
                } else {
                    self.false_monkey
                }
            };
            thrown_items.push((new_monkey, self.items.pop_front().unwrap()));
        }
        thrown_items
    }
}

/// Represents an operator performed on the worry level of items by monkey.
#[derive(Clone, Copy)]
pub enum Operation {
    Add { value: u64 },
    Mult { value: u64 },
    Pow { value: u32 }, // value is u32 here to allow use as exponent in .pow() method
}
