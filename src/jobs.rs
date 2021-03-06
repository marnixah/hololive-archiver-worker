use std::env;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Deserialize)]
#[derive(Serialize)]
#[derive(Debug)]
pub struct Job {
    pub automatic: bool,
    pub url: String,
    pub handler: String,
    pub id: u32,
    pub save_location: String,
    pub status: String,
    pub error: String,
    pub ip: String,
    pub hostname: String,
}

impl Job {
    pub fn sync(&mut self) {
        let client = reqwest::blocking::Client::new();
        let new_job = client.get(
            get_url(format!("/job/{}", self.id).as_str()).as_str()
        ).send().unwrap().json::<Job>().unwrap();
        self.status = new_job.status.clone();
        self.error = new_job.error.clone();
        self.url = new_job.url.clone();
        self.save_location = new_job.save_location.clone();
    }

    pub fn save_folder(&self) -> String {
        let path = PathBuf::from(self.save_location.clone());
        let mut folder = path.parent().unwrap().to_str().unwrap().to_string();
        folder.push_str("/");
        folder
    }

    pub fn update_status(&mut self, status: String) {
        self.sync();
        self.status = status;
        let client = reqwest::blocking::Client::new();
        client.patch(get_url(format!("/job/{}", self.id).as_str())).json(&self).send().unwrap();
    }

    pub fn update_error(&mut self, error: String) {
        self.sync();
        self.error = error;
        let client = reqwest::blocking::Client::new();
        client.patch(get_url(format!("/job/{}", self.id).as_str())).json(&self).send().unwrap();
    }

    pub fn update_hostname(&mut self, hostname: String) {
        self.sync();
        self.hostname = hostname;
        let client = reqwest::blocking::Client::new();
        client.patch(get_url(format!("/job/{}", self.id).as_str())).json(&self).send().unwrap();
    }
}

fn get_url(path: &str) -> String {
    let base_url = env::var("BASE_URL").unwrap_or(String::from("http://localhost:5000"));
    return format!("{}/{}", base_url, path);
}

pub fn pop_job() -> Result<Job, reqwest::Error> {
    let client = reqwest::blocking::Client::new();
    let response = client.delete(get_url("/job")).send().unwrap();
    return response.json();
}
