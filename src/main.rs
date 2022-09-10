use std::io;

fn main() {
    let mut input = String::new();

    println!("Enter your weight in kg: ");

    io::stdin().read_line(&mut input).unwrap();

    let weight: f32 = input.trim().parse().unwrap();

    println!("Weight on Mars is {}kg", calculate_weight_on_mars(weight));
}

fn calculate_weight_on_mars(weigth: f32) -> f32 {
    (weigth / 9.81) * 3.711
}

// write tests here
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_weight_on_mars() {
        assert_eq!(calculate_weight_on_mars(50.0), 18.914373);
    }
}
