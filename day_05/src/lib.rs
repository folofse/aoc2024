use std::cmp::Ordering;

pub fn part1(lines:Vec<String>)->i32 {
  return run_sort(lines, true);
}

pub fn part2(lines:Vec<String>)->i32 {
  return run_sort(lines, false);
}

fn run_sort(lines:Vec<String>, part1:bool )->i32{
  let mut result_val:i32 = 0;
  let rules = get_rules(&lines);
  let pages = get_pages(&lines);
  
  result_val = pages.into_iter().fold(0, |mut acc, mut row| {

    let order_string = row.iter().map(|d| d.to_string()).collect::<Vec<String>>().join("|");
    row.sort_by(|a, b| {
      if rules.iter().position(|r| r.to_owned() == format!("{}|{}", a, b)) != None{
        return Ordering::Less;
      } else if rules.iter().position(|r| r.to_owned() == format!("{}|{}", b, a)) != None{
        return Ordering::Greater;
      }else{
        return Ordering::Equal;
      }
    });
    let new_order_string = row.iter().map(|d| d.to_string()).collect::<Vec<String>>().join("|");

    if part1 {
      if order_string == new_order_string {
        acc += row[row.len() / 2 as usize];
      }
    }else{
      if order_string != new_order_string {
        acc += row[row.len() / 2 as usize];
      }
    }
   
    return acc;
  });
  return result_val;
}
fn get_rules(lines:&Vec<String>)->Vec<String>{
  return lines.iter().filter(|line| line.contains("|")).map(|line| line.to_owned()).collect::<Vec<String>>();
}
fn get_pages(lines:&Vec<String>)->Vec<Vec<i32>>{
  return lines.iter().filter(|line| line.contains(",")).map(|line| {
    return line.split(",").map(|d| d.parse::<i32>().unwrap()).collect::<Vec<i32>>()
  }).collect::<Vec<Vec<i32>>>();
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

    assert_eq!(part1(lines), 143 );
  }
  
  #[test]
  fn test1_part2() {
    let lines = read_lines("./src/input.txt")
      .iter()
      .map(|f| f.parse().unwrap())
      .collect();

    assert_eq!(part1(lines), 4281);
  }

  #[test]
  fn test2_part1() {
    let lines = read_lines("./src/example.txt")
      .iter()
      .map(|f| f.parse().unwrap())
      .collect();

    assert_eq!(part2(lines), 123);
  }

  #[test]  
  fn test2_part2() {
    let lines = read_lines("./src/input.txt")
      .iter()
      .map(|f| f.parse().unwrap())
      .collect();

    assert_eq!(part2(lines), 5466);
  }
}