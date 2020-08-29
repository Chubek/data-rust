use std::io;
use fantoccini::{Client, Locator};
use std::result::Result;
use std::error::Error;
extern crate odbc;
extern crate env_logger;
use odbc::*;
use odbc_safe::AutocommitOn;
use std::borrow::{Borrow, BorrowMut};
use std::boxed::Box;
use serde_json;
use std::io::Read;
use regex::Regex;
use std::str::Split;
use odbc::ResultSetState::Data;
use serde::de::Unexpected::Bool;
use std::iter::Zip;
use itertools::{Itertools, EitherOrBoth::*, max};
use tokio::stream::StreamExt;

struct Stuff {
    phone: Vec<String>,
    fax: Vec<String>,
    cert: Vec<String>,
    degree: Vec<String>,
    name: Vec<String>,
    addr_city: Vec<String>,
    addr_country: Vec<String>,
    site: Vec<String>,
    year: Vec<String>,
    biz_name: Vec<String>,
    iter_index: i32
}

fn match_value(val: &str) -> &str {
    let x = match Some(val) {
        Some(val) => val,
        None => ""
    };

    x
}


impl Stuff {
    fn construct_insert_statement(&mut self) -> Result<String, Box<dyn Error>> {
        let mut ret = String::from("INSERT INTO ApmaPodScrape VALUES \n");

        for i in 0_usize..(self.iter_index as usize) {
            ret.push_str(&*format!("\n ( \n '{}', \n '{}', \n '{}', \n '{}', \n '{}', \n\
             '{}', \n '{}', \n '{}', \n '{}' \n, '{}' \n ), \n",
                                   match_value(&*self.phone[i]),
                                   match_value(&*self.fax[i]),
                                   match_value(&*self.cert[i]),
                                   match_value(&*self.degree[i]),
                                   match_value(&*self.name[i]),
                                   match_value(&*self.addr_city[i]),
                                   match_value(&*self.addr_country[i]),
                                   match_value(&*self.site[i]),
                                   match_value(&*self.year[i]),
                                   match_value(&*self.biz_name[i])
            ));
        }

        Ok(ret.trim_end_matches(", \n").to_string())
    }

    fn new_stuff() -> Stuff {
        let stuff = Stuff {
            phone: " ,".repeat(19 ).split(",").into_iter().map(|x|x.to_string()).collect_vec(),
            fax: " ,".repeat(19 ).split(",").into_iter().map(|x|x.to_string()).collect_vec(),
            cert: " ,".repeat(19 ).split(",").into_iter().map(|x|x.to_string()).collect_vec(),
            degree: " ,".repeat(19 ).split(",").into_iter().map(|x|x.to_string()).collect_vec(),
            name: " ,".repeat(19 ).split(",").into_iter().map(|x|x.to_string()).collect_vec(),
            addr_city: " ,".repeat(19 ).split(",").into_iter().map(|x|x.to_string()).collect_vec(),
            addr_country: " ,".repeat(19 ).split(",").into_iter().map(|x|x.to_string()).collect_vec(),
            site: " ,".repeat(19 ).split(",").into_iter().map(|x|x.to_string()).collect_vec(),
            year: " ,".repeat(19 ).split(",").into_iter().map(|x|x.to_string()).collect_vec(),
            biz_name: " ,".repeat(19 ).split(",").into_iter().map(|x|x.to_string()).collect_vec(),
            iter_index: 0
        };

        stuff
    }

    fn set_iter_index(&mut self, index_assessor: i32) {
        println!("index assessor {}", index_assessor);
        if index_assessor % 3 == 0 {
            self.iter_index += 1;
        }


    }

    fn push_to_stuff(&mut self, flag: &str, value: String) {

        //println!("{} matched {}", flag, value);


        println!(" value {}", value);
        println!(" iter index {}", self.iter_index);

        match flag {
            "phone" => self.phone[self.iter_index as usize] = value,
            "fax" => self.fax[self.iter_index as usize] = value,
            "cert" => self.cert[self.iter_index as usize] = value,
            "degree" => self.degree[self.iter_index as usize] = value,
            "name" => self.name[self.iter_index as usize] = value,
            "addr_city" => self.addr_city[self.iter_index as usize] = value,
            "addr_country" => self.addr_country[self.iter_index as usize] = value,
            "site" => self.site[self.iter_index as usize] = value,
            "year" => self.year[self.iter_index as usize] = value,
            "biz" => self.biz_name[self.iter_index as usize] = value,
            _ => {}
        }
    }


}


struct RegexType {
    regex: Regex,
    reg_type: &'static str

}


impl RegexType {
    fn set_type(self: &Self, stuff: &mut Stuff, pattern: &str) {
        //println!("Got {}",pattern);
        if self.regex.is_match(pattern) {
            match self.reg_type {
                "phone" => stuff.push_to_stuff(self.reg_type, pattern.to_string()),
                "fax" => stuff.push_to_stuff(self.reg_type, pattern.to_string()),
                "cert" => stuff.push_to_stuff(self.reg_type, pattern.to_string()),
                "degree" => stuff.push_to_stuff(self.reg_type, pattern.to_string()),
                "name" => stuff.push_to_stuff(self.reg_type, pattern.to_string()),
                "addr_city" => stuff.push_to_stuff(self.reg_type, pattern.to_string()),
                "addr_country" => stuff.push_to_stuff(self.reg_type, pattern.to_string()),
                "site" => stuff.push_to_stuff(self.reg_type, pattern.to_string()),
                "year" => stuff.push_to_stuff(self.reg_type, pattern.to_string()),
                "biz" => stuff.push_to_stuff(self.reg_type, pattern.to_string()),
                _ => {}
            }

        }

        }


}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut caps = serde_json::map::Map::new();

    let opts = serde_json::json!({ "args": ["--headless"] });
    caps.insert("moz:firefoxOptions".to_string(), opts.clone());
    let mut c = Client::with_capabilities("http://127.0.0.1:4444", caps).await?;
    //11466


    let conn_str = "Driver={ODBC Driver 17 for SQL Server};Server=tcp:chubak-sql.database.windows.net,\
    1433;Database=marketing_scrape;Uid=chubak;Pwd=LAsvegas11;Encrypt=yes;TrustServerCertificate=no;\
    Connection Timeout=30;";

    let env = create_environment_v3().map_err(|e| e.unwrap())?;

    let sql_client = env.connect_with_connection_string(conn_str)?;

    for i in (20_i32..11466_i32).step_by(20_usize) {
        let mut stuff = Stuff::new_stuff();


        c.goto(&*format!("https://www.apma.org/Directory/FindAPodiatrist.cfm\
        ?Compact=0&FirstName=&LastName=&City=&State=&Zip=&Country=United+States&startrow={}\
        &endrow={}", i + 1, i + 20)).await?;

        let mut td_text = c.find_all(Locator::XPath("//tr/td")).await?;


        let mut i = 1;

        for mut elem in td_text {

            let txt = elem.text().await?;
            let splitsville = txt.split("\n");

            let mut split_vec = Vec::new();

            for split in splitsville {
                split_vec.push(split);
            }

            let re_phone = Regex::new(r"^\(\d+\) \d+.\d+ phone$").unwrap();
            let re_fax = Regex::new(r"^\(\d+\) \d+.\d+ fax$").unwrap();
            let re_cert = Regex::new(r"^[A-Z]+ \- [A-Z ]+.[A-Z ]+$").unwrap();
            let re_degree = Regex::new(r"(University|School|College|formerly)").unwrap();
            let re_name = Regex::new(r"([A-Z][a-z]+)(, DPM)").unwrap();
            let re_addr_first_line = Regex::new(r"\d+\s+[A-Z\s\d]+").unwrap();
            let re_addr_second_line = Regex::new(r"[A-Z][A-Z] \d+.\d+").unwrap();
            let re_site = Regex::new(r"www.\w+\.\w+").unwrap();
            let re_year = Regex::new(r"^\d\d\d\d$").unwrap();
            let re_biz = Regex::new(r"Clinic|Center|Foot|Surgery|Specialists|Ankle").unwrap();


            let re_phone_sct = RegexType {regex: re_phone, reg_type: "phone"};
            let re_fax_sct = RegexType {regex: re_fax, reg_type: "fax"};
            let re_cert_sct = RegexType {regex: re_cert, reg_type: "cert"};
            let re_degree_sct = RegexType {regex: re_degree, reg_type: "degree"};
            let re_name_sct = RegexType {regex: re_name, reg_type: "name"};
            let re_addr_city_sct = RegexType {regex: re_addr_first_line, reg_type: "addr_city"};
            let re_addr_country_sct = RegexType {regex: re_addr_second_line, reg_type: "addr_country"};
            let re_site_sct = RegexType {regex: re_site, reg_type: "site"};
            let re_year_sct = RegexType {regex: re_year, reg_type: "year"};
            let re_biz_sct = RegexType {regex: re_biz, reg_type: "biz"};


            let mut re_vec = vec![re_phone_sct, re_fax_sct, re_cert_sct, re_degree_sct,
                                  re_name_sct, re_addr_city_sct,
                                  re_addr_country_sct, re_site_sct, re_year_sct, re_biz_sct];

            let iter = re_vec.iter()
                .cartesian_product(split_vec.iter());



            for it in iter {
                it.0.set_type(&mut stuff, it.1);

            }

            stuff.set_iter_index(i);
            i += 1;
        }

        let insert_str = stuff.construct_insert_statement().unwrap();


        println!("{}", insert_str);

       execute_statement(&sql_client,&*insert_str)?;

    }

    c.close().await?;

    Ok(())

}


fn execute_statement(conn: &Connection<AutocommitOn>, sql_text: &str) -> Result<(), Box<dyn Error>> {
    let stmt = Statement::with_parent(conn)?;

    match stmt.exec_direct(sql_text)? {
        Data(mut stmt) => {
            let cols = stmt.num_result_cols()?;
            while let Some(mut cursor) = stmt.fetch()? {
                for i in 1..(cols + 1) {
                    match cursor.get_data::<&str>(i as u16)? {
                        Some(val) => print!(" {}", val),
                        None => print!(" NULL"),
                    }
                }
                println!();
            }
        }
        NoData(_) => println!("Query executed, no data returned"),
    }

    Ok(())
}


fn create_table(conn: &Connection<AutocommitOn>) -> Result<(), Box<dyn Error>> {
    let statement = "CREATE TABLE ApmaPodScrape (\
                                            Phone nvarchar(800),\
                                            Fax nvarchar(800),\
                                            Cert nvarchar(2400),\
                                            Degree nvarchar(3000),\
                                            Name nvarchar(500),\
                                            AddressCity nvarchar(1000),\
                                            AddressState nvarchar(1000),\
                                            Site nvarchar(400))";

    Ok(execute_statement(conn, statement)?)
}

