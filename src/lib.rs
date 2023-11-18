use std::{collections::HashMap};

const WOOVI_API: &str = "https://api.woovi.com/";

pub async fn get_charge(token: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let resp = client
        .get(format!("{}/{}",WOOVI_API,"api/openpix/v1/charge/"))
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    println!("{:#?}", resp);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::get_charge;

    #[test]
    fn get_charge_test() {
        let _result = get_charge("");
    }
}
