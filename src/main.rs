mod config;
mod epgstation_api;

use std::{env, path::PathBuf};

use anyhow::{Ok, Result};
use config::{
    config::{create_config_file, load_config},
    model::EncodeRule,
};
use epgstation_api::{
    api::Client,
    model::{FetchRecordedProperty, Record, RecordId, RequestEncodeProperty, RuleId},
};
use reqwest::Url;

async fn fetch_encode_queue(epgstation: &Client) -> Result<Vec<RecordId>> {
    let encode_response = epgstation.fetch_encode(true).await?;
    let encode_queue_record_ids: Vec<RecordId> = encode_response
        .running_items
        .iter()
        .chain(encode_response.wait_items.iter())
        .map(|item| item.recorded.id)
        .collect();
    Ok(encode_queue_record_ids)
}

async fn filter_encode_target<'a>(
    epgstation: &Client,
    records: &'a Vec<Record>,
) -> Result<Vec<&'a Record>> {
    let encode_queue_record_ids = fetch_encode_queue(epgstation).await?;
    let encord_target_records: Vec<_> = records
        .iter()
        .filter(|record| {
            record.video_files.iter().all(|file| file.type_ == "ts")
                && record.video_files.len() == 1
        })
        .filter(|record| !encode_queue_record_ids.contains(&record.id))
        .collect();
    Ok(encord_target_records)
}

fn find_encode_mode(
    target_rule_id: Option<RuleId>,
    encode_rules: &Vec<EncodeRule>,
    default_encode_mode: &str,
) -> String {
    let rule = encode_rules.iter().find(|rule| {
        let mut is_match = true;
        if let Some(rules) = &rule.rules {
            is_match &= target_rule_id
                .map(|target_id| rules.contains(&target_id))
                .unwrap_or(false);
        }

        if let Some(no_rule) = rule.no_rule {
            is_match &= no_rule == target_rule_id.is_none();
        }
        is_match
    });

    rule.map_or(default_encode_mode.to_string(), |rule| {
        rule.encode_mode.to_string()
    })
}

#[tokio::main]
async fn main() -> Result<()> {
    let settings_path = PathBuf::from("settings.toml");
    if !settings_path.exists() {
        println!("{0:?} is not found. Create a new {0:?}.", settings_path);
        create_config_file(&settings_path)?;
        println!(
            "{0:?} was created. \nEdit {0:?} and run it again.",
            settings_path
        );
        return Ok(());
    }
    let config = load_config(&settings_path)?;
    let epgstation = Client::new(Url::parse(&config.epgstation_url)?);

    if env::args().len() >= 2 && env::args().nth(1).unwrap() == "--rules" {
        let rules = epgstation.fetch_rules(0, 1000000, None, None).await?;
        println!("ID  | Keyword");
        for rule in rules {
            println!("{:3} | {}", rule.id, rule.search_option.keyword);
        }
        return Ok(());
    }

    let records = epgstation
        .fetch_recorded(FetchRecordedProperty {
            is_half_width: true,
            limit: 100_000_000,
            is_reverse: Some(true),
            ..Default::default()
        })
        .await?;
    let encode_target_records = filter_encode_target(&epgstation, &records).await?;

    for record in &encode_target_records {
        println!("Send encode request for {}...", record.name);
        let encode_mode = find_encode_mode(
            record.rule_id,
            &config.encode_rule,
            &config.default_encode_mode,
        );

        epgstation
            .request_encode(RequestEncodeProperty {
                recorded_id: record.id,
                source_video_file_id: record.video_files[0].id,
                parent_dir: "recorded".to_string(),
                mode: encode_mode.to_string(),
                remove_original: false,
            })
            .await?;
    }

    Ok(())
}
