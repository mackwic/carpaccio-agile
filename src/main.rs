
extern crate clap;
use clap::*;

fn compute_price(price: f32, quantity: u32) -> f32 {
    price * (quantity as f32)
}

fn has_discount(price: f32) -> f32 {
    if price >= 50_000.0 {
        return 0.15
    }

    if price >= 10_000.0 {
        return 0.1
    }

    if price >= 7_000.0 {
        return 0.07
    }

    if price >= 5_000.0 {
        return 0.05
    }

    if price >= 1000.0 {
        return 0.03
    }

    return 0.0
}

fn compute_discount(total: f32, discount: f32) -> f32 {
     total - total * discount
}

fn has_tax(country: &str) -> Option<f32> {
    match country {
        "UT" => Some(0.0685),
        "NV" => Some(0.08),
        "TX" => Some(0.0625),
        "AL" => Some(0.04),
        "CA" => Some(0.0825),
        _ => None
    }
}

#[test]
fn test_price_compute() {
    assert_eq!(120.0, compute_price(12 as f32, 10));
}

#[test]
fn test_discount() {
    assert_eq!(0.0, has_discount(100 as f32));
    assert_eq!(0.03, has_discount(1000 as f32));
    assert_eq!(0.03, has_discount(1500 as f32));
    assert_eq!(0.05, has_discount(5000 as f32));
    assert_eq!(0.05, has_discount(5003 as f32));
    assert_eq!(0.07, has_discount(7000 as f32));
    assert_eq!(0.07, has_discount(7001 as f32));
    assert_eq!(0.1, has_discount(10_000 as f32));
    assert_eq!(0.1, has_discount(11_000 as f32));
    assert_eq!(0.15, has_discount(50_000 as f32));
    assert_eq!(0.15, has_discount(55_000 as f32));
}

#[test]
fn test_taxes() {
    assert_eq!(None, has_tax("FR"));
    assert_eq!(Some(0.0685), has_tax("UT"));
    assert_eq!(has_tax("NV"), Some(0.08));
    assert_eq!(has_tax("TX"), Some(0.0625));
    assert_eq!(has_tax("AL"), Some(0.04));
    assert_eq!(has_tax("CA"), Some(0.0825));
}

fn main() {
    println!("Carpaccio v0.1");
    println!("***************");

    let matches = App::new("carpaccio").args_from_usage(
                              "<PRICE>     'Price of product'
                               <QUANTITY>  'Quantity of product'
                               <COUNTRY>   'Country of product'")
                          .get_matches();

    let price : f32 = matches.value_of("PRICE").unwrap().parse().expect("Please enter a correct price (an integer)");
    let quantity : u32 = matches.value_of("QUANTITY").unwrap().parse().expect("Please enter a correct quantity (an integer).");
    let country = matches.value_of("COUNTRY").unwrap();

    let tax = has_tax(country).expect("Unknown country, known countries are: UT, NV, TX, AL, CA");

    let total = compute_price(price, quantity);
    let discount = has_discount(total);
    let discounted_total = compute_discount(total, discount);
    let discounted_total_and_taxes = discounted_total + discounted_total * tax;

    println!("P x Q = {} x {} = {}", price, quantity, total);
    println!("total with discount = {}", discounted_total);
    println!("applicable tax (in {}) = {}", country, tax);
    println!("######");
    println!("total with discount and taxes = {}", discounted_total_and_taxes);
}


