#[cfg(target_os = "hermit")]
use hermit_sys as _;

use std::thread;
use std::time::Instant;

const NUM_STEPS: u64 = 100000000;
const NTHREADS: u64 = 2;

fn main() {
	let step = 1.0 / NUM_STEPS as f64;
	let mut sum = 0.0 as f64;
	let now = Instant::now();

	let threads: Vec<_> = (0..NTHREADS)
		.map(|tid| {
			thread::spawn(move || {
				let mut partial_sum = 0 as f64;
				let start = (NUM_STEPS / NTHREADS) * tid;
				let end = (NUM_STEPS / NTHREADS) * (tid + 1);

				for i in start..end {
					let x = (i as f64 + 0.5) * step;
					partial_sum += 4.0 / (1.0 + x * x);
				}

				partial_sum
			})
		})
		.collect();

	for t in threads {
		sum += t.join().unwrap();
	}

	let duration = now.elapsed();

	println!(
		"Time to calculate (local sum): {}",
		duration.as_secs() as f64 + (duration.subsec_nanos() as f64 / 1000000000.0)
	);
	println!("Pi: {}", sum * (1.0 / NUM_STEPS as f64));
}
