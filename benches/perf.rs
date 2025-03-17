use std::{fs::File, os::raw::c_int, path::Path};

use criterion::profiler::Profiler;
use pprof::{ProfilerGuard, ProfilerGuardBuilder};

/// Small custom profiler that can be used with Criterion to create a flamegraph for benchmarks.
/// Also see [the Criterion documentation on this][custom-profiler].
///
/// ## Example on how to enable the custom profiler:
///
/// ```
/// mod perf;
/// use perf::FlamegraphProfiler;
///
/// fn fibonacci_profiled(criterion: &mut Criterion) {
///     // Use the criterion struct as normal here.
/// }
///
/// fn custom() -> Criterion {
///     Criterion::default().with_profiler(FlamegraphProfiler::new())
/// }
///
/// criterion_group! {
///     name = benches;
///     config = custom();
///     targets = fibonacci_profiled
/// }
/// ```
///
/// The neat thing about this is that it will sample _only_ the benchmark, and not other stuff like
/// the setup process.
///
/// Further, it will only kick in if `--profile-time <time>` is passed to the benchmark binary.
/// A flamegraph will be created for each individual benchmark in its report directory under
/// `profile/flamegraph.svg`.
///
/// [custom-profiler]: https://bheisler.github.io/criterion.rs/book/user_guide/profiling.html#implementing-in-process-profiling-hooks
pub struct FlamegraphProfiler<'a> {
    frequency: c_int,
    active_profiler: Option<ProfilerGuard<'a>>,
}

impl FlamegraphProfiler<'_> {
    #[allow(dead_code)]
    pub fn new(frequency: c_int) -> Self {
        FlamegraphProfiler {
            frequency,
            active_profiler: None,
        }
    }
}

impl Profiler for FlamegraphProfiler<'_> {
    fn start_profiling(&mut self, _benchmark_id: &str, _benchmark_dir: &Path) {
        let profiler = ProfilerGuardBuilder::default().frequency(self.frequency).build().unwrap();

        self.active_profiler = Some(profiler);
    }

    fn stop_profiling(&mut self, _benchmark_id: &str, _benchmark_dir: &Path) {
        let flamegraph_path = Path::new("flamegraph.svg");
        let flamegraph_file = File::create(flamegraph_path).expect("File system error while creating flamegraph.svg");
        if let Some(profiler) = self.active_profiler.take() {
            profiler
                .report()
                .build()
                .unwrap()
                .flamegraph(flamegraph_file)
                .expect("Error writing flamegraph");
        }
    }
}
