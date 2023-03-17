use std::{env, fs::File, process::Command};

use reqwest::blocking::Client;

static USER_AGENT: &str = "ShayBox/LatestSpigot";
static URL: &str = "https://hub.spigotmc.org/jenkins/job/BuildTools/lastSuccessfulBuild/artifact/target/BuildTools.jar";
static PATH: &str = "BuildTools.jar";

fn main() -> anyhow::Result<()> {
    let temp_dir = tempfile::Builder::new().tempdir()?;

    let user_agent = env::var("USER_AGENT").unwrap_or(USER_AGENT.into());
    let client = Client::builder().user_agent(user_agent).build()?;

    let mut resp = client.get(URL).send()?;
    let mut file = File::create(temp_dir.path().join(PATH))?;
    resp.copy_to(&mut file)?;

    let current_dir = env::current_dir()?.as_path().display().to_string();
    let mut child = Command::new("java")
        .args(["-jar", PATH, "-o", &current_dir])
        .args(env::args())
        .current_dir(temp_dir.path())
        .spawn()?;

    child.wait()?;

    Ok(())
}
