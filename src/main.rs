
use tokio::net::{TcpListener, TcpStream};

// Function to handle client connections and commands
fn handle_client(mut _stream: TcpStream) -> Result<(), String> {
    Ok(())
}

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();

    loop {
        let stream = listener.accept().await;

        match stream {
            Ok((stream, _)) => {
                let result = handle_client(stream);

                if let Err(e) = result {
                    eprintln!("Error: {}", e);
                }
            }
            Err(e) => {
                eprintln!("Connection error: {}", e);
            }
        }
    }
}
