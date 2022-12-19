use std::collections::{HashSet, HashMap, VecDeque, hash_map::Entry};

fn main() {
    let time = std::time::Instant::now();
    // Day 1
    let mut meals: Vec<u32> = include_str!("1.txt")
    .split("\n\n")
    .map(|x| x.lines().map(|x| x.parse::<u32>().unwrap()).collect())
    .map(|x: Vec<u32>| x.iter().sum::<u32>())
    .collect::<Vec<u32>>();
    meals.sort();
    let d1 = (meals.last().unwrap(),  meals[meals.len() - 3..].iter().sum::<u32>());
    // Day 2
    let scores = &[0, 6, 3, 0, 6];
    let mut d2 = (0, 0);
    include_str!("2.txt").lines().map(|l| ((l.chars().nth(0).unwrap() as i32), (l.chars().nth(2).unwrap()) as i32))
    .for_each(|(elf, me)| {
        d2.0 += me - 'X' as i32 + 1 + scores[((elf - 'A' as i32 - (me - 'X' as i32)) + 2) as usize];
        d2.1 += (me - 'X' as i32) * 3 + ((elf - 'A' as i32 + (me - 'X' as i32) + 2) % 3) + 1; });
    // Day 3
    let as_num = |c: char| {
        if c.is_lowercase() { return c as u32 - 96; }
            c as u32 - 38
    };
    let input = include_str!("3.txt");
    let d3_p1 = input.lines()
    .map(|s| s.split_at(s.len()/2))
    .map(|(l, r)| as_num(l.chars().find(|c| r.contains(*c)).unwrap())).sum::<u32>();
    let d3_p2= input.lines().collect::<Vec<&str>>().chunks(3)
    .map(|x| as_num(x[0].chars().find(|c| x[1].contains(*c) && x[2].contains(*c)).unwrap())).sum::<u32>();
    let d3 = (d3_p1, d3_p2);
    // Day 4
    let vec_map = |t: Vec<&str>| t.iter().map(|s| s.parse::<u8>().unwrap()).collect();
    let input: Vec<Vec<u8>> = include_str!("4.txt").lines()
    .map(|l| l.split(',')).map(|mut s| (s.next().unwrap().split('-'), s.next().unwrap().split('-')))
    .map(|(e1, e2)| vec_map(e1.chain(e2).collect())).collect();
    let d4= (input.iter().filter(|nums| (nums[0] >= nums[2] && nums[1] <= nums[3]) || (nums[2] >= nums[0] && nums[3] <= nums[1])).count(),
    input.iter().filter(|nums| nums[0] <= nums[3] && nums[2] <= nums[1]).count());
    // Day 5
    // TODO make functional solution for this one.
    // Day 6
    let closure = |s| { include_str!("6.txt").as_bytes().windows(s).enumerate()
    .find_map(|(i, x)| { if x.to_vec().into_iter().collect::<HashSet<u8>>().len() == s { Some(i + s) } else { None }})};
    let d6 = (closure(4).unwrap(), closure(14).unwrap());
    // Day 7
    const DISC_SIZE: u64 = 70000000;
    const REQ_UNUSED: u64 = 30000000;
    let insert = |dirs: &VecDeque<String>, map: &mut HashMap<String, u64>, num: u64| {
        let key = dirs.iter().fold(String::new(), |acc, dir| acc + "/" + dir);
        if let Entry::Vacant(entry) = map.entry(key.clone()) {
            entry.insert(num);
        } else {
            *map.get_mut(&key).unwrap() += num;
        }
    };
    let mut file_map: HashMap<String, u64> = std::collections::HashMap::new();
    let mut dirs: VecDeque<String> = VecDeque::new();
    include_str!("7.txt").lines().for_each(|line| {
        let parts = line.split(" ").collect::<Vec<&str>>();
        if parts[1] == "cd" {
            if parts[2] == ".." { dirs.pop_back(); } else { dirs.push_back(parts[2].to_string()); }
        }
        if let Ok(num) = parts[0].parse::<u64>() {
            let mut temp_dirs = dirs.clone();
            insert(&temp_dirs, &mut file_map, num);
            (0..temp_dirs.len()).for_each(|_| {
                temp_dirs.pop_back();
                if !temp_dirs.is_empty() { insert(&temp_dirs, &mut file_map, num) }});
            }
    });
    let d7 = (file_map.iter().filter_map(|(_, v)| if *v < 100000 { Some(v) } else { None }).sum::<u64>(),
    file_map.values().filter(|v| *v >= &(REQ_UNUSED - (DISC_SIZE - file_map.get("//").unwrap()))).min().unwrap());
    // Day 8
    fn distance<'a, I>(tree: u8, direction: I) -> u32 where I: Iterator<Item = &'a u8>, {
        let mut distance = 0;
        for other_tree in direction {
            distance += 1;
            if *other_tree >= tree { break; }
        }
        distance
    }
    let is_seeable = |x: usize, y: usize, area: &Vec<Vec<u8>>| {
        !area[y][..x].iter().any(|other_tree| *other_tree >= area[y][x])
        || !area[y][x + 1..].iter().any(|other_tree| *other_tree >= area[y][x])
        || !area[..y].iter().any(|r| r[x] >= area[y][x]) || !area[y + 1..].iter().any(|r| r[x] >= area[y][x])
        || y == area.len() - 1 || x == area[0].len() - 1 || x == 0 || y == 0
    };

    let tree_map: Vec<Vec<u8>> = include_str!("8.txt").lines().filter(|l| !l.is_empty())
    .map(|s| s.chars().map(|c| c as u8 - 0x30).collect::<Vec<u8>>()).collect();
    let mut trees: Vec<Vec<bool>> = vec![vec![false; tree_map[0].len()]; tree_map.len()];
    for (x, row) in tree_map.iter().enumerate() {
        for (y, _) in row.iter().enumerate() {
            trees[y][x] = is_seeable(x, y, &tree_map);
        }
    }

    let mut scenery = 0;
    for (i, row) in tree_map.iter().enumerate() {
        for (j, _) in row.iter().enumerate() {
            let view =
            distance(tree_map[i][j], tree_map[..i].iter().map(|x| x[j]).collect::<Vec<u8>>().iter().rev()) * // up
            distance(tree_map[i][j], tree_map[i+1..].iter().map(|x| x[j]).collect::<Vec<u8>>().iter()) * // down
            distance(tree_map[i][j], tree_map[i][..j].iter().rev()) * // left
            distance(tree_map[i][j],tree_map[i][j+1..].iter()); // right
            if view > scenery { scenery = view; }
        }
    }
    let d8 = (trees.iter().map(|r| r.iter().filter(|t| **t).count()).sum::<usize>(), scenery);
    // Day 9
    let input: Vec<(&str, i16)> = include_str!("9.txt").lines().map(|l| { let mut l = l.split_ascii_whitespace();
    (l.next().unwrap(), l.next().unwrap().parse::<i16>().unwrap())}).collect();
    let mut d9 = (HashSet::<(i16, i16)>::new(), HashSet::<(i16, i16)>::new());
    let (mut head_tail, mut rope) = (vec![(0, 0); 2], vec![(0,0); 10]);

    input.into_iter().for_each(|(dir, a)| match dir {
        "U" => {
            rope_move(&mut head_tail, (0, 1), a, &mut d9.0);
            rope_move(&mut rope, (0, 1), a, &mut d9.1);
        }
        "D" => {
            rope_move(&mut head_tail, (0, -1), a, &mut d9.0);
            rope_move(&mut rope, (0, -1), a, &mut d9.1);
        }
        "L" => {
            rope_move(&mut head_tail, (-1, 0), a, &mut d9.0);
            rope_move(&mut rope, (-1, 0), a, &mut d9.1);
        }
        "R" => {
            rope_move(&mut head_tail, (1, 0), a, &mut d9.0);
            rope_move(&mut rope, (1, 0), a, &mut d9.1);
        }
        _ => panic!("Invalid direction"),
    });
    fn rope_move(rope: &mut Vec<(i16, i16)>, step: (i16, i16), val: i16, visited: &mut HashSet<(i16, i16)>) {
        for _ in 0..val {
        rope[0].0 += step.0;
        rope[0].1 += step.1;
            for i in 1..rope.len() {
                let (head, tail) = (rope[i - 1], rope[i]);
                if (head.0 - tail.0).abs() > 1 || (head.1 - tail.1).abs() > 1 {
                    rope[i].0 += if head.0 == tail.0 { 0 } else { (head.0 - tail.0) / (head.0 - tail.0).abs() };
                    rope[i].1 += if head.1 == tail.1 { 0 } else { (head.1 - tail.1) / (head.1 - tail.1).abs() };
                }
            }
            visited.insert(*rope.last().unwrap());
        }
    }
    let d9 = (d9.0.len(), d9.1.len());
    // Day 10
    let sprite = |reg: i64| { let reg: u64 = if reg <= 0 { (7+ reg*2) as u64 } else { 7_u64 << (reg - 1) };
    format!("{reg:064b}").chars().rev().collect::<String>().replace('1', "#").replace('0', ".")
    .split_at(40).0.to_owned() };
    let cycles = [20, 60, 100, 140, 180, 220];
    let mut crt = String::with_capacity(40 * 6 + 6);
    let (mut sig_sum, mut cycle, mut reg) = (0, 0, 1);
    let mut cycle_check = |c, r| {
        if cycles.contains(&c) { sig_sum += c * r; }
        if c as usize % 40 == 1 { crt.push('\n'); }
        crt.push(sprite(r).chars().nth((c as usize - 1) % 40).unwrap());};

    include_str!("10.txt")
        .lines()
        .for_each(|l| {
            cycle += 1;
            if l.starts_with("addx") {
                let x: i64 = l.split_once(' ').unwrap().1.parse().unwrap();
                cycle_check(cycle, reg);
                cycle +=1;
                cycle_check(cycle, reg);
                reg += x;
            } else { cycle_check(cycle, reg) }
        });
    let d10 = (sig_sum, crt);
    // Day 11
    // TODO results were too slow and too verbose
    // Day 12
    let input = include_str!("12.txt").lines().map(|l| l.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
    let mut start = (0, 0);
    let mut end = (0, 0);
    for (y, row) in input.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c == 'E' { end = (x, y); }
            if *c == 'S' { start = (x, y); }
        }
    }
    let mut low_points = Vec::new();
    let map: Vec<Vec<usize>> = input.iter().enumerate().map(|(y, v)| { v.iter().enumerate().map(|(x, c)| {
        if *c == 'a' { low_points.push((x, y)); }
        if *c == 'E' {
            25
        } else if *c == 'S' {
            low_points.push((x, y));
            0
        } else {
            *c as usize - 97
        }
    }).collect::<Vec<_>>()}).collect();
    let d12 = (path_find(&map.clone(), start, end),
        low_points.into_iter().fold(usize::MAX, |m, s| {
        let l = path_find(&map, s, end);
        if l < m { l } else { m }
    }));

    fn adjacent((x, y): (usize, usize), max_x: usize, max_y: usize) -> Vec<(usize, usize)> {
        let mut result = Vec::new();
        if x > 0 { result.push((x - 1, y)); }
        if x < max_x - 1 { result.push((x + 1, y)); }
        if y > 0 { result.push((x, y - 1)); }
        if y < max_y - 1 { result.push((x, y + 1)); }
        result
    }

    fn path_find(map: &Vec<Vec<usize>>, start: (usize, usize), end: (usize, usize)) -> usize {
        let mut queue = std::collections::VecDeque::new();
        let mut visited = HashSet::<(usize, usize)>::new();
        queue.push_back((start, 0));
        visited.insert(start);
        while let Some((pos, round)) = queue.pop_front() {
            if pos == end { return round; }
            for adj in adjacent(pos, map[0].len(), map.len()) {
                if !visited.contains(&adj) && map[adj.1][adj.0] <= map[pos.1][pos.0] + 1 {
                    visited.insert(adj);
                    queue.push_back((adj, round + 1));
                }
            }
        }
        usize::MAX
    }

    // Results
    println!("Days:\n1: {:?}\n2: {:?}\n3: {:?}\n4: {:?}\n5: {:?}\n6: {:?}\n7: {:?}\n8: {:?}\n9: {:?}\n10: {}{}
    \r11: {:?}\n12: {:?}\n13: {:?}\n14: {:?}\n15: {:?}\n16: {:?}\n17: {:?}\n18: {:?}\n19: {:?}\n20: {:?}
    \r21: {:?}\n22: {:?}\n23: {:?}\n24: {:?}\n25: {:?}\nIn: {}ms",
    d1, d2, d3, d4, 0, d6, d7, d8, d9, d10.0, d10.1, 0, d12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, time.elapsed().as_millis() );
}
