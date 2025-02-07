// I want a client to depend on an interface representing a Database instead of a concrete MySQL or PostgreSQL type,
// in order to be able to swap between databases easily without changing the client code

struct Client {
    database: Box<dyn Database>,
}

fn main() {
    let clients = [
        Client {
            database: Box::new(MySQL),
        },
        Client {
            database: Box::new(PostgreSQL),
        },
    ];
    clients.iter().for_each(|client| {
        println!("---");
        client.database.connect();
        println!("{}", client.database.query("SELECT * FROM users").unwrap());
        client.database.disconnect();
    });
}

trait Database {
    fn connect(&self);
    fn query(&self, query: &str) -> Result<String, String>;
    fn disconnect(&self);
}

struct MySQL;

impl Database for MySQL {
    fn connect(&self) {
        println!("Connected to MySQL");
    }

    fn query(&self, query: &str) -> Result<String, String> {
        println!("Querying MySQL: {}", query);
        Ok("Got 12 MySQL users".to_string())
    }

    fn disconnect(&self) {
        println!("Disconnected from MySQL");
    }
}

struct PostgreSQL;

impl Database for PostgreSQL {
    fn connect(&self) {
        println!("Connected to PostgreSQL");
    }

    fn query(&self, query: &str) -> Result<String, String> {
        println!("Querying PostgreSQL: {}", query);
        Ok("Got 43 PostgreSQL users".to_string())
    }

    fn disconnect(&self) {
        println!("Disconnected from PostgreSQL");
    }
}
