#[derive(Debug, Eq, PartialEq)]
pub struct Cups {
    /// curr is the current cup's number.
    curr: usize,
    /// next maps a cup at an index to the next cup (value).
    next: Vec<usize>,
}

impl Cups {
    /// Converts the given number into a game of cups.
    pub fn from(num: i32) -> Self {
        Cups::new(Cups::code_cups(num))
    }

    /// Converts the given number into a game of million cups.  The cups will start with the
    /// number's digits in order, then continue with cups numbered up until 1,000,000.
    pub fn million_from(num: i32) -> Self {
        // Cups is an ordered list of cups - fill in the remaining cups up to a million.
        let mut cups = Cups::code_cups(num);

        for cup in cups.len() + 1 ..= 1_000_000 {
            cups.push(cup);
        }

        Cups::new(cups)
    }

    /// Returns a game of cups based off of the given cups, in order.
    fn new(cups: Vec<usize>) -> Self {
        // Game starts with the first cup.
        let curr = cups[0];

        // Next maps a cup (at an index) to the next cup (value)
        let max_cup = *cups.iter().max().unwrap();
        let mut next = vec![0; max_cup as usize + 1];
        for i in 0..cups.len() - 1 {
            next[cups[i]] = cups[i + 1];
        }
        next[cups[cups.len() - 1]] = cups[0];

        Cups { curr, next }
    }

    /// code_cups splits the given number into a list of cups, in order.
    fn code_cups(num: i32) -> Vec<usize> {
        let mut cups = Vec::new();

        let mut remaining_num = num;
        while remaining_num > 0 {
            cups.push(remaining_num as usize % 10);
            remaining_num /= 10;
        }

        cups.reverse();

        cups
    }

    /// Shifts these cups once.
    pub fn shift(&mut self) -> &mut Self {

        // Crab picks up 3 cups immediately clockwise of the current cup.
        let picked_up_1 = self.next[self.curr];
        let picked_up_2 = self.next[picked_up_1];
        let picked_up_3 = self.next[picked_up_2];

        let after = self.next[picked_up_3];

        // Selects a destination cup: cup with label = current - 1.  If the label belongs
        // to a cup that was just picked up, keeps subtracting until it finds a valid cup.
        // Wraps around to the highest value if necessary.
        let mut target = self.curr - 1;
        if target == 0 {
            target = self.next.len() - 1;
        }

        while target == picked_up_1 || target == picked_up_2 || target == picked_up_3 {
            target -= 1;
            if target == 0 {
                target = self.next.len() - 1;
            }
        }

        // Places the picked up cups clockwise of the destination cup, preserving their order.
        let old_target_next = self.next[target];
        self.next[target] = picked_up_1;
        self.next[picked_up_3] = old_target_next;
        self.next[self.curr] = after;

        // println!("Cup: {}, picked_up: {:?}, new_cups: {:?}", cup, &picked_up, &new_cups);

        // Selects a new cup immediately clockwise of the current cup (head of cups.)
        self.curr = after;

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
        assert!(self.next.len() <= 10, "Code is only valid for up to 9 cups.");

        let mut code = self.curr;
        let mut cup = self.next[self.curr];
        while cup != self.curr {
            code *= 10;
            code += cup;
            cup = self.next[cup];
        }

        code as i32
    }

    /// Returns the code for the cups after the numbered cup.
    pub fn code_after(&self, after_cup: usize) -> i32 {
        assert!(self.next.len() <= 10, "code_after is only valid for up to 9 cups.");

        let mut code = 0;
        let mut cup = self.next[after_cup];
        while cup != after_cup {
            code *= 10;
            code += cup;
            cup = self.next[cup];
        }

        code as i32
    }

    /// Returns the product of the two numbers that occur after the given number.
    pub fn product_after(&self, num: usize) -> i64 {
        let num1 = self.next[num];
        let num2 = self.next[num1];

        num1 as i64 * num2 as i64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_sample() {
        assert_eq!(Cups { curr: 3, next: vec![0, 5, 4, 2, 1, 3] }, Cups::from(32415));
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
    fn sample_product_after() {
        assert_eq!(10, Cups::from(389125467).product_after(1));
        assert_eq!(18, Cups::from(389125467).shift_times(10).product_after(1));
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
    fn million_from_sample() {
        let cups = Cups::million_from(389125467);
        assert_eq!(1_000_001, cups.next.len()); // Zero is an empty cup - want 1M actual cups.
        assert_eq!(3, cups.curr);
    }

    #[test]
    fn shift_times_million_sample() {
        let mut cups = Cups::million_from(389125467);
        cups.shift_times(10_000_000);
        assert_eq!(149245887792, cups.product_after(1));
    }
}
