/*********************************************************************************************************************** 
 * Copyright (c) 2019 by the authors
 * 
 * Author: {{authors}}
 * License: {{license}}
 **********************************************************************************************************************/
#![no_std]
#![no_main]

//! # {{project-name}}
//! 

#[macro_use]
extern crate ruspiro_boot;
extern crate ruspiro_allocator;

come_alive_with!(kernel_alive);
run_with!(kernel_run);

pub fn kernel_alive(core: u32) {
    // your one-time initialization goes here
    println!("Kernel alive on core {}", core);
}

pub fn kernel_run(core: u32) -> ! {
    // your processing logic goes here
    println!("Kernel running on core {}", core);
    // never return from here...
    loop {}
}