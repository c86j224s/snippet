use serde::Deserialize;
use std::cmp::PartialEq;

#[derive(Deserialize, PartialEq, Debug)]
#[serde(tag = "type", content = "value")]
pub enum Stmt {
    #[serde(rename = "query")]
    Query(String),
    #[serde(rename = "procedure")]
    Procedure(String),
}

#[derive(Deserialize, PartialEq, Debug)]
pub enum Param {
    S(String),
    NS(String),
    I(isize),
}

#[derive(Deserialize, PartialEq, Debug)]
#[serde(rename = "request")]
pub struct Request {
    pub stmt: Stmt,
    #[serde(rename = "params")]
    pub params: Vec<Param>,
}

pub fn parse_json_request(req_str: &str) -> Result<Request, std::io::Error> {
    Ok(serde_json::de::from_str(req_str)?)
}

pub fn parse_xml_request(req_str: &str) -> Result<Request, serde_xml_rs::Error> {
    Ok(serde_xml_rs::de::from_str(req_str)?)
}

#[cfg(test)]
mod tests {
    #[test]
    fn json1() {
        use super::{parse_json_request, Request, Stmt, Param};

        let req1 = r###"{
            "stmt": {
                "type": "query",
                "value": "SELECT TOP 1 Id, CountryCode, Registered FROM UserProfileDb.dbo.UserCountries WHERE Id = ?"
            },
            "params": [
                { "S": "random guid" }
            ]
        }"###;
    
        let parsed = parse_json_request(req1).unwrap();
        assert_eq!(parsed, Request{
            stmt: Stmt::Query("SELECT TOP 1 Id, CountryCode, Registered FROM UserProfileDb.dbo.UserCountries WHERE Id = ?".to_owned()),
            params: vec![
                Param::S("random guid".to_owned())
            ]
        });
    }

    #[test]
    fn xml1() {
        use super::{parse_xml_request, Request, Stmt, Param};

        let req1 = r###"
            <request>
                <stmt type="query">
                    <value>SELECT TOP 1 Id, CountryCode, Registered FROM UserProfileDb.dbo.UserCountries WHERE Id = ?</value>
                </stmt>
                <params>
                    <S>random guid</S>
                </params>
            </request>
        "###;
    
        let parsed = parse_xml_request(req1).unwrap();
        assert_eq!(parsed, Request{
            stmt: Stmt::Query("SELECT TOP 1 Id, CountryCode, Registered FROM UserProfileDb.dbo.UserCountries WHERE Id = ?".to_owned()),
            params: vec![
                Param::S("random guid".to_owned())
            ]
        });

    }
}