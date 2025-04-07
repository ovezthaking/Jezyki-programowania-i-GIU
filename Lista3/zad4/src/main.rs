use std::cmp::Reverse;
use std::fs::File;
use serde_json::{from_reader, to_writer};
use serde::{ser::{Serialize, SerializeStruct, Serializer}, Deserialize};
use serde::de::*;
use std::path::Path;


#[derive(Debug, Clone)]
struct Osoba{
    imie: String,
    nazwisko: String,
    wzrost: i16,
    waga: i16,
    data_urodzenia: String,
}

impl Osoba{
    fn nowa_osoba(imie: String, nazwisko: String, wzrost: i16, waga: i16, data_urodzenia: String) -> Osoba{
        Osoba{
            imie,
            nazwisko,
            wzrost,
            waga,
            data_urodzenia,
        }
    }
}

impl Serialize for Osoba {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where 
        S:Serializer, 
    {
        let mut state = serializer.serialize_struct("Osoba", 5)?;
        state.serialize_field("Imię", &self.imie)?;
        state.serialize_field("Nazwisko", &self.nazwisko)?;
        state.serialize_field("Wzrost", &self.wzrost)?;
        state.serialize_field("Waga", &self.waga)?;
        state.serialize_field("DataUrodzenia", &self.data_urodzenia)?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for Osoba {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // Definicja pól, które będą deserializowane
        //const FIELDS: &[&str] = &["Imię", "Nazwisko", "Wzrost", "Waga", "Data urodzenia"];

        // Użycie serde do deserializacji struktury
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "PascalCase")]
        enum Field {
            Imię,
            Nazwisko,
            Wzrost,
            Waga,
            DataUrodzenia,
        }

        struct OsobaVisitor;

        impl<'de> Visitor<'de> for OsobaVisitor {
            type Value = Osoba;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct Osoba")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Osoba, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut imie = None;
                let mut nazwisko = None;
                let mut wzrost = None;
                let mut waga = None;
                let mut data_urodzenia = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Imię => imie = Some(map.next_value()?),
                        Field::Nazwisko => nazwisko = Some(map.next_value()?),
                        Field::Wzrost => wzrost = Some(map.next_value()?),
                        Field::Waga => waga = Some(map.next_value()?),
                        Field::DataUrodzenia => data_urodzenia = Some(map.next_value()?),
                    }
                }

                let imie = imie.ok_or_else(|| serde::de::Error::missing_field("Imię"))?;
                let nazwisko = nazwisko.ok_or_else(|| serde::de::Error::missing_field("Nazwisko"))?;
                let wzrost = wzrost.ok_or_else(|| serde::de::Error::missing_field("Wzrost"))?;
                let waga = waga.ok_or_else(|| serde::de::Error::missing_field("Waga"))?;
                let data_urodzenia = data_urodzenia.ok_or_else(|| serde::de::Error::missing_field("Data urodzenia"))?;

                Ok(Osoba {
                    imie,
                    nazwisko,
                    wzrost,
                    waga,
                    data_urodzenia,
                })
            }
        }
        const FIELDS:&[&str] = &["Imię", "Nazwisko", "Wzrost", "Waga", "DataUrodzenia"];
        deserializer.deserialize_struct("Osoba", FIELDS, OsobaVisitor)
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
    osoby.sort_by_key(| osoba | osoba.imie.clone());
}


fn sort_age(osoby: &mut Vec<Osoba>) {
    osoby.sort_by_key(| osoba | Reverse(osoba.data_urodzenia.clone()));
}


fn wzrost_range(osoby:&Vec<Osoba>, a:i16, b:i16) -> Vec<Osoba>{
    let mut w_range = vec![];
    
    for osoba in osoby{
        if osoba.wzrost >= a && osoba.wzrost <=b{
            w_range.push(osoba.clone());
        }
    }
    w_range
}


fn masa_info(osoby: &Vec<Osoba>) -> (f64, f64){
    let mut sum = 0.0;
    let mut i = 0;
    loop {
        if i >= osoby.len(){
            break;
        }
        sum += osoby[i].waga as f64;
        i+=1;
    }

    let avg = sum/osoby.len() as f64;

    (sum, avg)
}


fn sum_avg<F>(osoby: &Vec<Osoba>, f: F) -> (f64, f64)
where F:Fn(&Osoba) -> f64 {
    let mut sum = 0.0;
    for osoba in osoby{
        sum += f(osoba);
    }
    let avg = sum/osoby.len() as f64;
    (sum,avg)
}



fn list_all(osoby: &Vec<Osoba>){
    if osoby.is_empty() {
        println!("Lista osób jest pusta");
    }
    else {
        println!("Lista osób: ");
        for osoba in osoby.iter().enumerate() {
            println!("{:?}", &osoba);
        }
        print!("\n");
    }
}


fn save(osoby: &Vec<Osoba>){
    let file = File::create("osoby.json").expect("can't create a file");
    to_writer(file, &osoby).expect("Can't save any data");
}


fn load() -> Vec<Osoba>{
    let file = File::open("osoby.json").expect("Can't open a file");
    let osoby:Vec<Osoba> = from_reader(file).expect("Can't set a variable");
    osoby
}


fn add(osoby: &mut Vec<Osoba>, osoba: Osoba){
    osoby.push(osoba);
    save(&osoby);
}


fn remove(osoby: &mut Vec<Osoba>, index:usize ){
    osoby.remove(index);
    save(&osoby);
}
/* 
fn main() {
    let mut osoby = example_osobas();
    sort_alph(&mut osoby);
    
    for osoba in &osoby{
        println!("{:?}", &osoba);
    }

    println!("\n\n\n");

    sort_age(&mut osoby);

    for osoba in &osoby{
        println!("{:?}", &osoba);
    }

    let przedzial = wzrost_range(&osoby, 160, 170);
    println!("\n\n\n");
    for osoba in przedzial{
        println!("{:?}", &osoba);
    }
    
    println!("\n\n\n");

    let (masa_sum, masa_avg) = masa_info(&osoby);
    println!("Śuma mas wynosoi {}, a ich średnia {}", masa_sum, masa_avg);

    let (wiek_sum, wiek_avg) = sum_avg(&osoby, |osoba| {
        let data: String = String::from(&osoba.data_urodzenia);
        let rok = &data[..4];
        let rok_f64: f64 = (*rok).parse().unwrap();
        let obecny = 2025.0;
        obecny-rok_f64
    });

    println!("\nSuma wieku osób wynosi {}, a ich średnia {}", wiek_sum, wiek_avg);

    save(&osoby);

    let osoby_frf:Vec<Osoba> = load();

    println!("\nOsoby zaimportowane z dysku:\n{:?}", osoby_frf);

    print!("\n\n");
    list_all(&osoby_frf);
}
*/

fn main() {

    let mut osoby:Vec<Osoba>;
    let path = Path::new("osoby.json");
    if path.exists() {
        osoby = load();
    }
    else{
        osoby = example_osobas();
    }
    

    save(&osoby);
    
    //list_all(&osoby);

    let mut running = true;

    while running {
        osoby = load();
        let mut choice = String::new();
        println!("Dostępne opcje: list_all(), add(), remove(), find(), exit()");
        let stdin = std::io::stdin();
        stdin.read_line(&mut choice).expect("Failed to read line (choice)");
        choice = choice.trim().to_string();
        println!("Wybrano: {}", choice);

        match choice.as_str() {
            "list_all()" =>{
                list_all(&osoby);
            },
            "exit()" => {
                println!("Zamykanie programu");
                running = false;
            }
            "add()" => {
                let mut imie = String::new();
                println!("Podaj imię:");
                stdin.read_line(&mut imie).expect("Failed to read line (imie)");
                let mut nazwisko = String::new();
                println!("Podaj nazwisko:");
                stdin.read_line(&mut nazwisko).expect("Failed to read line (nazwisko)");
                let mut wzrost = String::new();
                println!("Podaj wzrost:");
                stdin.read_line(&mut wzrost).expect("Failed to read line (wzrost)");
                let mut waga = String::new();
                println!("Podaj wagę:");
                stdin.read_line(&mut waga).expect("Failed to read line (waga)");
                let mut data_urodzenia = String::new();
                println!("Podaj datę urodzenia (rrrr-mm-dd):");
                stdin.read_line(&mut data_urodzenia).expect("Failed to read line (data_urodzenia)");

                let nowa_osoba = Osoba::nowa_osoba(
                    imie.trim().to_string(),
                    nazwisko.trim().to_string(),
                    wzrost.trim().parse().unwrap(),
                    waga.trim().parse().unwrap(),
                    data_urodzenia.trim().to_string(),
                );
                println!("\nDodano nową osobę: {:?}", &nowa_osoba);
                add(&mut osoby, nowa_osoba);
                
            },
            "remove()" => {
                list_all(&osoby);
                println!("Podaj index osoby, którą chcesz usunąć z listy powyżej: ");
                let mut index = String::new();
                stdin.read_line(&mut index).expect("Falied to read index");
                let index_usize: usize = index.trim().parse().expect("Failed to parse");
                
                if index_usize >= osoby.len(){
                    println!("Index osoby za duży, osoba o takim indexie nie istnieje");
                }
                else if index_usize < osoby.len(){
                    println!("Usuwanie osoby {:?} ...", &osoby[index_usize]);
                    remove(&mut osoby, index_usize);
                }
                else {
                    println!("Nie jest to liczba -_-");
                }
            },
            "find()" => {

            },
            _ => println!("Nieznana opcja: {}", choice),
        }
    }
}