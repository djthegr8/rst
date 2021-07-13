use std::collections::{HashMap, HashSet};
use std::iter::{FromIterator, Skip};

use crate::util::Categories::All;

/// Calculates the mean of the given `i32` vector
///
/// # Arguments
///
/// * `list`: The vector to calculate mean of.
///
/// returns: `f32` : mean of the vector values
///
/// # Examples
///
/// ```rust
/// let vector = vec![1, 2, 3, 4, 5]
/// println!(calc_mean(vector)) // prints 3
/// ```
fn calc_mean(list: &Vec<i32>) -> f32 {
    (list.iter().sum::<i32>() as f32) / (list.len() as f32)
}

fn calc_median(list: &Vec<i32>) -> f32 {
    let mut cp = list.clone();
    cp.sort();
    let len = cp.len();
    let mut retval = cp[(len + 1) / 2] as f32;
    if len % 2 == 0 {
        retval += cp[len / 2] as f32;
        retval /= 2.0;
    }
    retval
}

fn calc_mode(list: &Vec<i32>) -> i32 {
    let mut map: HashMap<&i32, i32> = HashMap::new();
    for i in list.iter() {
        let freq_option = map.get_mut(i);
        match freq_option {
            Some(k) => *k += 1,
            None => {
                map.insert(i, 0);
            }
        }
    }
    *map.into_iter().max_by_key(|x| x.1).unwrap().0
}

fn to_pig_latin(english_: &String) -> String {
    let mut english = english_.clone();
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let fl = english.chars().next().unwrap();
    if vowels.contains(&fl) {
        return english + "-hay";
    } else {
        english.remove(0);
        english.push_str(&*format!("-{}ay", fl));
        return english;
    }
}

#[derive(PartialEq)]
pub enum CmdType {
    Add,
    Remove,
    Get,
}

#[derive(Debug, Hash, Eq, PartialEq)]
pub enum Categories {
    Engineering,
    Sales,
    HR,
    All,
}

fn handle_add(employees: &mut HashMap<Categories, Vec<String>>, name: String, cat: Categories) {
    let mr = employees.get_mut(&cat).unwrap();
    mr.push(name);
}

fn handle_remove(employees: &mut HashMap<Categories, Vec<String>>, name: &String, cat: Categories) {
    let mr = employees.get_mut(&cat).unwrap();
    if let Some(pos) = mr.iter().position(|x| x == name) {
        mr.swap_remove(pos);
    }
}

fn handle_get(employees: &HashMap<Categories, Vec<String>>, cat: Categories) {
    match cat {
        Categories::All => {
            for (_, vector) in employees {
                for val in vector {
                    println!("{}", val);
                }
            }
        }
        _ => {
            println!("Employees in {:?}", cat);
            let fl = employees.iter().filter(|x| x.0 == &cat);
            for (_, vec) in fl {
                for val in vec {
                    println!("{}", val)
                }
            }
        }
    };
}

pub fn handle_command(
    employees: &mut HashMap<Categories, Vec<String>>,
    inp: &String,
    cmd: CmdType,
) {
    let mut skip_vec: Vec<&str> = inp.split(' ').skip(1).collect();
    let ct = skip_vec.last();
    let mut cat = Categories::Engineering;
    match ct.unwrap() {
        &"engineering" => {}
        &"sales" => cat = Categories::Sales,
        &"hr" => cat = Categories::HR,
        &"all" => {
            if CmdType::Get != cmd {
                println!("Invalid category! Try again");
                return;
            }
            handle_get(&employees, All);
            return;
        }
        &_ => {
            println!("Invalid category! Try again");
            return;
        }
    }
    skip_vec.pop();
    let as_str: String = skip_vec.join(" ");
    match cmd {
        CmdType::Add => handle_add(employees, as_str, cat),
        CmdType::Remove => handle_remove(employees, &as_str, cat),
        CmdType::Get => handle_get(employees, cat)
    };
}
