use std::io;
use std::collections::HashMap;

fn average(nums :&Vec<i32>) -> f32{
    nums.iter().sum::<i32>() as f32 / nums.len() as f32
}

fn median(nums: &Vec<i32>) -> f32{
    if nums.is_empty() {
        return 0.0;
    }

    let mut sorted_nums = nums.clone();
    sorted_nums.sort();

    let mid = sorted_nums.len() / 2;

    if sorted_nums.len() % 2 == 0 {
        average(&sorted_nums[mid-1 .. mid+1].to_vec()) as f32
    }
    else{
        sorted_nums[mid] as f32
    }
}

fn mode_method1(nums :&Vec<i32>) -> Vec<i32>{
    let mut map = HashMap::new();
    for item in nums{
        let count = map.entry(item).or_insert(0);
        *count += 1;
    }

    let max = map.values().cloned().max().unwrap_or(0);
    
    let mode_result :Vec<i32> = map.into_iter()
                            .filter(|&(_,v)| v == max)
                            .map(|(&k, _)| k)
                            .collect();

    mode_result
}

fn mode_method2(nums :&Vec<i32>) -> String{
    let nums_count = nums.len();
    let mode_result = mode_method1(nums);
    if mode_result.len() < nums_count {
        format!("{:?}", mode_result)
    }
    else{
        String::from("None")
    }
}

fn main() {
    println!("Enter the numbers separated by comma");
    let mut comma_sep_nums = String::new();
    io::stdin().read_line(&mut comma_sep_nums)
        .expect("Failed to read input");
    println!("You entered {}",comma_sep_nums);

    //Approach 1 to get Vec<i32> for input_nums
    let mut input_nums = Vec::new(); 
    for val in comma_sep_nums.split(","){
        match val.trim().parse::<i32>(){
            Ok(v) => input_nums.push(v),
            Err(_) => (),
        }
    }
    println!("Approach 1 input_nums = {:?}", input_nums);

    //Approach 2 to get Vec<i32> for input_nums
    let input_nums: Vec<i32> = comma_sep_nums.split(",")
    //.map(|s| s.trim().parse::<i32>().expect("Not a integer"))
    .map(|s| s.trim().parse().unwrap())
    .collect::<Vec<i32>>();
    println!("Approach 2 input_nums = {:?}", input_nums);

    let avg = average(&input_nums);
    println!("Average = {}", avg);

    let med = median(&input_nums);
    println!("Median = {}", med);

    let mode = mode_method1(&input_nums);
    println!("Mode (Method 1) = {:?}", mode);
    let mode = mode_method2(&input_nums);
    println!("Mode (Method 2) = {}", mode);
}
