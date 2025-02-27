pub(crate) fn get_line_similarity(line_one: &Vec<(f64, f64)>, line_two: &Vec<(f64, f64)>) {
    let mut total = 0.0;
    let length = line_one.clone().len() as f64;
    for i in 0..line_one.len() {
        total += (line_one[i].1 - line_two[i].1).abs()
    }
    println!("Avg Distance of points: {}", total / length);
}
