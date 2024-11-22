use std::collections::HashMap;

use axum::http::HeaderMap;
use reqwest::Client;

use crate::{
    dtos::Response,
    models::{OTPVerifyResponse, VerifyOtp},
};

#[derive(Clone)]
pub struct TwillioService {
    service_id: String,
    account_sid: String,
    auth_token: String,
}

impl TwillioService {
    pub fn new() -> TwillioService {
        TwillioService {
            service_id: String::from("YOUR_SERVICE_ID"),
            account_sid: String::from("YOUR_ACCOUNT_ID"),
            auth_token: String::from("AUTH_TOKEN"),
        }
    }

    pub async fn send_otp(&self, phone_number: String) -> Result<(), &'static str> {
        let url = format!(
            "https://verify.twilio.com/v2/Services/{serv_id}/Verifications",
            serv_id = self.service_id
        );

        let mut header = HeaderMap::new();
        header.insert(
            "Content-Type",
            "application/x-www-form-urlencoded".parse().unwrap(),
        );

        let mut form_body: HashMap<&str, String> = HashMap::new();
        //println!("😔 Failed to connect to the database: {:?}", phone_number);
        form_body.insert("To", phone_number.to_string());
        form_body.insert("Channel", "sms".to_string());

        let client = Client::new();
        let res = client
            .post(url)
            .basic_auth(self.account_sid.clone(), Some(self.auth_token.clone()))
            .headers(header)
            .form(&form_body)
            .send()
            .await;

        match res {
            Ok(_) => Ok(()),
            Err(_) => Err("Error sending OTP"),
        }
    }
    pub async fn verify_otp(&self, verify_otp: VerifyOtp) -> Result<(), &'static str> {
        let url = format!(
            "https://verify.twillio.com/v2/Services/{serv_id}/VerificationCheck",
            serv_id = self.service_id
        );

        let mut header = HeaderMap::new();
        header.insert(
            "Content-Type",
            "application/x-www-form-urlencoded".parse().unwrap(),
        );

        let mut form_body: HashMap<&str, &String> = HashMap::new();
        form_body.insert("To", &verify_otp.phone);
        form_body.insert("Code", &verify_otp.code);

        let client = Client::new();
        let res = client
            .post(url)
            .basic_auth(self.account_sid.clone(), Some(self.auth_token.clone()))
            .headers(header)
            .form(&form_body)
            .send()
            .await;

        match res {
            Ok(response) => {
                let data = response.json::<OTPVerifyResponse>().await;
                match data {
                    Ok(result) => {
                        if result.status == "approved" {
                            Ok(())
                        } else {
                            Err("Error verifying OTP")
                        }
                    }
                    Err(_) => Err("Error verifying OTP"),
                }
            }
            Err(_) => Err("Error sending OTP"),
        }
    }
}
