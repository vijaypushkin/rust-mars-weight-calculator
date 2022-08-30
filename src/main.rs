fn main() {
    println!("Weight on Mars is {}kg", calculate_weight_on_mars(68.0));
}

fn calculate_weight_on_mars(weigth: f32) -> f32 {
    (weigth / 9.81) * 3.711
}
