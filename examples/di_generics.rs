fn main() {
    let client = Client::new(MySQL);
    println!("---");
    client.db.connect();
    client.db.query("SELECT * FROM users").unwrap();
    client.db.disconnect();

    let client = Client::new(PostgreSQL);
    println!("---");
    client.db.connect();
    client.db.query("SELECT * FROM users").unwrap();
    client.db.disconnect();
}

struct Client<T>
where
    T: DatabaseConnection + DatabaseQuery,
{
    db: T,
}

trait DatabaseConnection {
    fn connect(&self);
    fn disconnect(&self);
}

trait DatabaseQuery {
    fn query(&self, query: &str) -> Result<String, String>;
}

impl<T> Client<T>
where
    T: DatabaseConnection + DatabaseQuery,
{
    fn new(db: T) -> Self {
        Self { db }
    }
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
        println!("Querying MySQL: {}", query);
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
        println!("Querying PostgreSQL: {}", query);
        Ok("Got 43 PostgreSQL users".to_string())
    }
}
