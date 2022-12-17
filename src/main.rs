#![feature(get_many_mut)]
use puzzles::*;

mod puzzles;

fn main() {
    // println!("Highest 3 Calories: {:?}", day_1::highest_calories(3));
    // println!("RPS Score: {:?}", day_2::score());
    // println!("Rucksack Sum: {:?}", day_3::sum());
    // println!("Badge Sum: {:?}", day_3::badge_sum());
    // println!("Overlap Sum: {:?}", day_4::overlap_sum());
    // println!("Overlap any Sum: {:?}", day_4::overlap_any_sum());
    // println!("Crates: {}", day_5::print_crates());
    // println!("First Marker: {}", day_6::first_marker(14));
    // println!(
    //     "Sum of dirs less than 100000: {}",
    //     day_7::sum_of_dir_smaller_than_100000()
    // );
    // println!(
    //     "Size of smallest dir to delete: {}",
    //     day_7::size_of_smallest_dir_to_delete()
    // );
    println!("Number of visible trees: {}", day_8::visible());
    println!("Highest Scenic Score: {}", day_8::max_scenic_score());
}
