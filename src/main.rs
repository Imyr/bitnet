use reqwest::{Client, Response};
use clap::{Parser, Subcommand};

/// CLI to manage internet access for students of BIT Mesra
#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Login to network
    Login {username: String, password: String},
    /// Logout from network
    Logout
}

async fn login(client: Client, credentials: (String, String)) -> Response {
    let login_url = "http://172.16.1.1:8090/login.xml";
    let form = [("mode", "191"), ("username", &credentials.0[..]), ("password", &credentials.1[..])];
    let res = client.post(login_url).form(&form).send().await;
    match res {
        Ok(result) => {
            return result
        }
        Err(error) => {
            panic!("{}", error)
        }
    }
}

async fn logout(client: Client) -> Response {
    let logout_url = "http://172.16.1.1:8090/logout.xml";
    let form = [("mode", "193"), ("username", " ")];
    let res = client.post(logout_url).form(&form).send().await;
    match res {
        Ok(result) => {
            return result
        }
        Err(error) => {
            panic!("{}", error)
        }
    }
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let client = Client::new();
    match cli.command {
        Commands::Login { username, password } => {
            println!("{}", "Logging in...");
            let response = login(client, (username, password)).await;
            print!("{}", response.text().await.unwrap());
            println!("{}", "Logged in.");
        }
        Commands::Logout => {
            println!("{}", "Logging out...");
            let response = logout(client).await;
            print!("{}", response.text().await.unwrap());
            println!("{}", "Logged out.");
        }
    }   
}
