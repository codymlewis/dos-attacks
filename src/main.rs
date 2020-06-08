// Copyright (C) 2020 Cody Lewis
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

extern crate pnet;

use std::env;

mod icmp_flood;

fn main() {
    println!("dos-attacks  Copyright (C) 2020  Cody Lewis");
    println!("This program comes with ABSOLUTELY NO WARRANTY.");
    println!("This is free software, and you are welcome to redistribute it under certain conditions.");
    println!();
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        panic!("Not enough arguments");
    }
    let attack_name = &args[1];
    let addr = &args[2];
    run_attack(attack_name)(addr);
}

fn run_attack(attack: &String) -> Box<dyn Fn(&String)> {
    match attack.as_str() {
        "ping-flood" => Box::new(icmp_flood::run),
        _ => return Box::new(move |a| panic!("No attack named {}", a)),
    }
}
