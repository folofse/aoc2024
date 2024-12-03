use regex::Regex;

pub fn part1(lines:Vec<String>)->i32 {
  let mut result_val:i32 = 0;
  let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

  for line in lines {
    if !line.is_empty() {
      for caps in re.captures_iter(line.as_str()) {
        let val1 = &caps[1].parse::<i32>().unwrap();
        let val2 = &caps[2].parse::<i32>().unwrap();
        result_val += val1 * val2;
      }
    } 
  }

  return result_val;
}

pub fn part2(lines:Vec<String>)->i32 {
  let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
  let mut result_val: i32 = 0;
  let mut next_instruction = "do()";

  for line in &lines {
    if !line.is_empty() {
      let res = split_keep(line);
      for row in res {

        if row.contains("do()") || row.contains("don't()") {
          next_instruction = row;
          continue;
        }
        
        if next_instruction == "do()"{   
          for cap in re.captures_iter(row) {
            let val1 = &cap[1].parse::<i32>().unwrap();
            let val2 = &cap[2].parse::<i32>().unwrap();

            result_val += val1 * val2;
          }
        }
      }
    }
  }

  return result_val;
}

fn split_keep<'a>(line: &'a str) -> Vec<&'a str> {
  let mut last = 0;
  let mut res: Vec<&str> = Vec::new();
  let re_row = Regex::new(r"(?:do\(\)|don't\(\))").unwrap();

  for mat in re_row.find_iter(&line) {
    if last != mat.start() {
      res.push(&line[last..mat.start()]);
    }
    res.push(&line[mat.start()..mat.end()]);
    last = mat.start() + mat.len();
  }
  if last < line.len() {
    res.push(&line[last..line.len()]);
  }

  return res;
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

    assert_eq!(part1(lines), 161 );
  }
  
  #[test]
  fn test1_part2() {
    let lines = read_lines("./src/input.txt")
      .iter()
      .map(|f| f.parse().unwrap())
      .collect();

    assert_eq!(part1(lines), 161289189);
  }

 
  #[test]
  fn test2_part1() {
    let lines = read_lines("./src/example2.txt")
      .iter()
      .map(|f| f.parse().unwrap())
      .collect();

    assert_eq!(part2(lines), 48);
  }

  #[test]  
  fn test2_part2() {
    let lines = read_lines("./src/input.txt")
      .iter()
      .map(|f| f.parse().unwrap())
      .collect();

    assert_eq!(part2(lines), 83595109);
  }
  
}