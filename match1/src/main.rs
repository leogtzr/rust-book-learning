enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("some custom code...");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}", state);
            25
        },
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Arizona,
    Arkansas,
    California,
    Colorado,
    Connecticut,
    Delaware,
    Florida,
    Georgia,
    Hawai,
    Idaho,
    Illinois,
    Indiana,
    Iowa,
    Kansas,
    Kentucky,
    Luisiana,
    Maine,
    Maryland,
    Massachusetts,
    Míchigan,
    Minnesota,
    Misisipi,
    Misuri,
    Montana,
    Nebraska,
    Nevada,
    NuevoHampshire,
    NuevaJersey,
    NuevoMexico,
    NuevaYork,
    CarolinaDelNorte,
    DakotaDelNorte,
    Ohio,
    Oklahoma,
    Oregón,
    Pensilvania,
    RhodeIsland,
    CarolinaDelSur,
    DakotaDelSur,
    Tennessee,
    Texas,
    Utah,
    Vermont,
    Virginia,
    Washington,
    VirginiaOccidental,
    Wisconsin,
    Wyoming,
}

fn main() {
    let coin1 = Coin::Dime;

    let cents = value_in_cents(coin1);

    println!("Value = {}", cents);

    let alaska = value_in_cents(Coin::Quarter(UsState::Alaska));
    println!("Alaska = {:?}", alaska);
}
