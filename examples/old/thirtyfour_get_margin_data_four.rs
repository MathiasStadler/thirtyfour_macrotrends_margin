#![warn(unused_extern_crates)]
#[warn(dead_code)]

// RUSTFLAGS=-Wunused-crate-dependencies cargo 

// env_logger
// RUST_LOG=info ./main
use log::info;

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
const STOCK_SYMBOL: &str = "TREX";

const ACTION_CLICK_INTERACTABLE: &str ="action_click_interactable";
const ACTION_CLICK: &str = "action_click";
const ACTION_FORM_CLICK_SELECTION_FIELD: &str ="action_form_click_selection_field";
const ACTION_FORM_FILL_FIELD_WITH_SELECT: &str ="action_form_fill_field_with_select";
const ACTION_FORM_FILL_FIELD: &str = "action_form_fill_field";
const ACTION_SCREENSHOT_WEB_ELEMENT: &str = "screenshot_web_element";


const WEB_XPATH: &[&[&str]] = &[
    //No.,Action,FieldName,xpath
    // static stock symbol
    //&["1",ACTION_FORM_FILL_FIELD_WITH_SELECT,"TREX","/html/body/div[1]/div[2]/div[2]/div[2]/div/form/div[1]/div[1]/span[1]/input"],
    // const stock symbol
    &["1",ACTION_FORM_FILL_FIELD_WITH_SELECT,STOCK_SYMBOL,"/html/body/div[1]/div[2]/div[2]/div[2]/div/form/div[1]/div[1]/span[1]/input"],
    &["2",ACTION_CLICK,"revenue","/html/body/div[1]/div[2]/div[2]/div[2]/div/form/div[1]/div[2]/ul/li[1]/a/span"],
    &["3",ACTION_CLICK_INTERACTABLE,"click","/html/body/div[9]/div[1]/div[1]/div/button"],
    &["4",ACTION_CLICK,"click","/html/body/div[3]/div[3]/div[1]/div[1]/ul[1]/li[1]/a"],
    &["5",ACTION_CLICK,"click","/html/body/div/div/div[1]/div[4]"],
    &["6",ACTION_CLICK,"click","/html/body/div[1]/div[2]/div[2]/div/div/div[2]/div/div/div[3]"],
    ];

    // /html/body/div/div/div[1]/div[4]

fn main() -> color_eyre::Result<(), Box<dyn Error>> {
    
    color_eyre::install()?;

    env_logger::init();

    info!("env_logger: starting up");

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
    thread::sleep(Duration::from_secs(2));
   
    thread::sleep(Duration::from_secs(2));

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
    println!("Status driver => {:?}", _driver.status().await?);
    
    // show line number
    // let current_line = line!();
    // println!("defined on line: {current_line}");
    
    thread::sleep(Duration::from_secs(waiting_period));
    Ok(())
}

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
    // wait browser already load
    // ElementWaitable
    wait_seconds_of_browser(_driver.clone(), 10).await?;

// println!("{}",WEB_XPATH.len());

    for field in 0..WEB_XPATH.len() {
        println!("No.   => {}", WEB_XPATH[field][0]);
        println!("Action => {}", WEB_XPATH[field][1]);
        println!("Field => {}", WEB_XPATH[field][2]);

        // https://stackoverflow.com/questions/45183797/element-not-interactable-exception-in-selenium-web-automation
        if ACTION_CLICK_INTERACTABLE == WEB_XPATH[field][1] {

            wait_seconds_of_browser(_driver.clone(), 20).await?;

            println!("Action =>  ACTION_CLICK_INTERACTABLE ({})", WEB_XPATH[field][1]);
            let current_line = line!();
            println!("defined on line: {current_line}");
                        
            let elem_form: WebElement = _driver.find(By::XPath(WEB_XPATH[field][3])).await?;
            elem_form.click().await?;
            wait_seconds_of_browser(_driver.clone(), 20).await?;
        }
        else if ACTION_CLICK == WEB_XPATH[field][1] {
            println!("Action =>  ACTION_CLICK ({})", WEB_XPATH[field][1]);
            
            let current_line = line!();
            
            println!("DEBUG:  code line: {current_line}");
            
            wait_seconds_of_browser(_driver.clone(), 5).await?;

            let elem_form: WebElement = _driver.find(By::XPath(WEB_XPATH[field][3])).await?;
            
            elem_form.click().await?;
            
            wait_seconds_of_browser(_driver.clone(), 5).await?;
        
        } else if ACTION_FORM_FILL_FIELD == WEB_XPATH[field][1] {
            println!(
                "Action =>  ACTION_FORM_FILL_FIELD ({})",
                WEB_XPATH[field][1]
            );
            let current_line = line!();
            println!("defined on line: {current_line}");
            let elem_form: WebElement = _driver.find(By::XPath(WEB_XPATH[field][3])).await?;
            println!("DEBUG => send_keys {}",WEB_XPATH[field][2]);
            elem_form.send_keys(WEB_XPATH[field][2]).await?;
            println!("DEBUG => press enter");
            elem_form.send_keys(Key::Enter).await?;
            wait_seconds_of_browser(_driver.clone(), 5).await?;
        } else if ACTION_FORM_FILL_FIELD_WITH_SELECT == WEB_XPATH[field][1] {
            println!(
                "Action =>  ACTION_FORM_FILL_FIELD_WITH_SELECT ({})",
                WEB_XPATH[field][1]
            );
            let current_line = line!();
            println!("defined on line: {current_line}");
            let elem_form: WebElement = _driver.find(By::XPath(WEB_XPATH[field][3])).await?;
            println!("DEBUG => send_keys {}",WEB_XPATH[field][2]);
            elem_form.send_keys(WEB_XPATH[field][2]).await?;
            println!("DEBUG => select field");
            // elem_form.send_keys(Key::Enter).await?;
            println!("DEBUG => press enter");
            
            
            wait_seconds_of_browser(_driver.clone(), 5).await?;
        } else if ACTION_FORM_CLICK_SELECTION_FIELD == WEB_XPATH[field][1] {
            // maybe double action click

        } else if ACTION_SCREENSHOT_WEB_ELEMENT == WEB_XPATH[field][1] {
            println!(
                "Action =>  ACTION_FORM_FILL_FIELD ({})",
                WEB_XPATH[field][1]
            );
            let current_line = line!();
            println!("defined on line: {current_line}");
            wait_seconds_of_browser(_driver.clone(), 5).await?;
            let _web_element: WebElement = _driver.find(By::XPath(WEB_XPATH[field][3])).await?;
            let _screenshot_name: &str = WEB_XPATH[field][2];

            screenshot_web_element(_web_element, _screenshot_name).await?;
        } else {
            println!("ACTION NOT FOUND");
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

    println!("START save_table_to_file_first");
    // 1st table left side
    const OUTPUT_FILE_NAME_ONE:&str = "output_TREX.csv";
    
    const TABLE_XPATH_ONE:&[&[&str]] = &[
         //No.,FieldName,xpath
         &["t1","colum_name","/html/body/div[3]/div[3]/div[1]/div[9]/table/thead[2]/tr"],
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

const TABS_OF_DATA:&[&[&str]] = &[
     //No.,FieldName,site xpath,table xpath
     &["t1","prices","/html/body/div[3]/div[3]/div[1]/div[1]/ul[1]/li[1]/a","/html/body/div[3]/div[3]/div[1]/div[9]/table"],
     &["t2","financial",""],
     &["t3","revenue","/html/body/div[3]/div[3]/div[1]/div[8]/div[1]/table"],
     

    ];


async fn save_table_to_file(_driver: WebDriver) -> color_eyre::Result<(), Box<dyn Error>> {

// switch to tab
Ok(())

}

//save_TABLE_XPATH
#[allow(dead_code)]
async fn save_table_to_file_worker(_driver: WebDriver,output_file_name:&str,table_xpath:&[&[&str]]) -> color_eyre::Result<(), Box<dyn Error>> {
    let mut field = 0;

    let mut wtr = WriterBuilder::new()
    .flexible(true)
    .has_headers(false)
    .from_writer(vec![]);

    // debug
    println!("No.   => {}", table_xpath[field][0]);
    println!("Field => {}", table_xpath[field][1]);
    println!("XPath => {}", table_xpath[field][2]);

    let thead_rows_vec: Vec<WebElement> = _driver.find_all(By::XPath(table_xpath[0][2])).await?;

    println!("DEBUG: thead_rows_vec len => {:?}", thead_rows_vec.len());

    let mut row = 0;

    for thead_row in thead_rows_vec {
        let thead_cell_vec: Vec<WebElement> = thead_row.find_all(By::XPath("th")).await?;

        println!("DEBUG: thead_cell_vec len => {:?}", thead_cell_vec.len());

        let mut column = 0;
        for thead_cell in thead_cell_vec {
            column = column + 1;
            let cell_text = thead_cell.text().await?;
            println!(
                "DEBUG: write_field row/column {}/{} => {}",
                row, column, cell_text
            );
            wtr.write_field(cell_text)?;
        } //finish inner for loop => thead_cell

        println!("DEBUG: write_record");
        let _ = &wtr.write_record(None::<&[u8]>)?;

        field = 1;

        // debug
        println!("No.   => {}", table_xpath[field][0]);
        println!("Field => {}", table_xpath[field][1]);
        println!("XPath => {}", table_xpath[field][2]);

        let tbody_row_vec: Vec<WebElement> =
            _driver.find_all(By::XPath(table_xpath[field][2])).await?;

        row = 0;
        for tbody_row in tbody_row_vec {
            row = row + 1;

            let tbody_cell_vec: Vec<WebElement> = tbody_row.find_all(By::XPath("td")).await?;
            let mut column = 0;
            for tbody_cell in tbody_cell_vec {
                column = column + 1;
                let cell_text = tbody_cell.text().await?;
                println!(
                    "DEBUG: write_field row/column {}/{} => {}",
                    row, column, cell_text
                );
                wtr.write_field(cell_text)?;
            } //finish inner for loop => tbody_cell

            println!("DEBUG: write_record");

            // don't use result
            let _ = &wtr.write_record(None::<&[u8]>)?;
        } //finish for loop => tbody_row
    } //finish for loop => thead_row

    println!("output_file_name => {}",output_file_name);
    let mut file = File::create(output_file_name)?;
    file.write_all(&wtr.into_inner()?)?;

    Ok(())
}

async fn initialize_driver() -> Result<WebDriver, WebDriverError> {
    let _caps = DesiredCapabilities::chrome();

    // let mut caps: thirtyfour::ChromeCapabilities = DesiredCapabilities::chrome();
    // caps.add_chrome_arg("--headless")?;
    // caps.add_chrome_arg("--no-sandbox")?;
    //  caps.add_chrome_arg("--disable-dev-shm-usage")?;

    let driver = WebDriver::new("http://localhost:9515", _caps).await?;
    driver.maximize_window().await?;
    Ok(driver)
}




/*
rustfmt  ./examples/tokio_finviz_method_five.rs
*/
