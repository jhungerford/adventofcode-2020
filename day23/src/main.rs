use day23::Cups;

fn main() {
    let input = 253149867;

    println!("Part 1: {}", Cups::from(input).shift_times(100).code_after(1));
    println!("Part 2: {}", Cups::million_from(input).shift_times(10_000_000).product_after(1));
}
