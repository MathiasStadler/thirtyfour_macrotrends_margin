// FROM HERE
// https://github.com/stevepryde/thirtyfour/blob/main/thirtyfour/examples/selenium_example.rs

//! Requires selenium running on port 4444:
//!
//!     java -jar selenium-server-standalone-3.141.59.jar
//!
//! Run as follows:
//!
//!     cargo run --example selenium_example
use log::info;
use thirtyfour::prelude::*;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    // The use of color_eyre gives much nicer error reports, including making
    // it much easier to locate where the error occurred.
    color_eyre::install()?;

    // let caps = DesiredCapabilities::chrome();
    // NOTE: For selenium 3.x, use "http://localhost:4444/wd/hub/session".
    // let driver = WebDriver::new("http://localhost:4444", caps).await?;

    let _driver = initialize_driver().await?;

    // Navigate to https://wikipedia.org.
    _driver.goto("https://wikipedia.org").await?;
    let elem_form = _driver.find(By::Id("search-form")).await?;

    // Find element from element.
    let elem_text = elem_form.find(By::Id("searchInput")).await?;

    // Type in the search terms.
    elem_text.send_keys("selenium").await?;

    // Click the search button.
    let elem_button = elem_form.find(By::Css("button[type='submit']")).await?;
    elem_button.click().await?;

    // Look for header to implicitly wait for the page to load.
    _driver.find(By::ClassName("firstHeading")).await?;
    assert_eq!(_driver.title().await?, "Selenium - Wikipedia");

    // Always explicitly close the browser. There are no async destructors.
    _driver.quit().await?;

    Ok(())
}

async fn initialize_driver() -> Result<WebDriver, WebDriverError> {
    info!("initialize_driver - start");

    let mut _caps = DesiredCapabilities::chrome();

    // let mut caps: thirtyfour::ChromeCapabilities = DesiredCapabilities::chrome();
    // caps.add_chrome_arg("--headless")?;
    // caps.add_chrome_arg("--no-sandbox")?;
    //  caps.add_chrome_arg("--disable-dev-shm-usage")?;

    _caps.add_arg("--remote-debugging-pipe")?;
    _caps.add_arg("--no-sandbox")?;
    
    let driver_result = WebDriver::new("http://localhost:9515", _caps).await;

    // let result = WebDriver::new("http://localhost:4444/wd/hub", &caps).await;
    let driver = match driver_result {
        Ok(value) => value,
        Err(error) => return Err(error),
    };

    driver.maximize_window().await?;
    info!("initialize_driver - end");
    Ok(driver)
}
