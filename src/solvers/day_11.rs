use std::{cell::RefCell, collections::HashMap, str::Chars};

use crate::{args::exercise::Exercise, day_types::ExerciseEngine};

pub struct Solver11 {}

#[derive(Debug)]
struct Device {
    name: String,
    outputs: Vec<String>,
    cached_exploration: [[RefCell<Option<i128>>; 2]; 2],
}

fn string_of_3_chars(chars: &mut Chars<'_>) -> String {
    [
        chars.next().unwrap(),
        chars.next().unwrap(),
        chars.next().unwrap(),
    ]
    .iter()
    .collect()
}

impl Device {
    fn from_string(chars: &mut Chars<'_>) -> Self {
        let mut output: Vec<String> = Vec::new();
        let name: String = string_of_3_chars(chars);

        chars.next();
        chars.next();

        loop {
            output.push(string_of_3_chars(chars));
            if chars.next().is_none() {
                break;
            };
        }

        Self {
            outputs: output,
            name,
            cached_exploration: [
                [RefCell::new(None), RefCell::new(None)],
                [RefCell::new(None), RefCell::new(None)],
            ],
        }
    }

    fn out() -> Self {
        Self {
            name: "out".into(),
            outputs: Vec::new(),
            cached_exploration: [
                [RefCell::new(None), RefCell::new(None)],
                [RefCell::new(None), RefCell::new(None)],
            ],
        }
    }

    fn explore_simple(&self, devices: &HashMap<String, Device>) -> i128 {
        if self.name == "out".to_string() {
            return 1_i128;
        }

        let mut cnt = 0_i128;

        for output in &self.outputs {
            cnt += devices[output].explore_simple(devices);
        }
        cnt
    }

    fn explore_fft_dac(
        &self,
        devices: &HashMap<String, Device>,
        fft_found: bool,
        dac_found: bool,
    ) -> i128 {
        if let Some(res) = *self.cached_exploration[fft_found as usize][dac_found as usize].borrow()
        {
            return res;
        }

        if self.name == "out".to_string() {
            return (fft_found && dac_found) as i128;
        }

        let mut cnt = 0_i128;

        for output in &self.outputs {
            cnt += devices[output].explore_fft_dac(
                devices,
                fft_found || self.name == "fft".to_string(),
                dac_found || self.name == "dac".to_string(),
            );
        }

        self.cached_exploration[fft_found as usize][dac_found as usize].replace(Some(cnt));
        cnt
    }
}

impl ExerciseEngine for Solver11 {
    fn solve(&self, args: crate::args::Args, lines: Vec<String>) -> i128 {
        let mut devices: HashMap<String, Device> = lines
            .into_iter()
            .map(|line| {
                let mut chars = line.chars();

                let device = Device::from_string(&mut chars);
                (device.name.clone(), device)
            })
            .collect();

        devices.insert("out".into(), Device::out());

        match args.exercise {
            Exercise::One => devices["you".into()].explore_simple(&devices),
            Exercise::Two => devices["svr".into()].explore_fft_dac(&devices, false, false),
        }
    }
}
