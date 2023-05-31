use std::error::Error;
use std::{env, fs};

pub struct Config {
    pub query: String,
    pub file_path: String,
    //zadanie 4: dodaj pole, które będzie przechowywać informację o tym czy ignorujemy rozmiar liter
}

impl Config {
    pub fn build(
        mut args: impl Iterator<Item=String>,
    ) -> Result<Config, &'static str> {
        args.next();
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };
        //zadanie 4: uzupełnij tą linię by odczytać z zmiennych środowiskowych czy program ma ignorować
        //rozmiar liter. tzn. jeśli jest ta zmienna ustawiona to ma ignorować.
        // https://doc.rust-lang.org/beta/std/env/fn.var.html
        // opisana wyżej funkcja zwraca Result<String, VarError>, zamiast używać matcha, możesz użyć
        // metody is_ok(), która zwraca boola, jeśli zmienna środowiskowa jest ustawiona, to zwróci true

        //zadanie 3, tutaj zwróc wartość OK(Config{}), z odpowiednimi wartosciami configu
        //w zadaniu 4 będzie koniecznie zapisanie również informacji o tym czy ignorujemy rozmiar liter
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    //zadanie 4: w poniższej linii warto użyć match, który w zależności od wartości config.ignore_case
    //wywoła search lub search_case_insensitive. Możemy też użyć if...else
    let results = search(&config.query, &contents);
    for line in results {
        println!("{line}");
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines().filter(|line| line.contains(query)).collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    //zadanie 4, uzupełnij tą linię tak by było wyszukiwanie case insensitive
}
