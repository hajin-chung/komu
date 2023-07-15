use cuid;
use sqlite;
use std::sync::{Arc, Mutex};
use std::thread;

macro_rules! time_it {
    ($context:literal, $s:stmt) => {
        let timer = std::time::Instant::now();
        $s
        println!("{}: {:?}", $context, timer.elapsed());
    };
}

fn generate_insert_data(is_first: bool) -> String {
    let mut id = String::from("secret!!");

    if !is_first {
        id = cuid::cuid2();
    }

    format!(
        "INSERT INTO users VALUES ('{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}');",
        id,
        cuid::cuid2(),
        cuid::cuid2(),
        cuid::cuid2(),
        cuid::cuid2(),
        cuid::cuid2(),
        cuid::cuid2(),
        cuid::cuid2(),
        cuid::cuid2()
    )
}

fn init_db(connection: &sqlite::Connection) {
    let create_table_query = "
        CREATE TABLE users (
            id TEXT, 
            name TEXT, 
            nick TEXT, 
            sex TEXT, 
            url TEXT, 
            email TEXT, 
            foo TEXT, 
            bar TEXT, 
            baz TEXT
        );
    ";
    connection.execute(create_table_query).unwrap();

    for i in 0..1000 {
        connection.execute(generate_insert_data(i == 0)).unwrap();
    }
}

fn query_db(i: i32, connection: Arc<Mutex<sqlite::Connection>>) {
    let mut values = Vec::new();
    let age_query = "SELECT * FROM users WHERE id == 'secret!!'";
    connection
        .lock()
        .unwrap()
        .iterate(age_query, |pairs| {
            for &(_name, value) in pairs.iter() {
                match value {
                    Some(c) => values.push(String::from(c)),
                    None => println!("value was None"),
                }
            }
            true
        })
        .unwrap();
    println!("from thread number {}", i);
    println!("result total value count: {}", values.len());
}

fn main() {
    let connection = sqlite::open(":memory:").unwrap();

    time_it!("init_db", init_db(&connection));

    let mut threads = Vec::new();
    let connection_arc = Arc::new(Mutex::new(connection));

    let start = std::time::Instant::now();

    for i in 1..10 {
        let connection = Arc::clone(&connection_arc);
        threads.push(thread::spawn(move || query_db(i, connection)));
    }

    while threads.len() > 0 {
        let cut_thread = threads.remove(0);
        cut_thread.join().unwrap();
    }

    let duration = start.elapsed();
    println!("elapsed: {:?}", duration);
}
