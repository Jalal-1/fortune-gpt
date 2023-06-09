use fortune_gpt::domain::job::field::{
    EscrowId, Expires, ManifestUrl, Password, Posted, ShortCode,
};
use fortune_gpt::service::ask::{GetJob, NewJob, UpdateJob};
use fortune_gpt::web::api::{ApiKey, API_KEY_HEADER};
use fortune_gpt::Job;
use std::error::Error;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
enum Command {
    Get {
        shortcode: ShortCode,
        #[structopt(short, long, help = "password")]
        password: Option<String>,
    },
    New {
        #[structopt(help = "escrow_id")]
        job: String,
        #[structopt(help = "posted")]
        posted: u64,
        #[structopt(short, long, help = "password")]
        password: Option<Password>,
        #[structopt(short, long, help = "expiration date")]
        expires: Option<Expires>,
        #[structopt(short, long, help = "manifest_url")]
        manifest_url: Option<ManifestUrl>,
    },
    Update {
        shortcode: ShortCode,
        job: String,
        #[structopt(short, long, help = "password")]
        password: Option<Password>,
        #[structopt(short, long, help = "expiration date")]
        expires: Option<Expires>,
        #[structopt(short, long, help = "manifest_url")]
        manifest_url: Option<ManifestUrl>,
    },
}

#[derive(StructOpt, Debug)]
#[structopt(name = "fortuneclient", about = "Fortune-GPT API Client")]
struct Opt {
    #[structopt(subcommand)]
    command: Command,

    #[structopt(default_value = "http://127.0.0.1:8000", env = "FORTUNE_GPT_ADDR")]
    addr: String,

    #[structopt(long)]
    api_key: ApiKey,
}

fn get_job(addr: &str, ask_svc: GetJob, api_key: ApiKey) -> Result<Job, Box<dyn Error>> {
    let client = reqwest::blocking::Client::builder().build()?;
    let addr = format!("{}/api/job/{}", addr, ask_svc.shortcode.into_inner());
    let mut request = client.get(addr);
    request = match ask_svc.password.into_inner() {
        Some(password) => request.header(reqwest::header::COOKIE, format!("password={}", password)),
        None => request,
    };
    request = request.header(API_KEY_HEADER, api_key.to_base64());
    Ok(request.send()?.json()?)
}

fn new_job(addr: &str, ask_svc: NewJob, api_key: ApiKey) -> Result<Job, Box<dyn Error>> {
    let client = reqwest::blocking::Client::builder().build()?;
    let addr = format!("{}/api/job", addr);
    let mut request = client.post(addr);
    request = request.header(API_KEY_HEADER, api_key.to_base64());
    Ok(request.json(&ask_svc).send()?.json()?)
}

fn update_job(addr: &str, ask_svc: UpdateJob, api_key: ApiKey) -> Result<Job, Box<dyn Error>> {
    let client = reqwest::blocking::Client::builder().build()?;
    let addr = format!("{}/api/job", addr);
    let mut request = client.put(addr);
    request = request.header(API_KEY_HEADER, api_key.to_base64());
    Ok(request.json(&ask_svc).send()?.json()?)
}

fn run(opt: Opt) -> Result<(), Box<dyn Error>> {
    match opt.command {
        Command::Get {
            shortcode,
            password,
        } => {
            let req = GetJob {
                password: Password::new(password.unwrap_or_default())?,
                shortcode,
            };
            let job = get_job(opt.addr.as_str(), req, opt.api_key)?;
            println!("{:#?}", job);
            Ok(())
        }
        Command::New {
            job,
            posted,
            password,
            expires,
            manifest_url,
        } => {
            let req = NewJob {
                escrow_id: EscrowId::new(job.as_str())?,
                posted: Posted::new(posted),
                manifest_url: manifest_url.unwrap_or_default(),
                expires: expires.unwrap_or_default(),
                password: password.unwrap_or_default(),
            };
            let job = new_job(opt.addr.as_str(), req, opt.api_key)?;
            println!("{:#?}", job);
            Ok(())
        }
        Command::Update {
            job,
            password,
            expires,
            manifest_url,
            shortcode,
        } => {
            let password = password.unwrap_or_default();
            let svc_req = GetJob {
                password: password.clone(),
                shortcode: shortcode.clone(),
            };
            let original_job = get_job(opt.addr.as_str(), svc_req, opt.api_key.clone())?;
            let svc_req = UpdateJob {
                escrow_id: EscrowId::new(job.as_str())?,
                expires: expires.unwrap_or(original_job.expires),
                manifest_url: manifest_url.unwrap_or(original_job.manifest_url),
                password,
                shortcode,
            };
            let job = update_job(opt.addr.as_str(), svc_req, opt.api_key)?;
            println!("{:#?}", job);
            Ok(())
        }
    }
}

fn main() {
    let opt = Opt::from_args();
    if let Err(e) = run(opt) {
        eprintln!("An error occurred: {}", e);
    }
}
