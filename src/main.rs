use reqwest::blocking::Client;
use std::{fs::File, process::Command};

static USER_AGENT: &str = "ShayBox/LatestSpigot";
static URL: &str = "https://hub.spigotmc.org/jenkins/job/BuildTools/lastSuccessfulBuild/artifact/target/BuildTools.jar";
static PATH: &str = "BuildTools.jar";

fn main() {
    let temp_dir = tempfile::Builder::new()
        .tempdir()
        .expect("Failed to create temporary directory");

    let user_agent = std::env::var("USER_AGENT").unwrap_or(USER_AGENT.to_string());
    let client = Client::builder()
        .user_agent(user_agent)
        .build()
        .expect("Failed to build client");

    let mut resp = client.get(URL).send().expect("Failed to get response");
    let mut file = File::create(temp_dir.path().join(PATH)).expect("Failed to create file");

    resp.copy_to(&mut file).expect("Failed to write file");

    let current_dir = std::env::current_dir()
        .expect("Failed to get current directory")
        .as_path()
        .display()
        .to_string();

    let mut child = Command::new("java")
        .args(["-jar", PATH, "-o", &current_dir])
        .args(std::env::args())
        .current_dir(temp_dir.path())
        .spawn()
        .expect("Failed to execute process");

    child.wait().expect("Failed to wait for child process");
}
