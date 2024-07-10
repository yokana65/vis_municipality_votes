use std::collections::HashMap;

use anyhow::Result;
use askama::Template;

use crate::structs::votes::Vote;

#[derive(Template)]
#[template(path = "main.html")]
pub struct VoteSummaryTemplate<'a> {
    vote_name: &'a str,
    vote_records: Vec<VoteRecordTemplate<'a>>,
}

pub struct VoteRecordTemplate<'a> {
    name_muni: &'a str,
    vote_perc: HashMap<String, f64>,
    total_votes: &'a i32,
    has_geometry: bool,
}

pub fn render_html_summary(vote: &Vote) -> Result<String> {
    let vote_records: Vec<VoteRecordTemplate> = vote
        .vote_records
        .iter()
        .map(|record| {
            let vote_perc_rd: HashMap<String, f64> = record.vote_perc
            .iter()
            .map(|(party, &percentage)| {
                let rounded = (percentage * 100.0).round() / 100.0;
                (party.clone(), rounded)
            })
            .collect();

            VoteRecordTemplate {
                name_muni: &record.name_muni,
                vote_perc: vote_perc_rd,
                total_votes: &record.total_votes,
                has_geometry: record.geometry.is_some(),
            }
        })
        .collect();

    let template = VoteSummaryTemplate {
        vote_name: &vote.name,
        vote_records,
    };

    Ok(template.render()?)
}
