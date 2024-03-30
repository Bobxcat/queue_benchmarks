use std::{
    collections::{LinkedList, VecDeque},
    sync::Mutex,
};

use criterion::{
    black_box, criterion_group, criterion_main, measurement::Measurement, BenchmarkGroup,
    BenchmarkId, Criterion,
};
use queue_benchmarks::Queue;

trait TestInput: Clone + Eq + Sized {
    fn get_input_with_offset(length: usize, offset: usize) -> Vec<Self> {
        let mut i = Self::get_input(length + offset);
        i.split_off(offset)
    }
    fn get_input(length: usize) -> Vec<Self>;
}

// impl TestInput for String {
//     fn get_input(length: usize) -> Vec<Self> {
//         todo!()
//     }
// }

impl TestInput for i32 {
    fn get_input(length: usize) -> Vec<Self> {
        let mut v = vec![];
        while v.len() < length {
            v.push(100);
            v.push(101);
            v.push(102);
            v.push(-2);
        }
        v.truncate(length);
        v
    }
}

#[inline]
fn push_pop<T>(q: &mut impl Queue<T>, input: &mut Vec<T>, push: usize, pop: usize) -> Option<()> {
    for _ in 0..push {
        q.push(input.pop()?);
    }
    for _ in 0..pop {
        q.pop();
    }
    None
}

fn bench_queue<T: TestInput, Q: Queue<T>>(grp: &mut BenchmarkGroup<'_, impl Measurement>) {
    const TEST_SIZES: &[usize] = &[10usize, 100, 1000, 10_000, 100_000];
    for &i in TEST_SIZES {
        let id = BenchmarkId::new(std::any::type_name::<Q>(), i);
        let input = T::get_input_with_offset(i, i);
        grp.bench_function(id, move |b| {
            b.iter(|| {
                let mut q = Q::init_empty();
                let mut input = input.clone();
                let mut x = 10;
                'outer: loop {
                    for a in 0..x {
                        for x in 0..x / 7 {
                            for y in 0..x / 7 {
                                let Some(()) =
                                    push_pop(black_box(&mut q), &mut input, a + y, a + x)
                                else {
                                    break 'outer;
                                };
                            }
                        }
                    }

                    x *= 10;
                }
            });
        });
    }
}

fn bench_main(c: &mut Criterion) {
    let mut grp = c.benchmark_group("Queue Benchmarks By Input Size [i32]");

    bench_queue::<_, Vec<i32>>(&mut grp);
    bench_queue::<_, VecDeque<i32>>(&mut grp);
    bench_queue::<_, LinkedList<i32>>(&mut grp);

    grp.finish()
}

criterion_group!(benches, bench_main);
criterion_main!(benches);
