extern crate reqwest;
use std::collections::HashMap;

const REDIRECT_URL: &str = "http://baidu.com/?cmd=redirect&arubalp=12345";
const POST_URL: &str = "https://portalwy.dhu.edu.cn/post.php";
const USERNAME: &str = "10105123";
const PASSWORD: &str = "Lxy67794716";

fn get_sessid() -> Result<HashMap<String, String>, reqwest::Error> {
    // let mut post_url = POST_URL.to_string();
    let session = reqwest::blocking::Client::new();
    

    let resp = session.get(REDIRECT_URL).send()?;
    
    dbg!(&resp);
    if let Some(host) = resp.url().host() {
        let host: String = host.to_string();
        if host == "baidu.com" {
            
            for cookie_i in resp.cookies() {
                print!("!!!{:?}",cookie_i);
            }
            println!("Already logged in!");
            return Ok(HashMap::new());
        }

    }

    let cookie = resp.cookies();

    
    // let url = resp.url();
    // let param = url.clone().join(&POST_URL).unwrap();
    // POST_URL = POST_URL.replace("portalwy.dhu.edu.cn", param.host_str().unwrap_or_default()).as_str();
    let mut cookies: HashMap<String, String> = HashMap::new();
    for cookie_i in cookie {
        cookies.insert(cookie_i.name().to_string(), cookie_i.value().to_string());
    }
    return Ok(cookies);
}

fn login() {
    match get_sessid() {
        Ok(cookies) => {
            if cookies.is_empty() {
                println!("Skipped login...");
                return;
            }
            let client = reqwest::blocking::Client::new();
            let resp = client
                .post(POST_URL)
                .form(&[("username", USERNAME), ("password", PASSWORD)])
                .header(
                    reqwest::header::COOKIE,
                    cookies
                        .iter()
                        .map(|(k, v)| format!("{}={}", k, v))
                        .collect::<Vec<String>>()
                        .join("; "),
                )
                .send()
                .unwrap();

            if resp.url().to_string().contains("error") {
                let param = reqwest::Url::parse(resp.url().to_string().as_str()).unwrap();
                let qs = param.query_pairs().collect::<HashMap<_, _>>();
                println!("Login Failed! Error: {:?}", qs.get("error").unwrap());
            } else {
                println!("Login Succeed!");
            }
        }
        Err(err) => {
            println!("Error: {:?}", err);
        }
    }
}

fn main() {
    login();
}
