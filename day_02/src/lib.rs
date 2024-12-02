#[derive(PartialEq)]
enum DIRECTION {
  NONE,
  INCREASING,
  DECREASING
}

pub fn part1(lines:Vec<String>)->i32 {
  let mut result_val:i32 = 0;

  for line in lines {
    if !line.is_empty() {
      let values = line.split_whitespace().map(|f| f.parse::<i32>().unwrap()).collect::<Vec<i32>>();
      let mut safe = true;
      
      if !direction_is_safe(&values) {
        safe = false;
      }
      if !diff_is_safe(&values) {
        safe = false;
      }
     
      if safe {
        result_val += 1;
      }
    } 
  }

  return result_val;
}

pub fn part2(lines:Vec<String>)->i32 {
  let mut result_val:i32 = 0;

  let mut result_val:i32 = 0;

  for line in lines {
    if !line.is_empty() {
      let values = line.split_whitespace().map(|f| f.parse::<i32>().unwrap()).collect::<Vec<i32>>();
      let mut safe = true;
      
      if !direction_is_safe(&values) {
        safe = false;
      }
      if !diff_is_safe(&values) {
        safe = false;
      }
      
      if !safe {
        for i in 0..values.len() {
          safe = true;

          // Get all values except the current index
          let mut new_values = values.clone();
          new_values.remove(i);

          if !direction_is_safe(&new_values) {
            safe = false;
          }
          if !diff_is_safe(&new_values) {
            safe = false;
          }
          if safe {
            break;
          }
        }
      }
      
      if safe {
        result_val += 1;
      }
    } 
  }

  return result_val
}

fn diff_is_safe(values:&Vec<i32>)->bool{
  let mut parent_value:i32 = 0;
  for(i , v) in values.iter().enumerate() {
    if i == 0 {
      parent_value = *v;
      continue;
    }
    let diff = v - &parent_value;
    if diff.abs() < 1 || diff.abs() > 3 {
      return false;
    }
    parent_value = *v;
  }
  return true;
}

fn direction_is_safe(values:&Vec<i32>)->bool {
  let mut direction = DIRECTION::NONE;
  let mut parent_value:i32 = 0;
  for(i , v) in values.iter().enumerate() {
    if i == 0 {
      parent_value = *v;
      continue;
    }

    if v > &parent_value {
      if direction == DIRECTION::DECREASING {
        return false;
      }
      direction = DIRECTION::INCREASING;
    } else {
      if direction == DIRECTION::INCREASING {
        return false;
      }
      direction = DIRECTION::DECREASING;
    } 
    parent_value = *v;
  }
  return true;
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

    assert_eq!(part1(lines), 2 );
  }
  
  #[test]
  fn test1_part2() {
    let lines = read_lines("./src/input.txt")
      .iter()
      .map(|f| f.parse().unwrap())
      .collect();

    assert_eq!(part1(lines), 490);
  }

 
  #[test]
  fn test2_part1() {
    let lines = read_lines("./src/example.txt")
      .iter()
      .map(|f| f.parse().unwrap())
      .collect();

    assert_eq!(part2(lines), 4);
  }
 
  #[test]  
  fn test2_part2() {
    let lines = read_lines("./src/input.txt")
      .iter()
      .map(|f| f.parse().unwrap())
      .collect();

    assert_eq!(part2(lines), 536);
  }
  
  
}