// Stress at 39 today
use std::cmp;

fn main(){
    let heights = vec![1,8,6,2,5,4,8,3,7];
//    let expected_answer = 49;
    let result = max_area(&heights);
    println!("Calculated maximum volume = {}", result);
}



fn calculate_volume_at_point(index1: usize, index2: usize, height: &Vec<i32>) -> i32 {
    let height1 = height[index1];
    let height2 = height[index2];
    let distance = (index2 - index1) as i32;
    let lesser_height = cmp::min(height1, height2);
    return distance * lesser_height;
}

pub fn max_area(height: &Vec<i32>) -> i32 {
    let mut max_volume = 0; // volumes are always >= 0
    for i in 0..height.len(){
        for j in i..height.len(){
            let volume = calculate_volume_at_point(i, j, height);
            if volume > max_volume {
                max_volume = volume;
            }
        }
    }
    return max_volume;
}
