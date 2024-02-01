use std::fs;

use nom::{
    branch::alt,
    bytes::complete::{tag, take_until, take_while1},
    character::complete::multispace1,
    combinator::{map, map_res},
    multi::many0,
    sequence::{preceded, terminated, tuple},
    IResult,
};

use eyre::eyre;
use serde::Serialize;

#[inline]
fn zone1970_single_line(input: &[u8]) -> IResult<&[u8], &[u8]> {
    let (input, (_, _, _, _, tz, _, _)) = tuple((
        take_until("\t"),
        multispace1,
        take_until("\t"),
        multispace1,
        take_while1(|c| c != b'\t' && c != b'\n'),
        take_until("\n"),
        line_rest,
    ))(input)?;

    Ok((input, tz))
}

#[inline]
fn line_rest(input: &[u8]) -> IResult<&[u8], ()> {
    map(take_until("\n"), |_| ())(input)
}

#[inline]
fn comment(input: &[u8]) -> IResult<&[u8], ()> {
    map(terminated(tag("#"), line_rest), |_| ())(input)
}

#[inline]
fn whitespace(input: &[u8]) -> IResult<&[u8], ()> {
    alt((map(multispace1, |_| ()), comment))(input)
}

#[inline]
fn hr(input: &[u8]) -> IResult<&[u8], ()> {
    map(many0(whitespace), |_| ())(input)
}

fn list_zoneinfo_inner(input: &[u8]) -> IResult<&[u8], Vec<&str>> {
    let (input, result) = many0(preceded(
        hr,
        map_res(zone1970_single_line, std::str::from_utf8),
    ))(input)?;

    Ok((input, result))
}

#[derive(Debug, Serialize)]
pub struct ZoneInfo {
    text: String,
    data: String,
}

pub fn list_zoneinfo() -> eyre::Result<Vec<ZoneInfo>> {
    let s = fs::read("/usr/share/zoneinfo/zone1970.tab")?;

    let mut list = list_zoneinfo_inner(&s)
        .map_err(|e| eyre!("{e}"))?
        .1
        .iter()
        .map(|x| ZoneInfo {
            text: x.to_string(),
            data: x.to_string(),
        })
        .collect::<Vec<_>>();

    let pos = list.iter().position(|x| x.text == "Asia/Shanghai").unwrap();
    let entry = list.remove(pos);
    list.insert(0, entry);

    let s = "Asia/Beijing".to_string();

    list.insert(
        0,
        ZoneInfo {
            text: s.clone(),
            data: s,
        },
    );

    Ok(list)
}
