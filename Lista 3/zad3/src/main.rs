use std::cmp::Reverse;

#[derive(Debug, Clone)]
struct Osoba{
    imie: String,
    nazwisko: String,
    wzrost: i16,
    waga: i8,
    data_urodzenia: String,
}

impl Osoba{
    fn nowa_osoba(imie: String, nazwisko: String, wzrost: i16, waga: i8, data_urodzenia: String) -> Osoba{
        Osoba{
            imie,
            nazwisko,
            wzrost,
            waga,
            data_urodzenia,
        }
    }
}

fn example_osobas() -> Vec<Osoba>{
    vec![Osoba::nowa_osoba("Jan".to_string(), "Kowalski".to_string(), 180, 70, "1999-12-28".to_string()),
        Osoba::nowa_osoba("Magda".to_string(), "Nowicka".to_string(), 164, 53, "2000-01-01".to_string()),
        Osoba::nowa_osoba("Anna".to_string(), "Wiśniewska".to_string(), 170, 65, "1995-05-15".to_string()),
        Osoba::nowa_osoba("Piotr".to_string(), "Lewandowski".to_string(), 178, 80, "1988-07-22".to_string()),
        Osoba::nowa_osoba("Katarzyna".to_string(), "Dąbrowska".to_string(), 165, 60, "1992-03-10".to_string()),
        Osoba::nowa_osoba("Marek".to_string(), "Wójcik".to_string(), 182, 85, "1985-11-30".to_string()),
        Osoba::nowa_osoba("Ewa".to_string(), "Zamińska".to_string(), 168, 58, "1996-09-06".to_string()),
        Osoba::nowa_osoba("Ewa".to_string(), "Kamińska".to_string(), 168, 58, "1996-09-05".to_string()),
        Osoba::nowa_osoba("Tomasz".to_string(), "Kowalczyk".to_string(), 175, 75, "1990-04-18".to_string()),
        Osoba::nowa_osoba("Agnieszka".to_string(), "Zielińska".to_string(), 160, 55, "1993-08-25".to_string()),
        Osoba::nowa_osoba("Michał".to_string(), "Szymański".to_string(), 180, 78, "1987-12-12".to_string()),
        Osoba::nowa_osoba("Monika".to_string(), "Woźniak".to_string(), 167, 62, "1996-06-20".to_string()),
        Osoba::nowa_osoba("Adam".to_string(), "Kozłowski".to_string(), 185, 90, "1984-02-14".to_string()),
        Osoba::nowa_osoba("Barbara".to_string(), "Jankowska".to_string(), 163, 59, "1991-10-08".to_string()),
        Osoba::nowa_osoba("Grzegorz".to_string(), "Mazur".to_string(), 177, 82, "1989-03-03".to_string()),
        Osoba::nowa_osoba("Joanna".to_string(), "Majewska".to_string(), 169, 64, "1994-07-19".to_string()),
        Osoba::nowa_osoba("Paweł".to_string(), "Wojciechowski".to_string(), 181, 83, "1986-05-28".to_string()),
        Osoba::nowa_osoba("Karolina".to_string(), "Krawczyk".to_string(), 166, 57, "1997-01-22".to_string()),
        Osoba::nowa_osoba("Rafał".to_string(), "Piotrowski".to_string(), 179, 77, "1999-04-11".to_string()),
        Osoba::nowa_osoba("Jan".to_string(), "Bowalski".to_string(), 180, 70, "1999-12-28".to_string()),
    ]
}


fn sort_alph(osoby: &mut Vec<Osoba>) {
    osoby.sort_by_key(| osoba | osoba.nazwisko.clone());
    //println!("Sortowanie alfabetyczne po nazwisku:");
    /*for osoba in osoby.clone(){
        println!("{:?}", &osoba);
    }*/
    osoby.sort_by_key(| osoba| osoba.imie.clone());
    //println!("Sortowanie alfabetyczne po imieniu:");
    /*for osoba in osoby.clone(){
        println!("{:?}", &osoba);
    }*/
}


fn sort_age(osoby: &mut Vec<Osoba>) {
    //tutaj posortuj wg wieku rozumiem, że od najmłodszego do najstarszego
    osoby.sort_by_key(| osoba | Reverse(osoba.data_urodzenia.clone()));
}


fn main() {
    let mut osoby = example_osobas();
    sort_alph(&mut osoby);
    
    for osoba in osoby.clone(){
        println!("{:?}", &osoba);
    }

    println!("\n\n\n");

    sort_age(&mut osoby);

    for osoba in osoby.clone(){
        println!("{:?}", &osoba);
    }

    
    
}
