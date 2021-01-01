use std::iter::once;

#[derive(Debug, Eq, PartialEq)]
pub struct Cups {
    cups: Vec<i32>
}

impl Cups {
    /// Converts the given number into a game of cups.
    pub fn from(num: i32) -> Self {
        Cups { cups: Cups::code_cups(num) }
    }

    /// Converts the given number into a game of million cups.  The cups will start with the
    /// number's digits in order, then continue with cups numbered up until 1,000,000.
    pub fn million_from(num: i32) -> Self {
        let mut cups = Cups::code_cups(num);

        for cup in cups.len() + 1 .. 1_000_000 {
            cups.push(cup as i32);
        }

        Cups { cups }
    }

    fn code_cups(num: i32) -> Vec<i32> {
        let mut cups = Vec::new();

        let mut remaining_num = num;
        while remaining_num > 0 {
            cups.push(remaining_num % 10);
            remaining_num /= 10;
        }

        cups.reverse();

        cups
    }

    /// Shifts these cups once.
    pub fn shift(&mut self) -> &mut Self {
        // Crab picks up 3 cups immediately clockwise of the current cup.
        let num_cups = self.cups.len() as i32;

        let cup = self.cups.remove(0);
        let picked_up: Vec<i32> = self.cups.drain(0..3).collect();

        // Selects a destination cup: cup with label = current - 1.  If the label belongs
        // to a cup that was just picked up, keeps subtracting until it finds a valid cup.
        // Wraps around to the highest value if necessary.
        let mut target = cup - 1;
        if target == 0 {
            target = num_cups;
        }

        while picked_up.contains(&target) {
            target -= 1;
            if target == 0 {
                target = num_cups;
            }
        }


        let maybe_target_index = self.cups.iter().position(|&cup| cup == target);
        if maybe_target_index.is_none() {
            println!("# Cups: {}, Cup: {}, picked_up: {:?}, target: {}", num_cups, cup, &picked_up, target);
        }

        let target_index = maybe_target_index.unwrap();

        // Places the picked up cups clockwise of the destination cup, preserving their order.
        let new_cups = self.cups[0..=target_index].iter().cloned()
            .chain(picked_up.iter().cloned())
            .chain(self.cups[target_index+1..].iter().cloned())
            .chain(once(cup))
            .collect();

        // println!("Cup: {}, picked_up: {:?}, new_cups: {:?}", cup, &picked_up, &new_cups);

        self.cups = new_cups;

        // Selects a new cup immediately clockwise of the current cup (head of cups.)
        self
    }

    /// Shifts these cups 'turns' times.
    pub fn shift_times(&mut self, turns: usize) -> &mut Self {
        for _ in 0..turns {
            self.shift();
        }

        self
    }

    /// Returns the code for these cups.
    pub fn code(&self) -> i32 {
        let mut code = 0;
        for cup in &self.cups {
            code *= 10;
            code += cup;
        }

        code
    }

    /// Returns the code for the cups after the numbered cup.
    pub fn code_after(&self, num: i32) -> i32 {
        let num_index = self.cups.iter().position(|&cup| cup == num).unwrap();

        let mut code = 0;

        if num_index < self.cups.len() - 1 {
            for i in num_index + 1 .. self.cups.len() {
                code *= 10;
                code += self.cups[i];
            }
        }

        if num_index > 0 {
            for i in 0..num_index {
                code *= 10;
                code += self.cups[i];
            }
        }

        code
    }

    /// Returns the product of the two numbers that occur after the given number.
    pub fn product_after(&self, num: i32) -> i64 {
        let num_index = self.cups.iter().position(|&cup| cup == num).unwrap();
        let cup1 = self.cups[(num_index + 1) % self.cups.len()];
        let cup2 = self.cups[(num_index + 2) % self.cups.len()];

        cup1 as i64 * cup2 as i64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_sample() {
        assert_eq!(Cups { cups: vec![3, 2, 4, 1, 5] }, Cups::from(32415));
    }

    #[test]
    fn sample_code() {
        assert_eq!(32415, Cups::from(32415).code());
        assert_eq!(389125467, Cups::from(389125467).code());
        assert_eq!(92658374, Cups::from(92658374).code());
    }

    #[test]
    fn sample_code_after() {
        assert_eq!(92658374, Cups::from(583741926).code_after(1));
        assert_eq!(58374926, Cups::from(583749261).code_after(1));
        assert_eq!(58374926, Cups::from(158374926).code_after(1));
    }

    #[test]
    fn shift_sample() {
        let mut cups = Cups::from(389125467);

        assert_eq!(Cups::from(289154673), *cups.shift());
        assert_eq!(Cups::from(546789132), *cups.shift());
        assert_eq!(Cups::from(891346725), *cups.shift());
        assert_eq!(Cups::from(467913258), *cups.shift());
        assert_eq!(Cups::from(136792584), *cups.shift());
        assert_eq!(Cups::from(936725841), *cups.shift());
        assert_eq!(Cups::from(258367419), *cups.shift());
        assert_eq!(Cups::from(674158392), *cups.shift());
        assert_eq!(Cups::from(574183926), *cups.shift());
        assert_eq!(Cups::from(837419265), *cups.shift());
    }

    #[test]
    fn shift_times_sample() {
        let mut cups = Cups::from(389125467);
        assert_eq!(67384529, cups.shift_times(100).code_after(1));
    }

    #[test]
    fn shift_times_million_sample() {
        let mut cups = Cups::million_from(389125467);
        cups.shift_times(10_000_000);
        assert_eq!(149245887792, cups.product_after(1));
    }
}
