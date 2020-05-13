pub fn is_armstrong_number(num: u32) -> bool {
    let mut sum = 0;
    let nString = num.to_string();
    let length = nString.len() as u32;
    for d in nString.chars() {
       sum += d.to_digit(10)
               .unwrap()
               .pow(length);
    }
    sum == num
}

