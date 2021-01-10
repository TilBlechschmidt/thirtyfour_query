//! Requires chromedriver running on port 4444:
//!
//!     chromedriver --port=4444
//!
//! Run as follows:
//!
//!     cargo run --example wikipedia

use thirtyfour::prelude::*;
use thirtyfour_query::{ElementPoller, ElementQueryable, ElementWaitable};
use tokio::time::Duration;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let caps = DesiredCapabilities::chrome();
    let mut driver = WebDriver::new("http://localhost:4444", &caps).await?;

    // Disable implicit timeout in order to use new query interface.
    driver.set_implicit_wait_timeout(Duration::new(0, 0)).await?;

    // Set default ElementPoller strategy. This will be inherited by all future queries unless
    // specifically overridden.
    // The following will wait up to 20 seconds, polling in 0.5 second intervals.
    let poller =
        ElementPoller::TimeoutWithInterval(Duration::new(20, 0), Duration::from_millis(1000));
    driver.config_mut().set("ElementPoller", poller)?;

    // Navigate to https://wikipedia.org.
    driver.get("https://wikipedia.org").await?;

    let elem_form = driver.query(By::Id("search-form")).first().await?;

    // Find element from element using multiple selectors.
    // Each selector will be executed once per poll iteration.
    // The first element to match will be returned.
    let elem_text = elem_form
        .query(By::Css("thiswont.match"))
        .or(By::Id("searchInput"))
        .desc("search input")
        .first()
        .await?;

    // Type in the search terms.
    elem_text.send_keys("selenium").await?;

    // Click the search button. Optionally name the element to make error messages more readable.
    let elem_button =
        elem_form.query(By::Css("button[type='submit']")).desc("search button").first().await?;
    elem_button.click().await?;

    // Wait until the button no longer exists (two different ways).
    elem_button.wait_until("Timed out waiting for button to become stale").stale().await?;
    driver.query(By::Css("button[type='submit']")).nowait().not_exists().await?;

    // Look for header to implicitly wait for the page to load.
    driver.query(By::ClassName("firstHeading")).first().await?;
    assert_eq!(driver.title().await?, "Selenium - Wikipedia");

    Ok(())
}
