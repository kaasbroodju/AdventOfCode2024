#![feature(portable_simd)]

mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;
mod day_09;
mod day_10;
mod day_11;

fn main() {
    day_11::first_part();
    // day_03::first_part_state_machine();
    day_11::second_part();
    // day_04::second_part_one_line();
    // day_04::second_part_one_line_simd();
    // day_03::second_part_state_machine();
}