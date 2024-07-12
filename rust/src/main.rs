use rand::Rng;
use std::collections::HashMap;
use std::{thread, time};

fn main() {
    let print_area = 60;
    let mut grid = seed_grid(print_area);
    print(print_area, &grid);

    let mut generation = 0;
    loop {
        let mut new_grid: HashMap<(i32, i32), i32> = Default::default();
        let keys: Vec<&(i32, i32)> = grid.keys().collect();

        for key in keys {
            let val = grid[key];
            let (x, y) = key;
            let mut population = 0;
            let nb_coords = neighbour_coords(*x, *y);
            for coord in &nb_coords {
                if grid.contains_key(coord) {
                    population += grid[coord];
                }
            }

            if val == 1 && (population == 2 || population == 3) {
                new_grid.insert(key.clone(), 1);
                for coord in &nb_coords {
                    if !new_grid.contains_key(coord) {
                        new_grid.insert(coord.clone(), 0);
                    }
                }
            }

            if val == 0 && population == 3 {
                new_grid.insert(key.clone(), 1);
                for coord in &nb_coords {
                    if !new_grid.contains_key(coord) {
                        new_grid.insert(coord.clone(), 0);
                    }
                }
            }
        }

        grid = new_grid;
        generation += 1;
        std::process::Command::new("clear").status().unwrap();
        print(print_area, &grid);
        let total_population: i32 = grid.values().sum();
        println!("Generation: {generation}");
        println!("Population: {total_population}");
        let millis = time::Duration::from_millis(100);
        thread::sleep(millis);
    }
}

fn seed_grid(print_area: i32) -> HashMap<(i32, i32), i32> {
    let mut grid: HashMap<(i32, i32), i32> = Default::default();
    for _ in 0..35 {
        let x = rand::thread_rng().gen_range(print_area / 4..print_area - (print_area / 4));
        let y = rand::thread_rng().gen_range(print_area / 4..print_area - (print_area / 4));

        grid.insert((x, y), 1);
        grid.insert((x, y + 1), 1);
        grid.insert((x + 1, y), 1);

        for coord in neighbour_coords(x, y) {
            if !grid.contains_key(&coord) {
                grid.insert(coord, 0);
            }
        }

        for coord in neighbour_coords(x, y + 1) {
            if !grid.contains_key(&coord) {
                grid.insert(coord, 0);
            }
        }

        for coord in neighbour_coords(x + 1, y) {
            if !grid.contains_key(&coord) {
                grid.insert(coord, 0);
            }
        }
    }
    return grid;
}

fn neighbour_coords(x: i32, y: i32) -> Vec<(i32, i32)> {
    let mut coords: Vec<(i32, i32)> = vec![];
    coords.push((x + 1, y));
    coords.push((x, y + 1));
    coords.push((x + 1, y + 1));
    coords.push((x - 1, y - 1));
    coords.push((x - 1, y));
    coords.push((x, y - 1));
    coords.push((x + 1, y - 1));
    coords.push((x - 1, y + 1));
    return coords;
}

fn print(print_area: i32, grid: &HashMap<(i32, i32), i32>) {
    for y in 0..print_area {
        let mut points: Vec<String> = vec![];
        for x in 0..print_area {
            let key = (x, y);
            if !grid.contains_key(&key) || grid[&key] == 0 {
                points.push('.'.to_string());
            } else {
                points.push('#'.to_string());
            }
        }
        let line = points.join(" ");
        println!("{line}");
    }
}
