//! Convert city name to city code. Eg. Nanjing -> xxxxx

use chrono::prelude::*;
use reqwest::blocking;
use serde_json::Value;
use std::process;

fn get_code_from_name(name: &str) -> reqwest::Result<String> {
    let url: String = format!(
        "http://toy1.weather.com.cn/search?cityname={}&callback=&_={}",
        name,
        Local::now().format("%s")
    );
    let res = blocking::get(&url).and_then(|res| {
        let raw_str = res.text()?;
        let json_str = &raw_str[1..(raw_str.len() - 1)];
        Ok(serde_json::from_str(json_str).and_then(|query_info: Value| {
            match query_info[0]["ref"].as_str() {
                Some(ref_) => {
                    let ref_vec: Vec<&str> = ref_.split("~").collect();
                    Ok(ref_vec[0].to_owned())
                },
                _ => Ok("".to_owned())
            }
        }).unwrap())
    })?;
    Ok(res)
}

pub fn get_code(name_or_code: &str) -> String {
    match get_code_from_name(name_or_code) {
        Ok(citycode) if citycode == "" => process::exit(1),
        Ok(citycode) => citycode,
        _ => name_or_code.to_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_code_from_name() {
        let data = get_code("北京");
        assert_eq!("101010100", data);
        let data = get_code("101010100");
        assert_eq!("101010100", data);
        let data = get_code_from_name("是个啥").unwrap();
        assert_eq!("", data);
    }
}
