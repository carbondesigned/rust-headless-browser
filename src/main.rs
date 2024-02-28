use fantoccini::{ClientBuilder, Locator};
use std::error::Error;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = ClientBuilder::native()
        .connect("http://localhost:9515")
        .await?;

    // Go to the desired web page
    client.goto("https://labs.perplexity.ai/").await?;

    // Find the dropdown element by its id and select the option with the value "mixtral-8x7b-instruct"
    let select = client.find(Locator::Css("#lamma-select")).await?;
    select.select_by_value("mixtral-8x7b-instruct").await?;
    // then select the textarea with the placeholder "Ask anything..."
    let textarea = client
        .find(Locator::Css("textarea[placeholder='Ask anything...']"))
        .await?;
    textarea.send_keys("Hello, World!").await?;

    // Clean up the session
    // client.close().await?;

    Ok(())
}
