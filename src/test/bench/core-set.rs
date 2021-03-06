// Copyright 2013 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern mod extra;

use extra::bitv::BitvSet;
use extra::treemap::TreeSet;
use std::hashmap::HashSet;
use std::io;
use std::os;
use std::rand;
use std::uint;

struct Results {
    sequential_ints: float,
    random_ints: float,
    delete_ints: float,

    sequential_strings: float,
    random_strings: float,
    delete_strings: float
}

fn timed(result: &mut float, op: &fn()) {
    let start = extra::time::precise_time_s();
    op();
    let end = extra::time::precise_time_s();
    *result = (end - start);
}

impl Results {
    pub fn bench_int<T:MutableSet<uint>,
                 R: rand::Rng>(
                 &mut self,
                 rng: &mut R,
                 num_keys: uint,
                 rand_cap: uint,
                 f: &fn() -> T) {
        {
            let mut set = f();
            do timed(&mut self.sequential_ints) {
                for i in range(0u, num_keys) {
                    set.insert(i);
                }

                for i in range(0u, num_keys) {
                    assert!(set.contains(&i));
                }
            }
        }

        {
            let mut set = f();
            do timed(&mut self.random_ints) {
                for _ in range(0, num_keys) {
                    set.insert((rng.next() as uint) % rand_cap);
                }
            }
        }

        {
            let mut set = f();
            for i in range(0u, num_keys) {
                set.insert(i);
            }

            do timed(&mut self.delete_ints) {
                for i in range(0u, num_keys) {
                    assert!(set.remove(&i));
                }
            }
        }
    }

    pub fn bench_str<T:MutableSet<~str>,
                 R:rand::Rng>(
                 &mut self,
                 rng: &mut R,
                 num_keys: uint,
                 f: &fn() -> T) {
        {
            let mut set = f();
            do timed(&mut self.sequential_strings) {
                for i in range(0u, num_keys) {
                    let s = uint::to_str(i);
                    set.insert(s);
                }

                for i in range(0u, num_keys) {
                    let s = uint::to_str(i);
                    assert!(set.contains(&s));
                }
            }
        }

        {
            let mut set = f();
            do timed(&mut self.random_strings) {
                for _ in range(0, num_keys) {
                    let s = uint::to_str(rng.next() as uint);
                    set.insert(s);
                }
            }
        }

        {
            let mut set = f();
            for i in range(0u, num_keys) {
                set.insert(uint::to_str(i));
            }
            do timed(&mut self.delete_strings) {
                for i in range(0u, num_keys) {
                    assert!(set.remove(&uint::to_str(i)));
                }
            }
        }
    }
}

fn write_header(header: &str) {
    io::stdout().write_str(header);
    io::stdout().write_str("\n");
}

fn write_row(label: &str, value: float) {
    io::stdout().write_str(fmt!("%30s %f s\n", label, value));
}

fn write_results(label: &str, results: &Results) {
    write_header(label);
    write_row("sequential_ints", results.sequential_ints);
    write_row("random_ints", results.random_ints);
    write_row("delete_ints", results.delete_ints);
    write_row("sequential_strings", results.sequential_strings);
    write_row("random_strings", results.random_strings);
    write_row("delete_strings", results.delete_strings);
}

fn empty_results() -> Results {
    Results {
        sequential_ints: 0f,
        random_ints: 0f,
        delete_ints: 0f,

        sequential_strings: 0f,
        random_strings: 0f,
        delete_strings: 0f,
    }
}

fn main() {
    let args = os::args();
    let num_keys = {
        if args.len() == 2 {
            uint::from_str(args[1]).unwrap()
        } else {
            100 // woefully inadequate for any real measurement
        }
    };

    let seed = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let max = 200000;

    {
        let mut rng = rand::IsaacRng::new_seeded(seed);
        let mut results = empty_results();
        results.bench_int(&mut rng, num_keys, max, || HashSet::new::<uint>());
        results.bench_str(&mut rng, num_keys, || HashSet::new::<~str>());
        write_results("std::hashmap::HashSet", &results);
    }

    {
        let mut rng = rand::IsaacRng::new_seeded(seed);
        let mut results = empty_results();
        results.bench_int(&mut rng, num_keys, max, || TreeSet::new::<uint>());
        results.bench_str(&mut rng, num_keys, || TreeSet::new::<~str>());
        write_results("extra::treemap::TreeSet", &results);
    }

    {
        let mut rng = rand::IsaacRng::new_seeded(seed);
        let mut results = empty_results();
        results.bench_int(&mut rng, num_keys, max, || BitvSet::new());
        write_results("extra::bitv::BitvSet", &results);
    }
}
