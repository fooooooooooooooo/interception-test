use std::io::Write;
use std::time::Duration;
use std::{io, thread};

use anyhow::Context;
use interception::{is_mouse, Filter, Interception, MouseFlags, MouseState, Stroke};
use windows::Win32::UI::WindowsAndMessaging::{GetSystemMetrics, SM_CXSCREEN, SM_CYSCREEN};

const SCALE: f64 = 350.0;
const STEP_DELAY_MS: u64 = 1;
// scuffed, doesn't accurately control the run time
const RUN_TIME_MS: i32 = 4000;
const TIME_STEP: f64 = 0.07;

fn run() -> anyhow::Result<()> {
  println!("Checking for interception driver");

  let ic = Interception::new().context("Failed to create interception context, is the driver installed?")?;

  println!("Context created successfully");

  ic.set_filter(is_mouse, Filter::MouseFilter(MouseState::all()));

  println!("Waiting for device, move the mouse");

  let mouse_device = ic.wait();

  ic.set_filter(is_mouse, Filter::MouseFilter(MouseState::empty()));

  println!("Found device: {}", mouse_device);

  let display_size = unsafe { (GetSystemMetrics(SM_CXSCREEN), GetSystemMetrics(SM_CYSCREEN)) };

  println!("Found display of size {}x{}", display_size.0, display_size.1);

  let mut position = Point((display_size.0 / 2).into(), (display_size.1 / 2).into());

  let stroke = Stroke::Mouse {
    state: MouseState::MOVE,
    flags: MouseFlags::MOVE_ABSOLUTE,
    rolling: 0,
    x: position.0 as i32,
    y: position.1 as i32,
    information: 0,
  };

  ic.send(mouse_device, &[stroke]);

  print!("Running test pattern for 5s");

  io::stdout().flush().context("failed to flush stdout buffer?")?;

  let mut t: f64 = 0.0;

  loop {
    t += TIME_STEP;

    position = lissajous(t);

    let stroke = Stroke::Mouse {
      state: MouseState::MOVE,
      flags: MouseFlags::MOVE_ABSOLUTE,
      rolling: 0,
      x: (position.0 as i32) + 30000,
      y: (position.1 as i32) + 30000,
      information: 0,
    };

    ic.send(mouse_device, &[stroke]);

    thread::sleep(Duration::from_millis(STEP_DELAY_MS));

    if ((t / TIME_STEP) * STEP_DELAY_MS as f64) * 10.0 >= RUN_TIME_MS.into() {
      break;
    }
  }

  println!("...done");

  println!("If the mouse did not move, there is an issue with your interception installation");

  Ok(())
}

fn main() {
  if let Err(err) = run() {
    println!("Error: {:?}", err);
  };

  dont_disappear::any_key_to_continue::default();
}

struct Point(f64, f64);

// cool shapes :)

// fn hypotrochoid(t: f64) -> Point {
//   let f: f64 = 10.0 / 7.0;
//   let r: f64 = 5.0;
//   let r2: f64 = 3.0;
//   let d: f64 = 5.0;
//   let c: f64 = r - r2;

//   let ctr = (c * t) / r;

//   Point(
//     SCALE * f * (c * t.cos() + d * (ctr).cos()),
//     SCALE * f * (c * t.sin() - d * (ctr).sin()),
//   )
// }

// fn mirabilis(t: f64) -> Point {
//   let f = 1.0 / 2.0;
//   let k = 1.0 / (2.0 * PI);

//   let kt_exp = (k * t).exp();

//   Point(SCALE * f * (kt_exp * t.cos()), SCALE * f * (kt_exp * t.sin()))
// }

fn lissajous(t: f64) -> Point {
  let f = 10.0;
  let a = 2.0;
  let b = 3.0;

  Point(SCALE * f * ((a * t).sin()), SCALE * f * ((b * t).sin()))
}
