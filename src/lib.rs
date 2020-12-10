use rayon::prelude::*;
use scan_fmt::*;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs,
    io::*,
    iter::*,
    time::*,
};

pub mod collections;
pub mod console_vm;

use collections::bitarray::*;
use console_vm::{GameConsoleVM, Instruction, TerminationError};

pub fn execute_timed<F, Fout>(mut proc: F) -> (Fout, u128)
where
    F: FnMut() -> Fout,
{
    let t0 = Instant::now();
    let out = proc();
    let dt = t0.elapsed().as_millis();
    (out, dt)
}

pub fn aoc_1_0(target: i64) -> Option<i64> {
    let f = BufReader::new(fs::File::open("./in/input1.txt").unwrap());
    let mut val_table: HashSet<_> = HashSet::new();

    for line_res in f
        .lines()
        .map(|line_str| line_str.map(|str| str.parse::<i64>().unwrap_or_default()))
    {
        if let Ok(val) = line_res {
            val_table.insert(val);

            let x = target - val;
            let product = x * val;

            if let Some(_) = val_table.get(&x) {
                println!(
                    "{}+{}={} and multiplied together we get: {}",
                    val, x, target, product
                );
                return Some(product);
            }
        }
    }
    None
}

pub fn aoc_1_1(target: i64) -> Option<i64> {
    let f = BufReader::new(fs::File::open("./in/input1_big.txt").unwrap());

    let mut val_list = Vec::new();
    val_list.reserve(100_000);

    for line_res in f
        .lines()
        .map(|line_str| line_str.map(|str| str.parse::<i64>().unwrap_or_default()))
    {
        if let Ok(val) = line_res {
            val_list.push(val);
        }
    }
    val_list.par_sort_unstable();

    let len = val_list.len();
    for k in 0..len {
        let mut i = k + 1;
        let mut j = len - 1;
        while i < j {
            let (a, b, c) = unsafe {
                (
                    val_list.get_unchecked(i),
                    val_list.get_unchecked(j),
                    val_list.get_unchecked(k),
                )
            };
            let total = a + b + c;
            if total == target {
                println!("{}+{}+{}={}", a, b, c, total);
                return Some(a * b * c);
            } else if total > target {
                j -= 1;
            } else {
                i += 1;
            }
        }
    }
    None
    // println!("finished with nothing")
}

pub fn aoc_2_0(_in: ()) -> std::result::Result<(), std::io::Error> {
    let file = BufReader::new(fs::File::open("./in/input2.txt").unwrap());
    let mut table = HashMap::new();
    let mut valid_passwords = 0;
    let mut total_passwords = 0;

    let mut is_valid_password = |bounds: &Vec<i32>, target_char, password: &str| -> bool {
        let (min, max) = (bounds[0], bounds[1]);
        //clear table for re-use
        table.clear();

        for pchar in password.chars() {
            let count = if let Some(val) = table.get_mut(&pchar) {
                *val += 1;
                *val
            } else {
                table.insert(pchar, 1);
                1
            };
            if pchar == target_char && count > max {
                return false;
            }
        }

        let target_count = table.get(&target_char).map(|&a| a).unwrap_or(0);
        target_count >= min && target_count <= max
    };

    for line_res in file.lines() {
        line_res.map(|line| {
            total_passwords += 1;
            let fields: Vec<_> = line.split(' ').collect();
            let bounds: Vec<_> = fields[0]
                .split('-')
                .map(|str| str.parse::<i32>().unwrap())
                .collect();
            let target_char = fields[1].chars().next().unwrap();
            let password = fields[2];

            if is_valid_password(&bounds, target_char, password) {
                valid_passwords += 1;
                // println!(
                //     "VALID: bounds:[{},{}],target:{},pass:{}",
                //     bounds[0], bounds[1], target_char, password
                // );
            }
        })?;
    }

    println!(
        "valid passwords: {} out of  {}",
        valid_passwords, total_passwords
    );
    Ok(())
}

pub fn aoc_2_1(_in: ()) -> std::result::Result<(), std::io::Error> {
    let file = BufReader::new(fs::File::open("./in/input2_1.txt").unwrap());
    let mut valid_passwords = 0;
    let mut total_passwords = 0;
    for line_res in file.lines() {
        line_res.map(|line| {
            total_passwords += 1;
            let (lbound, ubound, target_char, password) =
                scan_fmt!(line.as_str(), "{}-{} {}: {}", usize, usize, char, String)
                    .ok()
                    .unwrap();
            let char_vec: Vec<_> = password.chars().collect();
            let first_pos_valid = char_vec[lbound - 1] == target_char;
            let second_pos_valid = char_vec[ubound - 1] == target_char;
            if first_pos_valid != second_pos_valid {
                valid_passwords += 1;
            }
        })?;
    }
    println!(
        "valid passwords: {} out of  {}",
        valid_passwords, total_passwords
    );
    Ok(())
}

pub fn aoc_3_0(_in: ()) {
    let file = BufReader::new(fs::File::open("./in/input3_0.txt").unwrap());
    let mut grid: Vec<Vec<char>> = Vec::new();
    for line_res in file.lines() {
        let _ = line_res.map(|line| {
            grid.push(line.chars().collect());
        });
    }
    let rows = grid.len();
    let cols = grid[0].len();
    let (mut pos_x, mut pos_y) = (0i32, 0i32);
    let mut total = 0;
    while pos_y < rows as i32 {
        let tile = grid[pos_y as usize][(pos_x as usize) % cols];
        if tile == '#' {
            total += 1;
        }
        pos_x += 3;
        pos_y += 1;
    }
    println!("trees encountered = {}", total);
}

pub fn aoc_3_1(_in: ()) {
    let file = BufReader::new(fs::File::open("./in/input3_0.txt").unwrap());
    let mut grid: Vec<Vec<char>> = Vec::new();

    for line_res in file.lines() {
        let _ = line_res.map(|line| {
            grid.push(line.chars().collect());
        });
    }

    let disp_list = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let (rows, cols) = (grid.len(), grid[0].len());
    let count_trees = |dx, dy| -> u32 {
        let (mut pos_x, mut pos_y) = (0i32, 0i32);
        let mut total = 0;
        while pos_y < rows as i32 {
            let tile = grid[pos_y as usize][(pos_x as usize) % cols];
            if tile == '#' {
                total += 1;
            }
            pos_x += dx;
            pos_y += dy;
        }
        total
    };
    let count_list: Vec<_> = disp_list
        .iter()
        .map(|&(dx, dy)| count_trees(dx, dy))
        .collect();
    let product: u32 = count_list.iter().product();
    println!("{:?}, product = {} ", count_list, product);
}

pub fn aoc_4_0(_in: ()) {
    let raw_text = fs::read_to_string("./in/input4_0.txt").unwrap();
    let mut record_list = Vec::new();
    let mut current_record = String::new();
    let mut main_ft = HashMap::new();

    for line in raw_text.lines() {
        if line.len() <= 1 {
            record_list.push(current_record.clone());
            current_record.clear();
        } else {
            if current_record.len() != 0 {
                current_record.push(' ');
            }
            current_record.push_str(line);
        }
    }
    if current_record.len() > 1 {
        record_list.push(current_record.clone());
        current_record.clear();
    }

    let valid_records = record_list
        .iter()
        .filter(|rec| is_valid_record(&mut main_ft, rec))
        .count();
    println!("valid records = {}", valid_records);
}

fn is_valid_record(field_table: &mut HashMap<String, String>, record: &String) -> bool {
    let mut cid_missing = true;
    let mut unique_fields = 0;

    let validation_table: [(&str, Box<dyn Fn(&&str) -> bool>); 7] = [
        (
            "byr",
            Box::new(|val: &&str| -> bool {
                if let Ok(num_val) = val.parse::<i32>() {
                    if !(num_val >= 1920 && num_val <= 2002) {
                        // println!("byr invalid");
                        return false;
                    }
                } else {
                    panic!("byr parse failed!");
                }
                true
            }),
        ),
        (
            "iyr",
            Box::new(|val: &&str| -> bool {
                if let Ok(num_val) = val.parse::<i32>() {
                    if !(num_val >= 2010 && num_val <= 2020) {
                        // println!("iyr invalid");
                        return false;
                    }
                } else {
                    panic!("iyr parse failed!");
                }
                true
            }),
        ),
        (
            "eyr",
            Box::new(|val: &&str| -> bool {
                if let Ok(num_val) = val.parse::<i32>() {
                    if !(num_val >= 2020 && num_val <= 2030) {
                        // println!("eyr invalid");
                        return false;
                    }
                } else {
                    panic!("eyr parse failed!");
                }
                true
            }),
        ),
        (
            "hgt",
            Box::new(|val: &&str| -> bool {
                let split_index_opt = val.find(char::is_alphabetic);
                if split_index_opt.is_none() {
                    // println!("hgt invalid");
                    return false;
                }
                let split_index = split_index_opt.unwrap();
                let num_slice = &val[0..split_index];
                if let Ok(num) = num_slice.parse::<i32>() {
                    let unit = &val[split_index..];
                    if unit == "cm" {
                        if !(num >= 150 && num <= 193) {
                            // println!("hgt invalid");
                            return false;
                        }
                    } else if unit == "in" {
                        if !(num >= 59 && num <= 76) {
                            // println!("hgt invalid");
                            return false;
                        }
                    } else {
                        // println!("hgt invalid");
                        return false;
                    }
                } else {
                    panic!("Faild to parse hgt: \"{}\"", val);
                }
                true
            }),
        ),
        (
            "hcl",
            Box::new(|val: &&str| -> bool {
                let has_hash = &val[0..1] == "#";
                let valid_remainder = val[1..]
                    .chars()
                    .all(|a| a.is_numeric() || (a.is_alphabetic() && a >= 'a' && a <= 'f'));
                if !(val.len() == 7 && has_hash && valid_remainder) {
                    // println!("hcl invalid");
                    return false;
                }
                true
            }),
        ),
        (
            "ecl",
            Box::new(|val: &&str| -> bool {
                let valid_color = ["amb", "blu", "gry", "grn", "hzl", "oth", "brn"].contains(val);
                if valid_color == false {
                    // println!("ecl invalid");
                    return false;
                }
                true
            }),
        ),
        (
            "pid",
            Box::new(|val: &&str| -> bool {
                let all_nums = val.chars().all(|a| a.is_numeric());
                if !(all_nums && val.len() == 9) {
                    // println!("pid invalid");
                    return false;
                }
                true
            }),
        ),
    ];

    field_table.clear();

    for field in record.split(' ') {
        let field_key_val: Vec<_> = field.split(':').collect();
        let (key, val) = (field_key_val[0], field_key_val[1]);
        if let None = field_table.get(&key.to_string()) {
            unique_fields += 1;
            field_table.insert(key.to_string(), val.to_string());
            if key == "cid" {
                cid_missing = false;
            }
        }
    }

    for (hdr, validate) in validation_table.iter() {
        if let Some(false) = field_table
            .get(&hdr.to_string())
            .map(|val| validate(&val.as_str()))
        {
            return false;
        }
    }

    (unique_fields == 8) || (unique_fields == 7 && cid_missing)
}

pub fn aoc_5_0(_in: ()) {
    let raw_text = fs::read_to_string("./in/input5.txt").unwrap();
    let eval_coordinate = |encode: &str, mut lb: i32, mut ub, lower| -> i32 {
        let len = encode.len();
        let mut index = 0;
        let mut chars = encode.chars();
        while index < len {
            let c = chars.next().unwrap();
            if index == len - 1 {
                return if c == lower { lb.min(ub) } else { lb.max(ub) };
            } else {
                let mid = (ub - lb) / 2 + lb;
                if c == lower {
                    ub = mid;
                } else {
                    lb = mid + 1;
                }
            }
            index += 1;
        }
        -1
    };
    let decode_coord = |encode: &str, w, h| -> (i32, i32) {
        let row_stream = &encode[0..7];
        let col_stream = &encode[7..];
        let row_coordinate = eval_coordinate(row_stream, 0i32, h - 1, 'F');
        let col_coordinate = eval_coordinate(col_stream, 0i32, w - 1, 'L');
        (row_coordinate, col_coordinate)
    };
    //part 1
    let filled_seats_list: Vec<_> = raw_text
        .lines()
        .map(|line| decode_coord(line, 8, 128))
        .map(|(row, col)| row * 8 + col)
        .collect();
    let max_seat_id = filled_seats_list.iter().max().unwrap();
    println!("max_seat id : {}", max_seat_id);
    //part 2
    let mut id_list: Vec<i32> = (0..8 * 128).collect();
    // remove seats listed in file
    for &index in filled_seats_list.iter() {
        id_list[index as usize] = i32::MAX;
    }
    //look for row with a single empty seat
    for i in 0..8 * 128 {
        if i > 0
            && i < 1023
            && id_list[i - 1] == i32::MAX
            && id_list[i + 1] == i32::MAX
            && id_list[i] != i32::MAX
        {
            println!("id:{}", id_list[i]);
        }
    }
}

pub fn aoc_6_0(_in: ()) {
    // let mut yes_table = HashMap::new();
    let raw_text = fs::read_to_string("./in/input6.txt").unwrap();
    let mut table = HashMap::new();
    let groups = raw_text.split("\n\n");

    let group_count = |group: &str, table: &mut HashMap<char, i32>| {
        let mut unique = 0;
        table.clear();
        for c in group.chars().filter(|a| a.is_alphabetic()) {
            match table.get_mut(&c) {
                None => {
                    unique += 1;
                    table.insert(c, 1);
                }
                Some(v) => {
                    *v += 1;
                }
            }
        }
        unique
    };

    let mut total_uniq: i32 = 0;
    for g in groups {
        let count = group_count(g, &mut table);
        println!("count: {}", count);
        total_uniq += count;
    }

    println!("total  = {}", total_uniq);
}

//guesses
//3171
pub fn aoc_6_1(_in: ()) {
    // let mut yes_table = HashMap::new();
    let raw_text = fs::read_to_string("./in/input6.txt").unwrap();
    let groups = raw_text.split("\n\n");
    let group_count = |group: &str| {
        let mut accum = !0;
        for person in group.split("\n") {
            // println!("{}", person);
            let bit_code: u32 = person
                .chars()
                .filter(|c| c.is_alphabetic())
                .map(|c| 1 << ((c as u8 - b'a') as u32))
                .sum();
            accum &= bit_code;
        }
        accum.count_ones()
    };
    let total_uniq: u32 = groups.map(|g| group_count(g)).sum();
    println!("total  = {}", total_uniq);
}

pub fn aoc_7_0(_in: ()) {
    let raw_text = fs::read_to_string("./in/input7bbo.txt").unwrap();
    let mut graph: HashMap<&str, Vec<(i32, &str)>> = HashMap::new();
    let mut verts = Vec::new();
    for (k, line) in raw_text.lines().enumerate() {
        let fields: Vec<_> = line.split("bags contain ").collect();
        let vertex = fields[0].trim();
        let rules = fields[1];
        verts.push(vertex);
        graph.insert(vertex, Vec::new());

        let edges = rules.split(",").map(|s| s.trim());

        for edge in edges.filter(|s| s.contains("no other bag") == false) {
            let edge_len = edge.len();
            let fs_loc = edge.find(' ').unwrap();
            let ss_loc = edge
                .chars()
                .rev()
                .enumerate()
                .filter(|&(_, c)| c == ' ')
                .map(|(i, _)| i)
                .next()
                .unwrap();
            let str_num = &edge[0..fs_loc];
            let str_col = &edge[fs_loc + 1..edge_len - ss_loc - 1];
            // println!("({},{})", str_num, str_col);

            let num = str_num.parse().unwrap();
            let vec = graph.get_mut(&vertex).unwrap();
            vec.push((num, str_col));
        }
    }
    println!("graph loaded...");

    let part_1 = verts
        .iter()
        .map(|&vert| can_fit_shiny_gold(&graph, vert))
        .filter(|&a| a)
        .count();
    println!("fitcount = {}", part_1);

    let part_2 = shiny_gold_contains(&mut graph, "shiny gold");
    println!("total_bag_count = {}", part_2);
}
fn shiny_gold_contains(graph: &HashMap<&str, Vec<(i32, &str)>>, start: &str) -> i32 {
    let mut count = 0;
    if let Some(edges) = graph.get(start) {
        for &(cap, color) in edges.iter() {
            for _ in 0..cap {
                count += shiny_gold_contains(graph, color) + 1;
            }
        }
    }
    return count;
}
fn can_fit_shiny_gold(graph: &HashMap<&str, Vec<(i32, &str)>>, start: &str) -> bool {
    if let Some(edges) = graph.get(start) {
        for &(_, color) in edges.iter() {
            if color == "shiny gold" || can_fit_shiny_gold(graph, color) {
                return true;
            }
        }
    }
    false
}

pub fn aoc_8_0(_in: ()) {
    let raw_text = fs::read_to_string("./in/input8.txt").unwrap();
    let mut vm = GameConsoleVM::new();
    vm.parse_text_code(raw_text);
    if let Err(TerminationError::CycleDetected) = vm.run() {
        println!("cycle detected! pc:{},acc:{} ", vm.pc, vm.acc);
    }
}
pub fn aoc_8_1(_in: ()) {
    let raw_text = fs::read_to_string("./in/input8.txt").unwrap();
    let mut vm = GameConsoleVM::new();
    let vm_ptr = &mut vm as *mut GameConsoleVM;
    vm.parse_text_code(raw_text);

    let mod_code = vm
        .code
        .iter_mut()
        .enumerate()
        .filter(|(_address, opcode)| opcode.is_jmp() || opcode.is_nop())
        .map(|(_addr, _opcode)| (_addr, _opcode));

    let vm_ref = unsafe { &mut *vm_ptr };
    for (int_addr, int) in mod_code {
        let original = *int;
        for k in 0..2 {
            let new_int = if k == 0 {
                int.into_jmp()
            } else {
                int.into_nop()
            };
            vm_ref.code[int_addr] = new_int;
            // println!("change dump:\n{}\nline:{}\n",&vm,int_addr);
            if vm_ref.run().is_ok() {
                println!("SUCCESS: acc:{} pc:{} ", vm.acc, vm.pc);
            }
        }
        vm_ref.code[int_addr] = original;
    }
}

//a stupid brute force solution
pub fn aoc_9_0(_in: ()) {
    let raw_text = fs::read_to_string("./in/input9.txt").unwrap();
    let nums: Vec<u64> = raw_text.lines().map(|a| a.parse().unwrap()).collect();
    let get_first_invalid = |mut nums: Vec<_>, preamble_size| -> Option<u64> {
        let mut preamble = Vec::new();
        for _ in 0..preamble_size {
            let val = nums.remove(0);
            preamble.push(val);
        }
        let is_valid = |preamble: &Vec<_>, val| {
            let len = preamble.len();
            for i in 0..len {
                for j in i+1..len {
                    if preamble[i] + preamble[j] == val {
                        return true;
                    }
                }
            }
            return false;
        };
        for &num in nums.iter() {
            if is_valid(&preamble, num) == false {
                return Some(num);
            } else {
                preamble.remove(0);
                preamble.push(num);
            }
        }
        None
    };
    let find_weakness = |nums: Vec<_>, part1| {
        for i in 0..nums.len() {
            for j in i + 1..nums.len() {
                let interval = &nums[i..=j];
                let sum: u64 = interval.iter().sum();
                if sum == part1 {
                    let part2: u64 =
                        interval.iter().min().unwrap() + interval.iter().max().unwrap();
                    return Some(part2);
                }
            }
        }
        None
    };
    let part1 = get_first_invalid(nums.clone(), 25).unwrap();
    let part2 = find_weakness(nums, part1).unwrap();
    println!("part 1 : {}", part1);
    println!("part 2 : {}", part2);
}
