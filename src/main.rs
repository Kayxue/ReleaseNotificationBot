use dotenv::dotenv;
use rand::Rng;
use twilight_util::builder::embed::{EmbedBuilder, EmbedFieldBuilder};
use xitca_web::{
    App, codegen::route, error::Error, handler::json::LazyJson, http::HeaderMap, middleware::Logger,
};


mod CustomError;
mod Discord;
mod GitHub;
mod Middleware;

use CustomError::BadRequest;
use GitHub::RequestBody::ReleaseRequestBody;

#[route("/github",method = post)]
async fn github_webhook(
    header: HeaderMap,
    body: Option<LazyJson<ReleaseRequestBody<'_>>>,
) -> Result<&'static str, Error> {
    if let Some(event) = header.get("x-github-event") {
        if event != "release" {
            return Ok("Receieved");
        }
    } else {
        return Err(BadRequest::new("Request is not from GitHub").into());
    }
    let valid_body = match body {
        Some(b) => b,
        None => return Err(BadRequest::new("Request body is missing").into()),
    };
    let body = valid_body.deserialize()?;
    if body.action != "edited" {
        return Ok("No action needed");
    }
    if body.release.assets.len() != 3 {
        return Ok("Assets are not complete");
    }

    // Generate random color for embed
    let mut rng = rand::thread_rng();
    let random_color = rng.gen_range(0..0xFFFFFF);

    // Build embed with release information
    let mut embed = EmbedBuilder::new()
        .title(body.release.name.unwrap_or("Unnamed Release"))
        .color(random_color);

    // Add a field for each asset with filename as name and download link as value
    for asset in body.release.assets.iter().flatten() {
        let download_link = format!("[Download Link]({})", asset.browser_download_url);
        let field = EmbedFieldBuilder::new(asset.name, download_link).build();
        embed = embed.field(field);
    }

    let embed = embed.build();

    if let Err(e) = Discord::send_embed(embed).await {
        eprintln!("Failed to send Discord embed: {}", e);
        // Don't fail the webhook even if Discord message fails
    }

    Ok("Release processed")
}

#[route("/",method = get)]
async fn root() -> &'static str {
    "Hello, World"
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    // Initialize Discord bot
    let discord_token =
        std::env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN environment variable must be set");

    // Initialize Discord HTTP client and channel ID
    Discord::init_http_client(discord_token.clone())
        .expect("Failed to initialize Discord HTTP client");

    Discord::init_channel_id().expect("Failed to initialize Discord channel ID");

    // Start Discord gateway to make bot online
    Discord::start_gateway(discord_token)
        .await
        .expect("Failed to start Discord gateway");

    println!("Discord bot is now online!");

    App::new()
        .at_typed(root)
        .at_typed(github_webhook)
        .enclosed_fn(Middleware::error_handler)
        .enclosed(Logger::new())
        .serve()
        .bind(("0.0.0.0", 3000))?
        .run()
        .await
}
