use reqwest::header::{AUTHORIZATION};
use serde::{Deserialize};
use std::{env, error};
use log::{warn, debug, info};
use anyhow::{Context, Result, Error};
use dotenv;

#[derive(Deserialize, Debug)]
struct AccessToken {
	access_token:	String,
	token_type:		String,
	expires_in:		i32,
	scope:			String,
	created_at:		i64,
}

async fn init_session() -> Result<AccessToken, Box<dyn error::Error>> {
	dotenv::dotenv().expect("Failed to read .env file!!");
	let client = reqwest::Client::new();
	let client_id = env::var("client_id")
			.with_context(|| format!("Failed to read `client_id`."))?;
	let client_secret = env::var("client_secret")
			.with_context(|| format!("Failed to read `client_secret`."))?;
	let params = [
        ("grant_type", "client_credentials"),
        ("client_id", client_id.as_str()),
        ("client_secret", client_secret.as_str()),
	];
	let response = client
		.post("https://api.intra.42.fr/oauth/token")
		.form(&params)
		.send()
		.await
		.unwrap();
	match response.status() {
		reqwest::StatusCode::OK => {
			debug!("init_session(): oauth token generated.!");
		}
		reqwest::StatusCode::UNAUTHORIZED => {
			warn!("init_session(): oauth token generat failed.");
			println!("Unauthorized");
		}
		_ => {
			panic!("Uh Oh! Something unexpected happened.");
		}
	};
	let token = response.json::<AccessToken>().await
			.with_context(|| format!("Failed to jsonize access token."))?;
	Ok(token)
}

// async fn check_login() -> Result<AccessToken, Box<dyn std::error::Error>> {
async fn check_login() -> Result<AccessToken, Box<dyn error::Error>> {
	let at = init_session().await;
	match at {
		Err(error) => {
			warn!("check_login(): check .env file.");
			// Err(Error::new(error))
			Err(error)
		}
		Ok(content) => {
			debug!("check_login(): AccessToken generated.");
			Ok(content)
		}
	}
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
	env_logger::init();
	let ac_token = check_login().await?;

	info!("{}", format!("AccessToken: {}", ac_token.access_token));
	Ok(())
}