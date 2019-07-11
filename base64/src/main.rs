fn base64_enc(input: &String) -> String {
  let lookup = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/=";

  let mut output = Vec::new();
  let mut mask = 0b1111_1100;
  let mut overflow = 0;
  let mut overflow_bits = 0;
  for chr in input.chars() {
    overflow_bits += 2;

    if overflow_bits == 8 {
      output.push(overflow);
      overflow = 0;
      overflow_bits = 2;
      mask = 0b1111_1100;
    }
    
    let octet = chr as u8;
    let mut sextet = 0;
    if overflow > 0 {
      sextet = overflow << (6 - (overflow_bits - 2));
    }
    if overflow_bits < 8 {
      sextet |= (octet & mask) >> overflow_bits;
    }
    overflow = octet & !mask;
    output.push(sextet);

    mask <<= 2;

    if mask == 0 {
      mask = 0b1111_1100;
    }
  }

  if overflow > 0 {
    output.push(overflow << (6 - overflow_bits));
  }

  match 6 - overflow_bits {
    2 => output.push(lookup.len() as u8 - 1),
    4 => {
      output.push(lookup.len() as u8 - 1);
      output.push(lookup.len() as u8 - 1);
    },
    6 => (),
    0 => (),
    _ => panic!("Unexpected remaining overflow")
  }

  let mut result = String::new();
  for idx in output {
    result.push(lookup.chars().nth(idx as usize).unwrap());
  }

  return result;
}

fn main() {
  let input1 = String::from("any carnal pleasure.");
  let result1 = base64_enc(&input1);
  println!("{} = {}", input1, result1);

  let input2 = String::from("any carnal pleasure");
  let result2 = base64_enc(&input2);
  println!("{} = {}", input2, result2);

  let input3 = String::from("any carnal pleasur");
  let result3 = base64_enc(&input3);
  println!("{} = {}", input3, result3);

  let input4 = String::from("any carnal pleasu");
  let result4 = base64_enc(&input4);
  println!("{} = {}", input4, result4);
}
