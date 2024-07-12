// update here
// RUN PRG /W full log output
// RUST_LOG=debug cargo run --example thirtyfour_get_margin_data_twenty_four 2>&1 | tee output1.txt

// start cleanup

// RUSTFLAGS=-Wunused-crate-dependencies cargo

// env_logger
// RUST_LOG=info ./main

// thirtyfour 2024
// https://www.zenrows.com/blog/rust-selenium#install-selenium
#[allow(unused_imports)]
use log::{debug, error, info, log_enabled, Level};

// use async_recursion::async_recursion;

use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::process;

use std::thread;
use std::time::Duration;

use std::env::set_var;
use thirtyfour::ChromiumLikeCapabilities;
#[allow(unused_imports)]
use thirtyfour::{prelude::WebDriverError, By, DesiredCapabilities, Key, WebDriver, WebElement};

const WEB_PAGE: &str = "https://wikipedia.org";

pub type WebDriverResult<T> = Result<T, WebDriverError>;

fn main() -> color_eyre::Result<(), Box<dyn Error>> {
    color_eyre::install()?;

    // set default log level
    set_var("RUST_LOG", "debug");

    env_logger::builder()
        .format(|buf, record| {
            let warn_style = buf.default_level_style(log::Level::Warn);
            let _timestamp = buf.timestamp();
            writeln!(
                buf,
                // FROM HERE
                // https://docs.rs/env_logger/latest/src/custom_format/custom_format.rs.html#35
                "{}:{}  {warn_style}{}{warn_style:#} - {}",
                // record.level(),
                record.file().unwrap_or("unknown"),
                record.line().unwrap_or(0),
                // chrono::Local::now().format("%Y-%m-%dT%H:%M:%S"),
                record.level(),
                record.args(),
            )
        })
        .init();

    error!("RUST_LOG maybe NOT enable");
    error!("Used: => RUST_LOG=info < prg >");

    let rt: tokio::runtime::Runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?;
    let _ = rt.block_on(run());

    info!("env_logger: ended");
    process::exit(0);
}

async fn run() -> color_eyre::Result<(), Box<dyn Error>> {
    let _driver = initialize_driver().await?;

    _driver.goto(WEB_PAGE).await?;

    // thread::sleep(Duration::from_secs(10));
    wait_seconds_of_browser(_driver.clone(), 10).await?;

    close_browser(_driver.clone()).await?;

    Ok(())
}

#[allow(dead_code)]
async fn close_browser(_driver: WebDriver) -> color_eyre::Result<(), Box<dyn Error>> {
    // Always explicitly close the browser.
    _driver.quit().await?;

    Ok(())
}

#[allow(dead_code)]
async fn screenshot_browser(driver: WebDriver) -> color_eyre::Result<(), Box<dyn Error>> {
    // FROM HERE  screenshot of browser windows
    // https://stackoverflow.com/questions/60999624/trying-to-take-and-save-a-screenshot-of-a-specific-element-selenium-python-ch

    let _screenshot = driver.screenshot_as_png().await?;

    // FROM HERE  write to file
    // https://doc.rust-lang.org/std/fs/struct.File.html
    let mut _file = File::create("screenshot.png")?;
    _file.write_all(&_screenshot)?;

    // println!("Screenshot of browser windows => {:?} ",screenshot);
    Ok(())
}

// https://github.com/stevepryde/thirtyfour/issues/4?ref=https://githubhelp.com
//
async fn wait_seconds_of_browser(
    _driver: WebDriver,
    waiting_period: u64,
) -> color_eyre::Result<(), Box<dyn Error>> {
    debug!("wait for page completed load => wait for status from chrome driver");
    debug!("driver=> {:?}", _driver.status().await?);
    debug!("Thread sleep for {} seconds", waiting_period);
    thread::sleep(Duration::from_secs(waiting_period));
    Ok(())
}

// 25 year
// https://www.macrotrends.net/stocks/charts/TREX/trex/stock-price-history
// sec data
// https://www.sec.gov/cgi-bin/viewer?action=view&cik=1069878&accession_number=0001193125-23-266276&xbrl_type=v#

#[allow(dead_code)]
const TABS_OF_DATA: &[&[&str]] = &[
    //No.,FieldName,site xpath,table xpath
    &[
        "t1",
        "prices",
        "/html/body/div[3]/div[3]/div[1]/div[9]/table",
    ],
    &["t2", "financial", ""],
    &[
        "t3",
        "revenue",
        "/html/body/div[3]/div[3]/div[1]/div[8]/div[1]/table",
    ],
];

// FOUND HERE
// https://itehax.com/blog/web-scraping-using-rust
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

// FROM HERE
// https://users.rust-lang.org/t/how-to-print-the-type-of-a-variable/101947/2
#[allow(dead_code)]
fn print_type<T>(_: &T) {
    debug!("Type is => {}", std::any::type_name::<T>());
}

#[allow(dead_code)]
async fn tag_list_all_childes(
    _driver: WebDriver,
    #[allow(unused_variables)] xpath: &str,
) -> Result<(), Box<dyn Error>> {
    debug!("driver => {:?}", _driver.status().await?);

    let child_elems = _driver
        //.find_all(By::XPath("/*/*/X/node()"))
        .find_all(By::XPath("*"))
        .await?;

    for child_elem in child_elems {
        // extract string out of result
        let _tag_name = match child_elem.tag_name().await {
            Ok(x) => x,
            Err(_e) => continue,
        };
        debug!("\tlist_iframe_tag => tag_name =>  {}", _tag_name);

        let _result_tag_class_name: WebDriverResult<Option<String>> = child_elem.class_name().await;
        let _tag_class_name = match _result_tag_class_name {
            Ok(tag_class_name) => tag_class_name,
            Err(_e) => continue,
        };
        debug!("_tag_class_name => {:?}", _tag_class_name);
    }

    // debug!("\t go back to call fn ");

    Ok(())
}

/*
rustfmt  ./examples/thirtyfour_simple

cargo build --example thirtyfour_simple
*/
