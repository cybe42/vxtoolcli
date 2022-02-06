//use crate::cmd_manager;
use reqwest;
use url::{Url, Host};
use reqwest::{blocking::Client, blocking::RequestBuilder, StatusCode};

pub fn command(_args: Vec<&str>) -> bool {
    if _args.len() > 1 {
        let parsed = Url::parse(_args[1]);
        let parsed = match parsed {
            Ok(url) => url,
            Err(_error) => {
                println!("The url could not be parsed.");
                return true;
            }
        };
        if parsed.host() != Some(Host::Domain("discord.com")) && parsed.host() != Some(Host::Domain("discordapp.com")) {
            println!("URL must be from discord.");
            return true;
        }
        let client: Client = Client::new();
        let reqbuilder: RequestBuilder = client.delete(_args[1]);
        let result = reqbuilder.send();
        let result = match result {
            Ok(res) => res,
            Err(error) => {
                println!("Error while sending request: {}", error);
                return true;
            }
        };
        let status: StatusCode = result.status();
        match status.as_u16() {
            204 => println!("Sucessfully deleted webhook."),
            404 => println!("This webhook does not exist."),
            _ => println!("Unknown status code: {}", status)
        }
    } else {
        println!("A URL of the webhook is required as an argument.");
        return true;
    }
    return false;
}