#[macro_use]
extern crate bencher;

use bencher::Bencher;

fn day11_part1(bench: &mut Bencher) {
    bench.iter(|| solutions::day11::part1(true));
}

fn day11_part2(bench: &mut Bencher) {
    bench.iter(|| solutions::day11::part2(true));
}

benchmark_group!(benches, day11_part1, day11_part2);
benchmark_main!(benches);
