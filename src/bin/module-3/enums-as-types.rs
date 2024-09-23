// Create beer types enum
#[derive(Debug)]
enum BeerTypes {
    Lager,
    PaleAle,
    Stout,
    Pilsner,
}

// create beer struct
#[derive(Debug)]
struct Beer {
    name: String,
    brew_type: BeerTypes,
}

// create function supported_types that takes in a beer type and matches against all beer types enum options
fn is_supported_type(t: &BeerTypes) -> bool {
    match t {
        BeerTypes::Pilsner => true,
        BeerTypes::PaleAle => true,
        BeerTypes::Stout => true,
        _ => false,
    }
}

// function that returns string representation of the popularity of input style of beer
fn beer_type_popularity(t: &BeerTypes) -> String {
    match t {
        BeerTypes::Lager => String::from("Most popular"),
        BeerTypes::PaleAle => String::from("A little bitter"),
        BeerTypes::Pilsner => String::from("Classic"),
        BeerTypes::Stout => String::from("Like the name"),
        _ => String::from("We're not familiar with that one, sorry"),
    }
}
// fn main
// create two beer struct vars
fn main() {
    let beer1 = Beer {
        name: String::from("stout"),
        brew_type: BeerTypes::Stout,
    };

    let beer2 = Beer {
        name: String::from("classic pilsner"),
        brew_type: BeerTypes::Pilsner,
    };

    let beer1_popularity = beer_type_popularity(&beer1.brew_type);
    let beer2_popularity = beer_type_popularity(&beer2.brew_type);

    let beer1_supported = is_supported_type(&beer1.brew_type);
    let beer2_supported = is_supported_type(&beer2.brew_type);

    println!("Beer 1: {:?}, {}, {}", beer1, beer1_popularity, beer1_supported);
    println!("Beer 2: {:?}, {}, {}", beer2, beer2_popularity, beer2_supported);
}

// call supported types on beers
