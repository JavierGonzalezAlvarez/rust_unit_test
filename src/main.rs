use postgres::{Client, NoTls};

fn delete() -> Result<(), Box<dyn std::error::Error>> {

    let mut database: Client = Client::connect("postgresql://test:2525_ap@localhost/invoices", NoTls)?;
    println!("running delete");
    database.batch_execute("DELETE FROM usuario_1 WHERE (created_at < now() - interval '180 day');")?;
    println!("done delete");
    Ok(())
}

fn vacuum() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = Client::connect("postgresql://test:2525_ap@localhost/invoices", NoTls)?;
    client.batch_execute("VACUUM FULL usuario_1;")?;
    println!("vacuum executed on table header");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vacuum() {
        let result = vacuum();
        assert!(result.is_ok());
    }
}

fn main() {
    if let Err(err) = delete() {
        println!("error in delete: {}", err)
    };
    if let Err(err) = vacuum() {
        println!("error in vacuum: {}", err)
    };
    
}
