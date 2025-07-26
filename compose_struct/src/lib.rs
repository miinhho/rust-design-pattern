#[derive(Debug, Clone)]
struct ConnectionString(String);

#[derive(Debug, Clone, Copy)]
struct Timeout(u32);

#[derive(Debug, Clone, Copy)]
struct PoolSize(u32);

struct Database {
    connection_string: ConnectionString,
    timeout: Timeout,
    pool_size: PoolSize,
}

fn print_database(connection_str: ConnectionString, timeout: Timeout, pool_size: PoolSize) {
    println!("Connection string: {connection_str:?}");
    println!("Timeout: {timeout:?}");
    println!("Pool size: {pool_size:?}");
}

#[cfg(test)]
mod tests {
    use crate::{ConnectionString, Database, PoolSize, Timeout, print_database};

    #[test]
    fn test_database() {
        let mut db = Database {
            connection_string: ConnectionString("localhost".to_string()),
            timeout: Timeout(30),
            pool_size: PoolSize(100),
        };

        let connection_string = &mut db.connection_string;
        print_database(connection_string.clone(), db.timeout, db.pool_size);
        *connection_string = ConnectionString("new string".to_string());
    }
}
