use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufReader, BufWriter};

#[derive(Debug, Deserialize, Clone, Serialize)]
struct Campaign {
    campaign_id: u32,
    campaign_name: String,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
struct Schedule {
    campaign_id: u32,
    campaign_date: String,
    campaign_description: String,
}

#[derive(Debug, Serialize, Clone)]
struct CampaignSchedule {
    campaign: Campaign,
    schedules: Vec<Schedule>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let campaigns: Vec<Campaign> = read_campaigns();
    let schedules: Vec<Schedule> = read_schedules();
    let campaignschedules: Vec<CampaignSchedule> = campaigns
        .iter()
        .map(|campaign| {
            let matching_schedules: Vec<Schedule> = schedules
                .iter()
                .filter(|schedule| schedule.campaign_id == campaign.campaign_id)
                .cloned()
                .collect();

            CampaignSchedule {
                campaign: campaign.clone(),
                schedules: matching_schedules,
            }
        })
        .collect();

    let file = File::create("campaignschedules.json")?;
    let writer = BufWriter::new(file);
    serde_json::to_writer(writer, &campaignschedules)?;

    Ok(())
}

fn read_campaigns() -> Vec<Campaign> {
    let file = File::open("campaigns.json").expect("Could not find file");
    let reader = BufReader::new(file);
    let campaigns: Vec<Campaign> = serde_json::from_reader(reader).expect("Invalid JSON");
    campaigns
}

fn read_schedules() -> Vec<Schedule> {
    let file = File::open("schedules.json").expect("Could not find file");
    let reader = BufReader::new(file);

    let schedules: Vec<Schedule> = serde_json::from_reader(reader).expect("Invalid JSON");
    schedules
}
