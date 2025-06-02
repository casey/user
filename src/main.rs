use {
  llm::{
    builder::{LLMBackend, LLMBuilder},
    chat::ChatMessage,
  },
  std::{env, fs, os::unix::fs::PermissionsExt, process::Command, str},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let home = dirs::home_dir().ok_or("failed to get home directory")?;
  let readme = home.join("README.md");
  let contents = fs::read_to_string(&readme)?;

  let api_key = env::var("OPENAI_API_KEY")
    .map_err(|_| "OPENAI_API_KEY environment variable not set or not valid unicode")?;

  let llm = LLMBuilder::new()
    .backend(LLMBackend::OpenAI)
    .api_key(&api_key)
    .model("gpt-4.1-mini")
    .stream(false)
    .build()?;

  let mut history = Vec::new();

  let message = ChatMessage::user().content(&contents).build();

  history.push(message);

  loop {
    let response = llm.chat(&history).await?;

    let text = response.text().unwrap();

    println!("{text}");

    let script = "./script.bash";

    fs::write(script, &text)?;

    let metadata = fs::metadata(script)?;
    let mut permissions = metadata.permissions();
    permissions.set_mode(permissions.mode() | 0o111);
    fs::set_permissions(script, permissions)?;

    let output = Command::new(script).output()?;

    if !output.status.success() {
      panic!("Command failed: {}", output.status);
    }

    let stdout = str::from_utf8(&output.stdout).unwrap();

    history.push(ChatMessage::assistant().content(&text).build());

    history.push(ChatMessage::user().content(stdout).build());
  }
}
