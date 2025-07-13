struct Client {
    db: Database,
}

impl Client {
    fn new(db: Database) -> Self {
        Self { db }
    }
}

enum Database {
    MySQL(MySQL),
    PostgreSQL(PostgreSQL),
}

impl Database {
    // boilerplate...
    fn connect(&self) {
        match self {
            Database::MySQL(db) => db.connect(),
            Database::PostgreSQL(db) => db.connect(),
        }
    }

    fn query(&self, query: &str) -> Result<String, String> {
        match self {
            Database::MySQL(db) => db.query(query),
            Database::PostgreSQL(db) => db.query(query),
        }
    }

    fn disconnect(&self) {
        match self {
            Database::MySQL(db) => db.disconnect(),
            Database::PostgreSQL(db) => db.disconnect(),
        }
    }
}

fn main() {
    let clients = [
        Client::new(Database::MySQL(MySQL)),
        Client::new(Database::PostgreSQL(PostgreSQL)),
    ];
    clients.iter().for_each(|client| {
        println!("---");
        client.db.connect();
        client.db.query("SELECT * FROM users").unwrap();
        client.db.disconnect();
    });
}

trait DatabaseConnection {
    fn connect(&self);
    fn disconnect(&self);
}

trait DatabaseQuery {
    fn query(&self, query: &str) -> Result<String, String>;
}

struct MySQL;

impl DatabaseConnection for MySQL {
    fn connect(&self) {
        println!("Connected to MySQL");
    }
    fn disconnect(&self) {
        println!("Disconnected from MySQL");
    }
}

impl DatabaseQuery for MySQL {
    fn query(&self, query: &str) -> Result<String, String> {
        println!("Querying MySQL: {query}");
        Ok("Got 12 MySQL users".to_string())
    }
}

struct PostgreSQL;

impl DatabaseConnection for PostgreSQL {
    fn connect(&self) {
        println!("Connected to PostgreSQL");
    }
    fn disconnect(&self) {
        println!("Disconnected from PostgreSQL");
    }
}

impl DatabaseQuery for PostgreSQL {
    fn query(&self, query: &str) -> Result<String, String> {
        println!("Querying PostgreSQL: {query}");
        Ok("Got 43 PostgreSQL users".to_string())
    }
}
