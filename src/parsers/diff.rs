use nom::{
    bytes::complete::{tag, take_until},
    character::complete::newline,
    multi::{many0, many1},
    sequence::{preceded, tuple},
    IResult,
};

use crate::entities::diff::{DiffChange, DiffLineChange, FileDiff};

fn parse_line_change(input: &str) -> IResult<&str, DiffLineChange> {
    let (input, removed) = preceded(tag("-"), take_until("\n"))(input)?;
    let (input, _) = newline(input)?;
    let (input, added) = preceded(tag("+"), take_until("\n"))(input)?;
    let (input, _) = newline(input)?;
    Ok((
        input,
        DiffLineChange {
            removed: removed.to_string(),
            added: added.to_string(),
        },
    ))
}

fn parse_diff_change(input: &str) -> IResult<&str, DiffChange> {
    let (input, line_changes) = many1(parse_line_change)(input)?;
    Ok((input, DiffChange { line_changes }))
}

pub fn parse_file_diff(input: &str) -> IResult<&str, FileDiff> {
    let (input, _) = tag("@@")(input)?;
    let (input, _) = take_until("@@")(input)?;
    let (input, _) = tag("@@")(input)?;
    let (input, _) = newline(input)?;

    let (input, changes) = many0(parse_diff_change)(input)?;
    Ok((input, FileDiff { changes }))
}
