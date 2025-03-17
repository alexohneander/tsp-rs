use rand::rng;
use rand::seq::SliceRandom;

#[macro_use]
extern crate rocket;

fn calculate_distance(city1: &(f64, f64), city2: &(f64, f64)) -> f64 {
    let dx = city1.0 - city2.0;
    let dy = city1.1 - city2.1;
    (dx * dx + dy * dy).sqrt()
}

fn calculate_tour_length(tour: &[usize], cities: &[(f64, f64)]) -> f64 {
    let mut length = 0.0;
    for i in 0..tour.len() {
        let city1 = &cities[tour[i]];
        let city2 = &cities[tour[(i + 1) % tour.len()]];
        length += calculate_distance(city1, city2);
    }
    length
}

fn lin_kernighan(mut tour: Vec<usize>, cities: &[(f64, f64)]) -> Vec<usize> {
    // let rng = rng();
    let n = cities.len();
    let mut improved = true;

    while improved {
        improved = false;
        for i in 0..n {
            for j in (i + 2)..n {
                if i == 0 && j == n - 1 {
                    continue;
                }
                let mut new_tour = tour.clone();
                new_tour[i + 1..=j].reverse();
                if calculate_tour_length(&new_tour, cities) < calculate_tour_length(&tour, cities) {
                    tour = new_tour;
                    improved = true;
                }
            }
        }
    }
    tour
}

#[get("/")]
fn index() -> String {
    let cities = vec![(0.0, 0.0), (1.0, 5.0), (5.0, 2.0), (6.0, 7.0), (3.0, 4.0)];

    let mut tour: Vec<usize> = (0..cities.len()).collect();
    tour.shuffle(&mut rng());

    let best_tour = lin_kernighan(tour, &cities);

    // println!("Beste Tour: {:?}", best_tour);
    println!(
        "Länge der besten Tour: {}",
        calculate_tour_length(&best_tour, &cities)
    );

    let tour_str = format!("Beste Tour: {:?}", best_tour);
    tour_str
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
