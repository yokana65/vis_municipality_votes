use std::collections::HashMap;

use anyhow::Result;
use askama::Template;

use crate::structs::votes::Vote;

#[derive(Template)]
#[template(path = "main.html")]
struct VoteSummaryTemplate<'a> {
    vote_name: &'a str,
    vote_records: Vec<VoteRecordTemplate<'a>>,
}

struct VoteRecordTemplate<'a> {
    name_muni: &'a str,
    votes: &'a HashMap<String, i16>,
    has_geometry: bool,
}

pub fn render_html_summary(vote: &Vote) -> Result<String> {
    let vote_records: Vec<VoteRecordTemplate> = vote
        .vote_records
        .iter()
        .map(|record| VoteRecordTemplate {
            name_muni: &record.name_muni,
            votes: &record.votes,
            has_geometry: record.geometry.is_some(),
        })
        .collect();

    let template = VoteSummaryTemplate {
        vote_name: &vote.name,
        vote_records,
    };

    Ok(template.render()?)
}
