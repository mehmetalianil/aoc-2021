#![no_main]
#![no_std]

use ao_c2021 as _; // global logger + panicking-behavior + memory layout


#[path = "../firstdata.rs"]
mod firstdata;

use firstdata::input_const;

fn increment_counter(input: &[u32])-> u32 {
    let mut buffer = u32::MAX;
    let mut counter = 0u32;
    for value in input.iter(){
        //defmt::println!("{=u32},{=u32},{=u32}",*value, buffer, counter);
        if *value > buffer {
            counter += 1;
        }
        buffer = *value;
    }
    counter
}

fn increment_counter_windowed(input: &[u32])-> u32 {
    let mut counter = 0u32;
    let mut window = [0u32;3];

    let mut input_iter = input.iter();
    for idx in 0..3{
        window[idx] = *input_iter.next().unwrap();
    }
    defmt::println!("{}",window);

    
    for value in input.iter(){
        //defmt::println!("{=u32},{=u32},{=u32}",*value, buffer, counter);
        if *value > window[0] {
            counter += 1;
        }
        window[0] = window[1];
        window[1] = window[2];
        window[2] = *value;
    }
    counter
}

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::println!("Hello, AoC!");
    let result = increment_counter(&input_const);
    let result2 = increment_counter_windowed(&input_const);
    defmt::println!("First puzzle: {=u32}", result);
    defmt::println!("First puzzle: {=u32}", result2);
    ao_c2021::exit()
}
