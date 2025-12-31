use std::fs::File;
use std::io::read_to_string;
use std::thread::current;
// fn main() {
//     let file = File::open("./input.txt").expect("file not found");
//     let file_string_lines =  read_to_string(file).unwrap();
//     let mut layout : Vec<Vec<char>> = Vec::new();
//     let mut sum = 0;
//
//
//     for line in file_string_lines.lines() {
//         let split_chars = line.chars().collect::<Vec<char>>();
//         layout.push(split_chars);
//     }
//     for j in 0..layout.len() {
//         for i in 0..layout[j].len() {
//
//             sum += check_coordinates(&layout,i,j);
//
//         }
//         print!("\n");
//     }
//
//     println!("Sum: {}",sum);
//     // println!("Hello, world!");
// }
//
// fn check_coordinates(layout: &Vec<Vec<char>>, x: usize, y: usize) -> i32 {
//     let mut all_surrounding_rolls = 0;
//     let length_y = layout[0].len();
//     let length_x = layout.len();
//
//     if layout[y][x] != '@' {
//         print!("{}", layout[y][x]);
//         return 0;
//     }
//
//     for j in -1..=1 {
//         for i in -1..=1 {
//             if i == 0 && j == 0 {continue;}
//             let adjacent_y = y as i32 - j;
//             let adjacent_x = x as i32 - i;
//             if adjacent_y < 0
//                 || adjacent_x < 0
//                 || adjacent_y >= length_y as i32
//                 || adjacent_x >= length_x as i32
//             {
//                 continue;
//             }
//
//             if layout[adjacent_y as usize][adjacent_x as usize] == '@' {
//                 all_surrounding_rolls += 1;
//             }
//         }
//     }
//
//     if all_surrounding_rolls < 4 {
//         print!("x");
//         return 1;
//     }
//     print!("{}",layout[y][x]);
//     0
//
// }

fn main() {
    let file = File::open("./input.txt").expect("file not found");
    let file_string_lines =  read_to_string(file).unwrap();
    let mut layout : Vec<Vec<char>> = Vec::new();
    let mut total_sum = 0;
    let mut current_sum = 100;

    for line in file_string_lines.lines() {
        let split_chars = line.chars().collect::<Vec<char>>();
        layout.push(split_chars);
    }

    while current_sum != 0 {
        current_sum = 0;
        for j in 0..layout.len() {
            for i in 0..layout[j].len() {
                current_sum += check_coordinates_part2(&mut layout,i,j);
            }
            print!("\n");
        }
        total_sum += current_sum;
        print!("\n");
    }



    println!("Sum: {}",total_sum);
    // println!("Hello, world!");
}

fn check_coordinates_part2(layout: &mut Vec<Vec<char>>, x: usize, y: usize) -> i32 {
    let mut all_surrounding_rolls = 0;
    let length_y = layout[0].len();
    let length_x = layout.len();

    if layout[y][x] != '@' {
        print!("{}", layout[y][x]);
        return 0;
    }

    for j in -1..=1 {
        for i in -1..=1 {
            if i == 0 && j == 0 {continue;}
            let adjacent_y = y as i32 - j;
            let adjacent_x = x as i32 - i;
            if adjacent_y < 0
                || adjacent_x < 0
                || adjacent_y >= length_y as i32
                || adjacent_x >= length_x as i32
            {
                continue;
            }

            if layout[adjacent_y as usize][adjacent_x as usize] == '@' {
                all_surrounding_rolls += 1;
            }
        }
    }

    if all_surrounding_rolls < 4 {
        layout[y][x] = '.';
        print!(".");

        return 1;
    }
    print!("{}",layout[y][x]);
    0

}