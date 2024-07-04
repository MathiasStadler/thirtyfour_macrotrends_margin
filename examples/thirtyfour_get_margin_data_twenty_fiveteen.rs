// RUN PRG /W full log output
// RUST_LOG=debug cargo run --example thirtyfour_get_margin_data_twenty_four 2>&1 | tee output1.txt

// start cleanup

#![warn(unused_extern_crates)]
#[warn(dead_code)]
// RUSTFLAGS=-Wunused-crate-dependencies cargo

// env_logger
// RUST_LOG=info ./main

// thirtyfour 2024
// https://www.zenrows.com/blog/rust-selenium#install-selenium
#[allow(unused_imports)]
use log::{debug, error, info, log_enabled, Level};

#[allow(unused_imports)]
use async_recursion::async_recursion;

// check maybe double
// #[allow(unused_imports)]
// use thirtyfour::error::WebDriverResult;

use csv::WriterBuilder;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::process;
use std::thread;
use std::time::Duration;

use thirtyfour::{prelude::WebDriverError, By, DesiredCapabilities, Key, WebDriver, WebElement};

const WEB_PAGE: &str = "https://www.macrotrends.net";
//const STOCK_SYMBOL: &str = "TREX";
const STOCK_SYMBOL: &str = "CROX";

const ACTION_CLICK_INTERACTABLE: &str = "action_click_interactable";
const ACTION_CLICK: &str = "action_click";

// const ACTION_FORM_CLICK_SELECTION_FIELD: &str ="action_form_click_selection_field";
const ACTION_FORM_FILL_FIELD_WITH_SELECT: &str = "action_form_fill_field_with_select";
const ACTION_FORM_FILL_FIELD: &str = "action_form_fill_field";
const ACTION_SCREENSHOT_WEB_ELEMENT: &str = "screenshot_web_element";
const ACTION_ENTER_FRAME: &str = "action_enter_frame";

const WEB_XPATH: &[&[&str]] = &[
    //No.,Action,FieldName,xpath
    &[
        "1",
        ACTION_FORM_FILL_FIELD_WITH_SELECT,
        STOCK_SYMBOL,
        "/html/body/div[1]/div[2]/div[2]/div[2]/div/form/div[1]/div[1]/span[1]/input",
    ],
    &[
        "2",
        ACTION_CLICK,
        "revenue",
        "/html/body/div[1]/div[2]/div[2]/div[2]/div/form/div[1]/div[2]/ul/li[1]/a/span",
    ],
    &[
        "3",
        ACTION_CLICK_INTERACTABLE,
        "click",
        "/html/body/div[9]/div[1]/div[1]/div/button",
    ],
    &[
        "4",
        ACTION_CLICK,
        "click",
        "/html/body/div[3]/div[3]/div[1]/div[1]/ul[1]/li[1]/a",
    ],
    &["5", ACTION_ENTER_FRAME, "enter_frame", "should empty"],
    // &["5",ACTION_CLICK,"click","/html/body/div[3]/div[3]/div[1]/div[1]/ul[1]/li[1]/a"],
];

pub type WebDriverResult<T> = Result<T, WebDriverError>;

fn main() -> color_eyre::Result<(), Box<dyn Error>> {
    color_eyre::install()?;

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
    let _place: &str = "Place";
    let _driver = initialize_driver().await?;

    _driver.goto(WEB_PAGE).await?;
    thread::sleep(Duration::from_secs(4));

    path_to(_driver.clone()).await?;
    save_table_to_file_first(_driver.clone()).await?;
    // close_browser(_driver.clone()).await?;

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

async fn screenshot_web_element(
    web_element: WebElement,
    screenshot_name: &str,
) -> color_eyre::Result<(), Box<dyn Error>> {
    // FROM HERE screenshot of browser windows
    // https://stackoverflow.com/questions/60999624/trying-to-take-and-save-a-screenshot-of-a-specific-element-selenium-python-ch

    let _screenshot = web_element.screenshot_as_png().await?;
    let mut _file = File::create(screenshot_name)?;
    _file.write_all(&_screenshot)?;

    info!("Screenshot of browser windows => {:?} ", _file);
    Ok(())
}
// https://github.com/stevepryde/thirtyfour/issues/4?ref=https://githubhelp.com
//
async fn wait_seconds_of_browser(
    _driver: WebDriver,
    waiting_period: u64,
) -> color_eyre::Result<(), Box<dyn Error>> {
    // wait for page already load
    debug!("driver=> {:?}", _driver.status().await?);
    debug!("Thread sleep for {} seconds", waiting_period);
    thread::sleep(Duration::from_secs(waiting_period));
    Ok(())
}

async fn path_to(_driver: WebDriver) -> color_eyre::Result<(), Box<dyn Error>> {
    info!("method START => path_to");

    wait_seconds_of_browser(_driver.clone(), 10).await?;

    debug!("XPATH => Browser steps {}", WEB_XPATH.len());

    for field in 0..WEB_XPATH.len() {
        debug!("Next field No.: => {}", WEB_XPATH[field][0]);
        debug!("\tAction => {}", WEB_XPATH[field][1]);
        debug!("\tField => {}", WEB_XPATH[field][2]);

        // https://stackoverflow.com/questions/45183797/element-not-interactable-exception-in-selenium-web-automation
        if ACTION_CLICK_INTERACTABLE == WEB_XPATH[field][1] {
            debug!(
                "Action START =>  ACTION_CLICK_INTERACTABLE ({})",
                WEB_XPATH[field][1]
            );
            wait_seconds_of_browser(_driver.clone(), 10).await?;
            let elem_form: WebElement = _driver.find(By::XPath(WEB_XPATH[field][3])).await?;
            elem_form.click().await?;
            // wait_seconds_of_browser(_driver.clone(), 10).await?;
            debug!(
                "Action FINISH =>  ACTION_CLICK_INTERACTABLE ({})",
                WEB_XPATH[field][1]
            );
        } else if ACTION_CLICK == WEB_XPATH[field][1] {
            debug!("Action START =>  ACTION_CLICK ({})", WEB_XPATH[field][1]);

            wait_seconds_of_browser(_driver.clone(), 5).await?;

            let elem_form_result: Result<WebElement, WebDriverError> =
                _driver.find(By::XPath(WEB_XPATH[field][3])).await;

            let elem_form = match elem_form_result {
                Ok(_web_element) => {
                    debug!(r#"ACTION_CLICK => web_element found"#);
                    _web_element
                }
                Err(e) => {
                    debug!(r#"Error web_element found"#);
                    eprintln!("Error {}", e);
                    continue;
                }
            };

            //click web element
            elem_form.click().await?;
            debug!("Action FINISH =>  ACTION_CLICK ({})", WEB_XPATH[field][1]);
        } else if ACTION_FORM_FILL_FIELD == WEB_XPATH[field][1] {
            debug!(
                "Action START =>  ACTION_FORM_FILL_FIELD ({})",
                WEB_XPATH[field][1]
            );

            let elem_form: WebElement = _driver.find(By::XPath(WEB_XPATH[field][3])).await?;
            debug!("\tDEBUG => send_keys {}", WEB_XPATH[field][2]);
            elem_form.send_keys(WEB_XPATH[field][2]).await?;
            debug!("\tDEBUG => press enter");
            elem_form.send_keys(Key::Enter).await?;
            wait_seconds_of_browser(_driver.clone(), 5).await?;
            debug!(
                "Action FINISHED =>  ACTION_FORM_FILL_FIELD ({})",
                WEB_XPATH[field][1]
            );
        } else if ACTION_FORM_FILL_FIELD_WITH_SELECT == WEB_XPATH[field][1] {
            debug!(
                "Action START =>  ACTION_FORM_FILL_FIELD ({})",
                WEB_XPATH[field][1]
            );
            let elem_form: WebElement = _driver.find(By::XPath(WEB_XPATH[field][3])).await?;
            debug!("\t send_keys {}", WEB_XPATH[field][2]);
            elem_form.send_keys(WEB_XPATH[field][2]).await?;
            debug!("\t select field");
            // elem_form.send_keys(Key::Enter).await?;
            debug!("\t press enter");

            wait_seconds_of_browser(_driver.clone(), 5).await?;
            debug!(
                "Action FINISH =>  ACTION_FORM_FILL_FIELD ({})",
                WEB_XPATH[field][1]
            );
        } else if ACTION_FORM_FILL_FIELD_WITH_SELECT == WEB_XPATH[field][1] {
            // empty
        } else if ACTION_SCREENSHOT_WEB_ELEMENT == WEB_XPATH[field][1] {
            debug!(
                "Action START =>  ACTION_SCREENSHOT_WEB_ELEMENT ({})",
                WEB_XPATH[field][1]
            );
            wait_seconds_of_browser(_driver.clone(), 5).await?;
            let _web_element: WebElement = _driver.find(By::XPath(WEB_XPATH[field][3])).await?;
            let _screenshot_name: &str = WEB_XPATH[field][2];
            screenshot_web_element(_web_element, _screenshot_name).await?;
            debug!(
                "Finish START =>  ACTION_SCREENSHOT_WEB_ELEMENT ({})",
                WEB_XPATH[field][1]
            );
        }
        // ACTION_ENTER_FRAME
        else if ACTION_ENTER_FRAME == WEB_XPATH[field][1] {
            debug!(
                "Action START =>  ACTION_ENTER_FRAME ({})",
                WEB_XPATH[field][1]
            );
            let _elem_iframe_result = _driver.find(By::XPath("/html/ins/div/iframe"));

            debug!("Start - check iframe element is available");
            let _elem_iframe: WebElement = match _elem_iframe_result.await {
                Ok(_iframe) => {
                    // enter into frame
                    debug!("Enter  into frame => pre step ");
                    let result_enter_frame = _iframe.clone().enter_frame().await;
                    //enter frame
                    // debug!("HERE => ENTER FRAME {} ",)
                    let _enter_frame = match result_enter_frame {
                        Ok(_iframe) => {
                            debug!("ENTER FRAME => first step after enter frame ");

                            // show all element inside frame
                            let child_elems = _driver.find_all(By::XPath("//*[//*]")).await?;
                            // let child_elems = _driver.find_all(By::XPath("//iframe")).await?;

                            // let child_elems = _driver.find_all(By::XPath("//iframe[//*][//*]")).await?;
                            // let child_elems =
                            //     _driver.find_all(By::XPath(".//*[//*]")).await?;
                            // //

                            debug!("HERE inside frame ");
                            let mut _counter = 0;
                            for child_elem in child_elems {
                                _counter = _counter + 1;

                                let _ = list_element(child_elem.clone()).await?;

                                let tag_name = child_elem.tag_name().await?;
                                debug!("Nr => {}", _counter);
                                debug!("\t  tag_name =>  {}", tag_name);
                                debug!("\t tag_name => {} <= ", tag_name);

                                // extract string out of result
                                let _tag_name = match child_elem.tag_name().await {
                                    Ok(x) => x,
                                    Err(_e) => continue,
                                };
                                debug!("\t HERE iframe=> tag_name =>  {}", _tag_name);

                                // extract text out of result
                                let _tag_text = match child_elem.text().await {
                                    Ok(x) => x,
                                    Err(_e) => continue,
                                };
                                debug!("\t HERE iframe=> tag_ text =>  {}", _tag_text);

                                //call fn tag_list_all_childes
                                // let _ = tag_list_all_childes(_driver.clone(), "xpath").await?;
                                // debug!(
                                //     "\tfinished tag_list_all_childes => tag_name =>  {}",
                                //     tag_name
                                // );
                            }

                            // execute script
                            // from here
                            // https://github.com/ultrafunkamsterdam/undetected-chromedriver/issues/144

                            // get generate webpage source by javascript
                            debug!("START execute script => get akt page source");
                            let _real_html = _driver
                                //.execute_script(
                                .execute(
                                    r#"return document.getElementsByTagName('html')[0].innerHTML"#,
                                    Vec::new(),
                                )
                                .await
                                .unwrap();

                            //HERE real_html
                            debug!("real html => {:?}", _real_html);
                            debug!("FINISHED execute script => get akt page source");

                            // HERE 338
                            // let child_elems = _driver.find_all(By::XPath(".//child::*[//*]//*")).await?;
                            //let child_elems = _driver.find_all(By::XPath("/*[//*]//*")).await?;
                            // let child_elems = _driver.find_all(By::XPath(".//child::*")).await?;
                            // let child_elems = _driver.find_all(By::XPath(".//child::*[contains(text(),'Close']")).await?;
                            let child_elems = _driver
                                .find_all(By::XPath("//*[text()[contains(.,'Close')]]"))
                                .await?;

                            // len - How many WebElement had we found?
                            debug!("found children n = {:?} ", child_elems.len());

                            let mut counter = 0;
                            for child_elem in &child_elems {
                                counter = counter + 1;
                                // debug!("sub Tag => {}", counter);
                                let _ = list_element(child_elem.clone()).await?;
                                // extract string out of result
                                let _tag_name = match child_elem.tag_name().await {
                                    Ok(x) => x,
                                    Err(_e) => continue,
                                };
                                debug!("\t HERE iframe=> tag_name =>  {}", _tag_name);

                                // extract text out of result
                                let _tag_text = match child_elem.text().await {
                                    Ok(x) => x,
                                    Err(_e) => continue,
                                };
                                debug!("\t HERE iframe=> tag_ text =>  {}", _tag_text);

                                //div
                                if _tag_name == "div" {
                                    let sub_child_elems =
                                        child_elem.find_all(By::XPath(".//child::*")).await?;

                                    debug!("START child element");
                                    //
                                    //reset counter to zero
                                    counter = 0;
                                    for sub_child_elem in sub_child_elems {
                                        counter = counter + 1;

                                        // extract string out of result
                                        let _sub_tag_name = match sub_child_elem.tag_name().await {
                                            Ok(x) => x,
                                            Err(_e) => continue,
                                        };

                                        // extract id out of result
                                        let _sub_tag_id = match sub_child_elem.id().await {
                                            Ok(x) => x,
                                            Err(_e) => continue,
                                        };

                                        // extract class name out of result
                                        let _sub_tag_class = match sub_child_elem.class_name().await
                                        {
                                            Ok(x) => x,
                                            Err(_e) => continue,
                                        };

                                        // extract text out of result
                                        let _sub_tag_text = match sub_child_elem.text().await {
                                            Ok(x) => x,
                                            Err(_e) => continue,
                                        };
                                        // debug!("\t_sub_tag_text =>  {:?}", _sub_tag_text);

                                        debug!(
                                            "n=>{:?} tag=>{:?} id=>{:?} text=>{:?} class=>{:?}",
                                            counter,
                                            _sub_tag_name,
                                            _sub_tag_id,
                                            _sub_tag_text,
                                            _sub_tag_class
                                        );

                                        // search contain
                                        // let _child_elems = sub_child_elem.find_all(By::XPath(".//child::*[//*]")).await?;

                                        let _child_elems = match sub_child_elem
                                            .find_all(By::XPath(".//child::*[//*]"))
                                            .await
                                        {
                                            Ok(x) => x,
                                            Err(_e) => {
                                                eprintln!("Err {}", _e);
                                                continue;
                                            }
                                        };

                                        // len - How many WebElement had we found?
                                        debug!("n = {:?} ", _child_elems.len());

                                        if _child_elems.len() > 0 {
                                            // debug_vec(_child_elems,_sub_tag_name);
                                            debug!("call debug_vec ");
                                            debug_vec(_child_elems.clone(), _sub_tag_name.clone(),"child_elems")
                                                .await?;
                                        };

                                        const XPATH_SEARCH: &str = ".//child::*[//*]";

                                        debug!("search children");
                                        let _search_child_elems = match sub_child_elem
                                            .find_all(By::XPath(XPATH_SEARCH))
                                            .await
                                        {
                                            Ok(x) => x,
                                            Err(_e) => {
                                                eprintln!("Err {}", _e);
                                                continue;
                                            }
                                        };

                                        if _search_child_elems.len() == 0 {
                                            debug!("NO search element FOUND");
                                        } else {
                                            debug!(
                                                "Found n=>{:?} (_search_child_elements) =  ",
                                                _search_child_elems.len()
                                            );
                                        }
                                    }
                                } //end div
                                let _ = list_element(child_elem.clone()).await?;

                                let _text = child_elem.text().await?;
                                // debug!("text => {}", _text);

                                // extract text inside span
                                let _tag_name = match child_elem.tag_name().await {
                                    Ok(x) => x,
                                    Err(_e) => continue,
                                };

                                // extract text inside span
                                let _tag_text = match child_elem.text().await {
                                    Ok(x) => x,
                                    Err(_e) => continue,
                                };

                                // extract id inside span
                                let _tag_id = match child_elem.id().await {
                                    Ok(x) => x,
                                    Err(_e) => continue,
                                };

                                let _result_tag_class_name: WebDriverResult<Option<String>> =
                                    child_elem.class_name().await;

                                let _tag_class_name = match _result_tag_class_name {
                                    Ok(tag_class_name) => tag_class_name,
                                    Err(_e) => continue,
                                };

                                debug!("inside iframe: tag => {:?} ( id=> {:?} text=>{:?} class=>{:?} )",_tag_name,_tag_id,_tag_text,_tag_class_name);

                                // let _ = list_element(child_elem).await?;
                                // let _ = list_element(child_elem.clone()).await?;
                            }

                            debug!("\t FINISHED show all elements inside iframe");
                        }
                        Err(_e) => {
                            error!("Failed enter frame!");
                            panic!("Failed enter frame");
                        }
                    };

                    debug!("iframe _driver.close().await?");
                    _iframe
                }

                Err(_e) => {
                    error!("Failed enter frame!");
                    panic!("Failed enter frame");
                }
            };

            wait_seconds_of_browser(_driver.clone(), 10).await?;
        }
    }

    wait_seconds_of_browser(_driver.clone(), 10).await?;

    Ok(())
}

async fn save_table_to_file_first(_driver: WebDriver) -> color_eyre::Result<(), Box<dyn Error>> {
    info!("START save_table_to_file_first");
    // 1st table left side
    const OUTPUT_FILE_NAME_ONE: &str = "output_TREX.csv";

    const TABLE_XPATH_ONE: &[&[&str]] = &[
        //No.,FieldName,xpath
        &[
            "t1",
            "colum_name",
            "/html/body/div[3]/div[3]/div[1]/div[9]/table/thead[2]/tr",
            "/html/body/div[3]/div[3]/div[1]/div[9]/table/thead[2]/tr/th[1]",
        ],
        // /html/body/div[3]/div[3]/div[1]/div[9]/table/thead[2]/tr/th[1]
        // /html/body/div[3]/div[3]/div[1]/div[9]/table/thead[2]/tr/th[1]
        &[
            "t2",
            "No.:",
            "/html/body/div[3]/div[3]/div[1]/div[9]/table/tbody/tr",
        ],
        // /html/body/div[3]/div[3]/div[1]/div[9]/table/tbody
    ];

    let _ = save_table_to_file_worker(_driver.clone(), OUTPUT_FILE_NAME_ONE, TABLE_XPATH_ONE).await;

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

#[allow(dead_code)]
async fn save_table_to_file(_driver: WebDriver) -> color_eyre::Result<(), Box<dyn Error>> {
    // switch to tab
    Ok(())
}

async fn save_table_to_file_worker(
    _driver: WebDriver,
    output_file_name: &str,
    table_xpath: &[&[&str]],
) -> color_eyre::Result<(), Box<dyn Error>> {
    let mut field = 0;

    debug!("start - save_table_to_file_worker");
    let mut wtr = WriterBuilder::new()
        .flexible(true)
        .has_headers(false)
        .from_writer(vec![]);

    // debug
    debug!("table No.   => {}", table_xpath[field][0]);
    debug!("table Field => {}", table_xpath[field][1]);
    debug!("table XPath => {}", table_xpath[field][2]);

    let thead_rows_vec: Vec<WebElement> = _driver.find_all(By::XPath(table_xpath[0][2])).await?;

    debug!("table thead_rows_vec len => {:?}", thead_rows_vec.len());

    let mut row = 0;

    for thead_row in thead_rows_vec {
        let thead_cell_vec: Vec<WebElement> = thead_row.find_all(By::XPath("th")).await?;

        debug!(
            "table THEAD thead_cell_vec len => {:?}",
            thead_cell_vec.len()
        );

        let mut column = 0;
        for thead_cell in thead_cell_vec {
            column = column + 1;
            let cell_text = thead_cell.text().await?;
            debug!("write_field row/column {}/{} => {}", row, column, cell_text);
            wtr.write_field(cell_text)?;
        } //finish inner for loop => thead_cell

        debug!("DEBUG: write_record");
        let _ = &wtr.write_record(None::<&[u8]>)?;

        field = 1;

        // debug
        debug!("No.   => {}", table_xpath[field][0]);
        debug!("Field => {}", table_xpath[field][1]);
        debug!("XPath => {}", table_xpath[field][2]);

        let tbody_row_vec: Vec<WebElement> =
            _driver.find_all(By::XPath(table_xpath[field][2])).await?;

        debug!(
            "table TBODY tbody_cell_vec len => {:?}",
            tbody_row_vec.len()
        );

        row = 0;
        for tbody_row in tbody_row_vec {
            row = row + 1;

            let tbody_cell_vec: Vec<WebElement> = tbody_row.find_all(By::XPath("td")).await?;
            let mut column = 0;
            for tbody_cell in tbody_cell_vec {
                column = column + 1;
                let cell_text = tbody_cell.text().await?;
                debug!(
                    "DEBUG: write_field row/column {}/{} => {}",
                    row, column, cell_text
                );
                wtr.write_field(cell_text)?;
            } //finish inner for loop => tbody_cell

            debug!("tbody: write_record");

            // don't use result
            let _ = &wtr.write_record(None::<&[u8]>)?;
        } //finish for loop => tbody_row
    } //finish for loop => thead_row

    info!("csv file name => {}", output_file_name);
    let mut file = File::create(output_file_name)?;
    file.write_all(&wtr.into_inner()?)?;

    Ok(())
}

// FOUND HERE
// https://itehax.com/blog/web-scraping-using-rust
async fn initialize_driver() -> Result<WebDriver, WebDriverError> {
    info!("initialize_driver - start");

    let _caps = DesiredCapabilities::chrome();

    // let mut caps: thirtyfour::ChromeCapabilities = DesiredCapabilities::chrome();
    // caps.add_chrome_arg("--headless")?;
    // caps.add_chrome_arg("--no-sandbox")?;
    //  caps.add_chrome_arg("--disable-dev-shm-usage")?;

    let driver = WebDriver::new("http://localhost:9515", _caps).await?;
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
        .find_all(By::XPath(".//span[normalize-space(text())=\"Close\"]/.."))
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


#[async_recursion]
async fn debug_vec(
    _child_elems: Vec<WebElement>,
    _sub_tag_name: String,
    _note:&str,
) -> color_eyre::Result<(), Box<dyn Error>> {
    
    debug!("START debug_vec");

    for _child_elem in _child_elems {
        debug!(
            "_parent/child => {} => {}/{} (class_name=>{:?} id=>{:?} text=>{:?})",
            _note,
            _sub_tag_name,
            _child_elem.tag_name().await?,
            _child_elem.class_name().await?,
            _child_elem.id().await?,
            _child_elem.text().await?,
        );
       // let _child_elems= _child_elem.find_all(By::XPath(".//*")).await?;
        let _child_elems= _child_elem.find_all(By::XPath("./*")).await?;
        
        if _child_elems.len() > 0{
            debug!("child elems tag {} n=>{}",_sub_tag_name,_child_elems.len());

          //  debug_vec(_child_elems.clone(), _sub_tag_name.clone(),"sub_child_elems")
          //  .await?;
        };
    }

    debug!("FINISHED debug_vec");
    Ok(())
}

async fn list_element(child_elem: WebElement) -> color_eyre::Result<(), Box<dyn Error>> {
    #[allow(unused_assignments)]
    let mut counter = 0;
    //div
    // if _tag_name == "div" {
    if true {
        let sub_child_elems = child_elem.find_all(By::XPath(".//child::*")).await?;

        debug!("START child element");
        //
        //reset counter to zero
        counter = 0;
        for sub_child_elem in sub_child_elems {
            counter = counter + 1;

            // extract string out of result
            let _sub_tag_name = match sub_child_elem.tag_name().await {
                Ok(x) => x,
                Err(_e) => continue,
            };

            // extract id out of result
            let _sub_tag_id = match sub_child_elem.id().await {
                Ok(x) => x,
                Err(_e) => continue,
            };

            // extract class name out of result
            let _sub_tag_class = match sub_child_elem.class_name().await {
                Ok(x) => x,
                Err(_e) => continue,
            };

            // extract text out of result
            let _sub_tag_text = match sub_child_elem.text().await {
                Ok(x) => x,
                Err(_e) => continue,
            };
            // debug!("\t_sub_tag_text =>  {:?}", _sub_tag_text);

            debug!(
                "n=>{:?} tag=>{:?} id=>{:?} text=>{:?} class=>{:?}",
                counter, _sub_tag_name, _sub_tag_id, _sub_tag_text, _sub_tag_class
            );

            // search contain
            // let _child_elems = sub_child_elem.find_all(By::XPath(".//child::*[//*]")).await?;

            let _child_elems = match sub_child_elem.find_all(By::XPath(".//child::*[//*]")).await {
                Ok(x) => x,
                Err(_e) => {
                    eprintln!("Err {}", _e);
                    continue;
                }
            };

            // len - How many WebElement had we found?
            debug!("n = {:?} ", _child_elems.len());

            if _child_elems.len() > 0 {
                debug!("call debug_vec ");
                // debug_vec(_child_elems,_sub_tag_name);
                // debug_vec(_child_elems.clone(), _sub_tag_name.clone()).await?;
                debug_vec(_child_elems.clone(), _sub_tag_name.clone(),"child_elems")
                                                .await?;
            };

            const XPATH_SEARCH: &str = ".//child::*[//*]";

            debug!("search children");
            let _search_child_elems = match sub_child_elem.find_all(By::XPath(XPATH_SEARCH)).await {
                Ok(x) => x,
                Err(_e) => {
                    eprintln!("Err {}", _e);
                    continue;
                }
            };

            /*
            if _search_child_elems.len() == 0 {
                debug!("NO search element FOUND");
            } else {
                debug!(
                    "Found n=>{:?} (_search_child_elements) =  ",
                    _search_child_elems.len()
                );
            }
            */

            debug!("call debug_vec ");
                // debug_vec(_child_elems,_sub_tag_name);
                // debug_vec(_search_child_elems.clone(), _sub_tag_name.clone()).await?;
                debug_vec(_child_elems.clone(), _sub_tag_name.clone(),"call debug_vec")
                                                .await?;


        }
    } //end div
    Ok(())
}

/*
rustfmt  ./examples/tokio_finviz_method_five.rs
*/
