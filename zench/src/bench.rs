use std::fmt::Display;
use std::time::Instant;

use crate::__internal::*;
use crate::benchmark::Benchmark;
use crate::engine::EngineAuto;
use crate::engine::IEngine;
use crate::global::Command;
use crate::global::Ignore;
use crate::location;
use crate::report::Report;
use crate::warmup::Warmup;

//#[doc = include_str!("../docs/zench_Bench.md")]
#[non_exhaustive]
#[derive(Clone, Debug)]
pub struct Bench<E> {
    pub(crate) warmup: Warmup,
    pub(crate) engine: E,
    pub(crate) report: Report,
    pub(crate) is_first_run: bool,
    pub(crate) initial_time: Option<Instant>,
}

impl Default for Bench<EngineAuto> {
    fn default() -> Self {
        Bench {
            warmup: Warmup::disabled(),
            engine: EngineAuto::default(),
            report: Report::new(),
            is_first_run: true,
            initial_time: None,
        }
    }
}

impl Bench<EngineAuto> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<E> Bench<E>
where
    E: IEngine,
{
    pub fn with_engine(e: E) -> Self
    where
        E: IEngine,
    {
        Bench {
            warmup: Warmup::disabled(),
            engine: e,
            report: Report::new(),
            is_first_run: true,
            initial_time: None,
        }
    }

    pub fn warmup(mut self, w: Warmup) -> Self {
        self.warmup = w;
        self
    }

    #[track_caller] // #[track_caller] to get location caller, do not remove
    pub fn bench<F>(&mut self, name: impl Display, mut closure: F)
    where
        F: FnMut(),
    {
        if Ignore::should_ignore() {
            let location = location!();

            if Ignore::already_seen(&location) || Command::not_set() {
                return;
            }

            if let Some(msg) = Ignore::reason() {
                fprintln!("\nbench {location} ... {msg}");
                Ignore::mark_seen(location);
            }

            return;
        }

        // -----------------------------------------
        // Initial preparations

        if self.is_first_run {
            self.init_timer();

            self.warmup
                .run(&mut || _ = closure);

            self.warmup_engine_once();
            self.is_first_run = false;

            // print current bench line
        }
        // -----------------------------------------
        // // print current bench line

        fprintln!(
            "bench {location}::{name}",
            name = name,
            location = location!()
        );

        // -----------------------------------------
        // collect data

        let (data, iters) = self
            .engine
            .collect_data(&mut closure);

        // -----------------------------------------
        // report

        let bench = Benchmark::new(name.to_string(), location!(), data, iters);

        self.report
            .push(bench);

        self.report
            .initial_time = self.initial_time;

        //fprint!(".");
    }

    pub fn report<F>(&mut self, r: F)
    where
        F: FnOnce(&mut Report),
    {
        // if !Ignore::should_ignore() {
        //     //let mut report = std::mem::take(&mut self.report);
        //     r(&mut self.report);
        // }

        if !Ignore::should_ignore() {
            let mut report = std::mem::take(&mut self.report);

            // copy only the initial time to the bench.report
            // to prevent return 'total time: unknown sec'
            self.report
                .initial_time = report.initial_time;

            r(&mut report);
        }
    }

    fn init_timer(&mut self) {
        self.initial_time = Some(Instant::now());
    }

    // just one execution
    fn warmup_engine_once(&self) {
        // create a empty closure for self warmup
        let mut empty_closure = || {};
        self.engine
            .collect_data(&mut empty_closure);
    }
}
