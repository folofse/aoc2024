use regex::Regex;

pub fn part1(lines:Vec<String>)->i32 {
  let mut result_val:i32 = 0;
  let mut left: Vec<i32> = Vec::new();
  let mut right: Vec<i32> = Vec::new();
  let re = Regex::new(r"(\d+)   (\d+)").unwrap();

  for line in lines {
    if !line.is_empty() {
      let caps = re.captures(line.as_str()).unwrap();
      left.push(caps[1].parse::<i32>().unwrap());
      right.push(caps[2].parse::<i32>().unwrap());
    } 
  }

  left.sort();
  right.sort();

  for i in 0..left.len() {
    result_val += (left[i] - right[i]).abs();
  }

  return result_val
}

pub fn part2(lines:Vec<String>)->i32 {
  let mut result_val:i32 = 0;
  let mut left: Vec<i32> = Vec::new();
  let mut right: Vec<i32> = Vec::new();
  let re = Regex::new(r"(\d+)   (\d+)").unwrap();

  for line in lines {
    if !line.is_empty() {
      let caps = re.captures(line.as_str()).unwrap();
      left.push(caps[1].parse::<i32>().unwrap());
      right.push(caps[2].parse::<i32>().unwrap());
    } 
  }

  for lft in left {
    let times_in_list = right.iter().filter(|&n| *n == lft as i32).count();
    result_val += lft * times_in_list as i32;
  }

  return result_val;
}


#[cfg(test)]
mod tests {
  use crate::part1;
  use crate::part2;

  use shared::read_lines;

  #[test]
  fn test1_part1() {
    let lines = read_lines("./src/example.txt")
      .iter()
      .map(|f| f.parse().unwrap())
      .collect();

    assert_eq!(part1(lines), 11 );
  }

  #[test]
  fn test1_part2() {
    let lines = read_lines("./src/input.txt")
      .iter()
      .map(|f| f.parse().unwrap())
      .collect();

    assert_eq!(part1(lines), 2164381);
  }

  
  #[test]
  fn test2_part1() {
    let lines = read_lines("./src/example.txt")
      .iter()
      .map(|f| f.parse().unwrap())
      .collect();

    assert_eq!(part2(lines), 31);
  }

  #[test]
  fn test2_part2() {
    let lines = read_lines("./src/input.txt")
      .iter()
      .map(|f| f.parse().unwrap())
      .collect();

    assert_eq!(part2(lines), 20719933);
  }
}