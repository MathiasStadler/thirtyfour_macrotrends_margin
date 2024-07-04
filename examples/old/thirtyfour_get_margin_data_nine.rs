#![warn(unused_extern_crates)]
#[warn(dead_code)]

// RUSTFLAGS=-Wunused-crate-dependencies cargo 

// env_logger
// RUST_LOG=info ./main

// thirtyfour 2024
// https://www.zenrows.com/blog/rust-selenium#install-selenium

// use log::info;
// #[allow(unused_imports)]
// use log::{debug, error, log_enabled, info, Level,LevelFilter};
// #[allow(unused_imports)]
// use env_logger::Builder;
// use log::LevelFilter;

#[allow(unused_imports)]
use log::{debug, error, log_enabled, info, Level};

use csv::WriterBuilder;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::process;
use std::thread;
use std::time::Duration;

use thirtyfour::{
    By,
    DesiredCapabilities,
    Key,
    prelude::WebDriverError,
    WebDriver,
    WebElement,
};



const WEB_PAGE: &str = "https://www.macrotrends.net";
//const STOCK_SYMBOL: &str = "TREX";
const STOCK_SYMBOL: &str = "CROX";


const ACTION_CLICK_INTERACTABLE: &str ="action_click_interactable";
const ACTION_CLICK: &str = "action_click";
const ACTION_FORM_CLICK_SELECTION_FIELD: &str ="action_form_click_selection_field";
const ACTION_FORM_FILL_FIELD_WITH_SELECT: &str ="action_form_fill_field_with_select";
const ACTION_FORM_FILL_FIELD: &str = "action_form_fill_field";
const ACTION_SCREENSHOT_WEB_ELEMENT: &str = "screenshot_web_element";
const ACTION_ENTER_FRAME: &str = "action_enter_frame";


const WEB_XPATH: &[&[&str]] = &[
    //No.,Action,FieldName,xpath
    // static stock symbol
    //&["1",ACTION_FORM_FILL_FIELD_WITH_SELECT,"TREX","/html/body/div[1]/div[2]/div[2]/div[2]/div/form/div[1]/div[1]/span[1]/input"],
    // const stock symbol
    &["1",ACTION_FORM_FILL_FIELD_WITH_SELECT,STOCK_SYMBOL,"/html/body/div[1]/div[2]/div[2]/div[2]/div/form/div[1]/div[1]/span[1]/input"],
    &["2",ACTION_CLICK,"revenue","/html/body/div[1]/div[2]/div[2]/div[2]/div/form/div[1]/div[2]/ul/li[1]/a/span"],
    &["3",ACTION_CLICK_INTERACTABLE,"click","/html/body/div[9]/div[1]/div[1]/div/button"],
    &["4",ACTION_CLICK,"click","/html/body/div[3]/div[3]/div[1]/div[1]/ul[1]/li[1]/a"],
    // &["5",ACTION_CLICK,"click","/html/body/div/div/div[1]/div[4]"],
    // &["6",ACTION_CLICK,"click","/html/body/div[1]/div[2]/div[2]/div/div/div[2]/div/div/div[3]"],
    &["5",ACTION_ENTER_FRAME,"enter_frame","should empty"],
    ];

    // /html/body/div/div/div[1]/div[4]

// run with Log
// RUST_LOG=debug cargo run --example thirtyfour_get_margin_data_six

// #[cfg(not(all(feature = "color", feature = "humantime")))]
fn main() -> color_eyre::Result<(), Box<dyn Error>> {
    
    color_eyre::install()?;

    // env_logger::init();

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

    
    //test env_logger
    info!("env_logger: starting up");
    debug!("this is a debug {}", "message");
    error!("this is printed by default");

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
    // thread::sleep(Duration::from_secs(2));

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

async fn screenshot_web_element (
    web_element: WebElement,
    screenshot_name: &str,
) -> color_eyre::Result<(), Box<dyn Error>> {
    
    // FROM HERE screenshot of browser windows
    // https://stackoverflow.com/questions/60999624/trying-to-take-and-save-a-screenshot-of-a-specific-element-selenium-python-ch
    
    let _screenshot = web_element.screenshot_as_png().await?;

    // FROM HERE  write to file
    // https://doc.rust-lang.org/std/fs/struct.File.html

    let mut _file = File::create(screenshot_name)?;
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
    // wait for page already load
    debug!("Status driver => {:?}",  _driver.status().await?);
    //old
    // println!("Status driver => {:?}", _driver.status().await?);
    
    // show line number
    // let current_line = line!();
    // println!("defined on line: {current_line}");
    
    thread::sleep(Duration::from_secs(waiting_period));
    Ok(())
}
#[allow(dead_code)]
async fn check_of_ad_iframe(_driver: WebDriver) -> color_eyre::Result<(), Box<dyn Error>>{

    // https://github.com/stevepryde/thirtyfour/blob/main/thirtyfour/examples/query/wikipedia.rs
    // / Find element from element using multiple selectors.

//c.find(By::Css("#iframe_page_id")).await?.click().await?;

_driver.find(By::Css("#ad_iframe")).await?.click().await?;

// c.find(By::Id("iframe_button")).await.expect_err("should not find the button in the iframe");
// c.find(By::Id("root_button")).await?; // Can find the button in the root context though.

Ok(())
}

async fn path_to(_driver: WebDriver) -> color_eyre::Result<(), Box<dyn Error>> {
    info!("method start => path_to");
    // wait browser already load
    // ElementWaitable
    wait_seconds_of_browser(_driver.clone(), 10).await?;

// println!("{}",WEB_XPATH.len());

    for field in 0..WEB_XPATH.len() {

        debug!("Next Field  => ");
       // println!("No.   => {}", WEB_XPATH[field][0]);
        debug!("Field No.: => {}",WEB_XPATH[field][0]);
    
       // println!("Action => {}", WEB_XPATH[field][1]);
        debug!("Action => {}",WEB_XPATH[field][1]);
        
    
        // println!("Field => {}", WEB_XPATH[field][2]);
        debug!("Field => {}",WEB_XPATH[field][2]);

        // Hint Format to long for console
        // debug!("Field No.: => {}, Action => {}, Field => {}",WEB_XPATH[field][0],WEB_XPATH[field][1],WEB_XPATH[field][2]);


        // https://stackoverflow.com/questions/45183797/element-not-interactable-exception-in-selenium-web-automation
        if ACTION_CLICK_INTERACTABLE == WEB_XPATH[field][1] {

            wait_seconds_of_browser(_driver.clone(), 20).await?;

            // println!("Action =>  ACTION_CLICK_INTERACTABLE ({})", WEB_XPATH[field][1]);
            debug!("Action =>  ACTION_CLICK_INTERACTABLE ({})", WEB_XPATH[field][1]);
            // let current_line = line!();
            // println!("defined on line: {current_line}");
                        
            let elem_form: WebElement = _driver.find(By::XPath(WEB_XPATH[field][3])).await?;
            elem_form.click().await?;
            wait_seconds_of_browser(_driver.clone(), 20).await?;
        }
        else if ACTION_CLICK == WEB_XPATH[field][1] {
            // println!("Action =>  ACTION_CLICK ({})", WEB_XPATH[field][1]);
            debug!("Action =>  ACTION_CLICK ({})", WEB_XPATH[field][1]);
                    
            
            wait_seconds_of_browser(_driver.clone(), 5).await?;

            let elem_form: WebElement = _driver.find(By::XPath(WEB_XPATH[field][3])).await?;
            
            elem_form.click().await?;
            
            wait_seconds_of_browser(_driver.clone(), 5).await?;
        
        } else if ACTION_FORM_FILL_FIELD == WEB_XPATH[field][1] {
            debug!(
                "Action =>  ACTION_FORM_FILL_FIELD ({})",
                WEB_XPATH[field][1]
            );
            // let current_line = line!();
            // println!("defined on line: {current_line}");
            let elem_form: WebElement = _driver.find(By::XPath(WEB_XPATH[field][3])).await?;
            debug!("DEBUG => send_keys {}",WEB_XPATH[field][2]);
            elem_form.send_keys(WEB_XPATH[field][2]).await?;
            debug!("DEBUG => press enter");
            elem_form.send_keys(Key::Enter).await?;
            wait_seconds_of_browser(_driver.clone(), 5).await?;
        } else if ACTION_FORM_FILL_FIELD_WITH_SELECT == WEB_XPATH[field][1] {
            debug!(
                "Action =>  ACTION_FORM_FILL_FIELD_WITH_SELECT ({})",
                WEB_XPATH[field][1]
            );
            
            let elem_form: WebElement = _driver.find(By::XPath(WEB_XPATH[field][3])).await?;
            debug!("send_keys {}",WEB_XPATH[field][2]);
            elem_form.send_keys(WEB_XPATH[field][2]).await?;
            debug!("select field");
            // elem_form.send_keys(Key::Enter).await?;
            debug!("press enter");
            
            
            wait_seconds_of_browser(_driver.clone(), 5).await?;
        } else if ACTION_FORM_CLICK_SELECTION_FIELD == WEB_XPATH[field][1] {
            // maybe double action click

        } else if ACTION_SCREENSHOT_WEB_ELEMENT == WEB_XPATH[field][1] {
            debug!(
                "Action =>  ACTION_FORM_FILL_FIELD ({})",
                WEB_XPATH[field][1]
            );
            
            wait_seconds_of_browser(_driver.clone(), 5).await?;
            let _web_element: WebElement = _driver.find(By::XPath(WEB_XPATH[field][3])).await?;
            let _screenshot_name: &str = WEB_XPATH[field][2];

            screenshot_web_element(_web_element, _screenshot_name).await?;
        } else if ACTION_ENTER_FRAME == WEB_XPATH[field][1] {
        
        info!("ACTION_ENTER_FRAME {}","insert");
        
        debug!("START: ACTION_ENTER_FRAME");
        
        // let _elem_iframe: WebElement = _driver.find(By::XPath("/html/ins/div/iframe")).await?;
        // FROM HERE
        // https://docs.rs/thirtyfour/latest/src/thirtyfour/webelement.rs.html#514-521
        // this works
        let _elem_iframe_result = _driver.find(By::XPath("/html/ins/div/iframe"));
        debug!("start - check iframe element is available");
        let _elem_iframe = match _elem_iframe_result.await{
            Ok(iframe) => {
                let _inside_frame = match iframe.enter_frame().await{
                    Ok(_inside_frame) => {
                        debug!("show all tag name inside frame");
                        // _driver.find(By::XPath("//child::*").await?;
                        // /* got one/first element=> DEBUG - Tag => html
                        // let child_elems = _driver.find_all(By::XPath("/*")).await?;
                        // //* got all elements
                        // let child_elems = _driver.find_all(By::XPath("//*")).await?;
                        // use only 2nd layer /*/node()
                        // let child_elems = _driver.find_all(By::XPath("/*/node()")).await?;
                        
                        // let child_elems = _driver.find_all(By::XPath("//*[@*]")).await?;
                        // let child_elems = _driver.find_all(By::XPath("//*[//*]")).await?;
                        let child_elems = _driver.find_all(By::XPath("//*[//*]")).await?;
                        for child_elem in child_elems {
                            
                            // assert_eq!(child_elem.tag_name().await?, "button");
                            let tag_name = child_elem.tag_name().await?;
                            
                            debug!("");
                            debug!("START tag_name {}",tag_name);

                            debug!("Tag name  (inside iframe)=> {}",tag_name);

                            let tag_id = child_elem.id().await?;
                            debug!("Tag id (inside iframe)=> {:?}",tag_id);

                            let tag_class_name = child_elem.class_name().await?;
                            debug!("Tag class name (inside iframe)=> {:?} ",tag_class_name);
                            
                            // let parent = child_elem.parent().await?;
                            // debug!("Tag parent (inside iframe)=> {:?} ",parent);

                            if tag_name == "span" {
                                debug!("match tag name <= {}",tag_name);
                                debug!("text => {}",child_elem.text().await?);

                                let outer_html = child_elem.outer_html().await?;
                                debug!("Tag outer_html (only span)=> {:?} ",outer_html);
                            }
                            
                            debug!("END tag_name {}",tag_name);
                            // debug!("");
                        }

                    },
                    Err(e) => {
                        debug!("Problem Not found the element: {:?}", e);
                },

                }; 
            },
            Err(e) => panic!("Problem Not found the element: {:?}", e),
        };
        
        // // wait for _driver 
        // wait_seconds_of_browser(_driver.clone(), 1).await?;

        // debug!("_elem_iframe => is available => ");
        // // debug!("iframe source {}",_driver.source().await?);
    
        // let _elem_button_close = _driver.find(By::XPath("//span[contains(.,'Close']"));
        // debug!("start -  button 6: is available and if click to close iframe");
        
        // let _elem_button_close = match _elem_button_close.await{
        //     Ok(iframe) =>iframe.click().await?,
        //     // Err(e) => panic!("Problem Not found the element: {:?}", e),
        //     // no panic!
        //     Err(e) => debug!("Error => Element NOT found: {:?}", e),
        // };

        debug!("_elem_button_close => click");

        debug!("leave the iframe - switch to frame(0)");
      //   _driver.switch_to().frame(0);

                

    
                

        debug!("FINISHED: ACTION_ENTER_FRAME");

    }
        else {
            error!("ERROR: ACTION NOT FOUND");
            process::exit(1);
            // error not found
        }
    }

    wait_seconds_of_browser(_driver.clone(), 20).await?;

    Ok(())
}

// https://stackoverflow.com/questions/73103205/how-to-load-a-irregular-csv-file-using-rust
// .flexible(true)
// .has_headers(false)
// https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=37d3c7c7eca04fbb17b83b90ef101a83

// 2.nd table
// /html/body/div[3]/div[3]/div[1]/div[8]/div[2]/table

async fn save_table_to_file_first(_driver: WebDriver) -> color_eyre::Result<(), Box<dyn Error>> {

    info!("START save_table_to_file_first");
    // 1st table left side
    const OUTPUT_FILE_NAME_ONE:&str = "output_TREX.csv";
    
    const TABLE_XPATH_ONE:&[&[&str]] = &[
         //No.,FieldName,xpath
         &["t1","colum_name","/html/body/div[3]/div[3]/div[1]/div[9]/table/thead[2]/tr"],
         // /html/body/div[3]/div[3]/div[1]/div[9]/table/thead[2]/tr/th[1]
         // /html/body/div[3]/div[3]/div[1]/div[9]/table/thead[2]/tr/th[1]
         &["t2","No.:",      "/html/body/div[3]/div[3]/div[1]/div[9]/table/tbody/tr"],
         // /html/body/div[3]/div[3]/div[1]/div[9]/table/tbody
        ];
        
        let _ = save_table_to_file_worker(_driver.clone(),OUTPUT_FILE_NAME_ONE,TABLE_XPATH_ONE).await;

// // 2nd table right side
// const OUTPUT_FILE_NAME_TWO:&str = "output_second.csv";

// const TABLE_XPATH_TWO:&[&[&str]] = &[
//      //No.,FieldName,xpath
//      &["t1","colum_name","/html/body/div[3]/div[3]/div[1]/div[8]/div[2]/table/thead/tr"],
//      &["t2","No.:",      "/html/body/div[3]/div[3]/div[1]/div[8]/div[2]/table/tbody/tr"],
//     ];
    
//     let _ = save_table_to_file_worker(_driver.clone(),OUTPUT_FILE_NAME_TWO,TABLE_XPATH_TWO).await;

 Ok(())
 }

// 25 year
// https://www.macrotrends.net/stocks/charts/TREX/trex/stock-price-history
// sec data
// https://www.sec.gov/cgi-bin/viewer?action=view&cik=1069878&accession_number=0001193125-23-266276&xbrl_type=v#

#[allow(dead_code)]
const TABS_OF_DATA:&[&[&str]] = &[
     //No.,FieldName,site xpath,table xpath
     &["t1","prices","/html/body/div[3]/div[3]/div[1]/div[1]/ul[1]/li[1]/a","/html/body/div[3]/div[3]/div[1]/div[9]/table"],
     &["t2","financial",""],
     &["t3","revenue","/html/body/div[3]/div[3]/div[1]/div[8]/div[1]/table"],
     

    ];

#[allow(dead_code)]
async fn save_table_to_file(_driver: WebDriver) -> color_eyre::Result<(), Box<dyn Error>> {

// switch to tab
Ok(())

}

async fn save_table_to_file_worker(_driver: WebDriver,output_file_name:&str,table_xpath:&[&[&str]]) -> color_eyre::Result<(), Box<dyn Error>> {
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

        debug!("table THEAD thead_cell_vec len => {:?}", thead_cell_vec.len());

        let mut column = 0;
        for thead_cell in thead_cell_vec {
            column = column + 1;
            let cell_text = thead_cell.text().await?;
            debug!(
                "write_field row/column {}/{} => {}",
                row, column, cell_text
            );
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

        
        debug!("table TBODY tbody_cell_vec len => {:?}", tbody_row_vec.len());
        
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

    info!("csv file name => {}",output_file_name);
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




/*
rustfmt  ./examples/tokio_finviz_method_five.rs
*/
