use {
    llm::{
        builder::{LLMBackend, LLMBuilder},
        chat::ChatMessage,
    },
    std::{env, fs},
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
        .model("gpt-3.5-turbo")
        .stream(false)
        .build()?;

    let message = ChatMessage::user().content(&contents).build();

    let response = llm.chat(&[message]).await?;

    println!("{response}");

    Ok(())
}
