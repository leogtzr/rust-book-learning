enum Day {
    LUNES,
    MARTES,
    MIERCOLES,
    JUEVES,
    VIERNES,
    SABADO,
    DOMINGO
}

fn main() {
    let day: Option<Day> = Some(Day::DOMINGO);

    if let Some(Day::MIERCOLES) = day {
        println!("It is wednesday");
    }
}
