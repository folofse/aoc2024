pub fn part1(lines:Vec<String>)->i32 {
  let mut result_val:i32 = 0;
  let mut pos_rc = (0,0);  

  for (r, line) in lines.iter().enumerate() {
    for c in 0..line.len() {
      if line[c..].starts_with("X") {
        pos_rc = (r as i32, c as i32);
        result_val += word_hits(&lines, pos_rc) 
      }
    }
  }


  return result_val;
}
pub fn part2(lines:Vec<String>)->i32 {
  let mut result_val:i32 = 0;
  let mut pos_rc = (0,0);  

  for (r, line) in lines.iter().enumerate() {
    for c in 0..line.len() {
      if line[c..].starts_with("A") {
        pos_rc = (r as i32, c as i32);
        result_val += if x_hit(&lines, pos_rc) { 1 } else { 0 };
      }
    }
  }


  return result_val;
}
fn word_hits(lines:&Vec<String>, pos_rc:(i32, i32))->i32{
  let word = "XMAS";
  let mut possible_hits = vec![true, true, true, true, true, true, true, true]; 
  
  for i in 1..word.len() as i32 {
    let positions: Vec<(i32, i32)> = vec![
      (pos_rc.0-i, pos_rc.1-i), // Top left
      (pos_rc.0-i, pos_rc.1),   // Top middle
      (pos_rc.0-i, pos_rc.1+i), // Top right
      (pos_rc.0, pos_rc.1-i),   // Middle left
      (pos_rc.0, pos_rc.1+i),   // Middle right
      (pos_rc.0+i, pos_rc.1-i), // Bottom left
      (pos_rc.0+i, pos_rc.1),   // Bottom middle
      (pos_rc.0+i, pos_rc.1+i)  // Bottom right
    ];

    for (j, pos) in positions.iter().enumerate() { 
      if pos.0 < 0 || pos.1 < 0 || pos.0 >= lines.len() as i32 || pos.1 >= lines[0].len() as i32 {
        possible_hits[j] = false;
        continue;
      }
      let u_pos = (pos.0 as usize, pos.1 as usize);
      let search_char = &lines[u_pos.0][u_pos.1..u_pos.1 + 1];

      if possible_hits[j] && search_char == &word[i as usize..i as usize + 1] {
      let u_pos = (pos.0 as usize, pos.1 as usize);
        possible_hits[j] = true;
      }else{
        possible_hits[j] = false;
      }   
    }
  }

  return possible_hits.iter().filter(|&x| *x).count() as i32;
}

fn x_hit(lines:&Vec<String>, pos_rc:(i32, i32))->bool{
  let word = "MAS".to_owned();
  let top_left = (pos_rc.0 - 1, pos_rc.1 - 1);
  let top_right = (pos_rc.0 - 1, pos_rc.1 + 1);
  let mut top_left_word = "".to_owned();
  let mut top_right_word = "".to_owned();

  for i in 0..(word.len()) as i32 {
    if pos_rc.0 - 1 < 0 || pos_rc.0 + 1 >= lines.len() as i32 || 
       pos_rc.1 - 1 < 0 ||  pos_rc.1 + 1 >= lines[0].len() as i32 {
      return false;
    }    
    let pos_tl = ((top_left.0 + i) as usize, (top_left.1 + i) as usize);
    let pos_tr = ((top_right.0 + i) as usize, (top_right.1 - i) as usize);

    top_left_word.push_str(&lines[pos_tl.0][pos_tl.1..pos_tl.1 as usize + 1]);
    top_right_word.push_str(&lines[pos_tr.0][pos_tr.1..pos_tr.1 as usize + 1]);
  }

  if (top_left_word == word || top_left_word == word.chars().rev().collect::<String>()) &&
     (top_right_word == word || top_right_word == word.chars().rev().collect::<String>()) {
    return true;
  }else{
    return false;
  }
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

    assert_eq!(part1(lines), 18 );
  }
  

  #[test]
  fn test1_part2() {
    let lines = read_lines("./src/input.txt")
      .iter()
      .map(|f| f.parse().unwrap())
      .collect();

    assert_eq!(part1(lines), 2554);
  }

 
  #[test]
  fn test2_part1() {
    let lines = read_lines("./src/example.txt")
      .iter()
      .map(|f| f.parse().unwrap())
      .collect();

    assert_eq!(part2(lines), 9);
  }

  #[test]  
  fn test2_part2() {
    let lines = read_lines("./src/input.txt")
      .iter()
      .map(|f| f.parse().unwrap())
      .collect();

    assert_eq!(part2(lines), 1916);
  }
  
}