use std::env;
use std::fs;

fn main() {

    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let contents = fs::read_to_string(file_path)
        .expect("Couldn't read the input file...");

    let lines = contents.lines();

    let mut running_total: u64 = 0;
    let mut place_1_elf: u64 = 0;
    let mut place_2_elf: u64 = 0;
    let mut place_3_elf: u64 = 0;



    for line in lines {
        if line == "" {
            if running_total > place_1_elf {
                place_3_elf = place_2_elf;
                place_2_elf = place_1_elf;
                place_1_elf = running_total
            }
            else if running_total > place_2_elf {
                place_3_elf = place_2_elf;
                place_2_elf = running_total;
            }
            else if running_total > place_3_elf {
                place_3_elf = running_total;
            }
            running_total = 0;
            continue
        }

        let this_foods_calories : u64 = line.parse().unwrap();
        running_total += this_foods_calories
    }

    // The last elf doesn't log a new line
    if running_total > place_1_elf {
        place_3_elf = place_2_elf;
        place_2_elf = place_1_elf;
        place_1_elf = running_total
    }
    else if running_total > place_2_elf {
        place_3_elf = place_2_elf;
        place_2_elf = running_total;
    }
    else if running_total > place_3_elf {
        place_3_elf = running_total;
    }

    println!("{}", place_1_elf + place_2_elf + place_3_elf);
}
