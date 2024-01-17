use nom::{
    branch::alt,
    bytes::complete::{tag, take_until, take_while},
    character::complete::digit1,
    combinator::{map_res, recognize},
    multi::many0,
    sequence::{terminated, tuple},
    IResult,
};

#[derive(Debug)]
pub struct Subtitle {
    pub index: u32,
    pub timeline: Timeline,
    pub subs: String,
}

#[derive(Debug)]
pub struct Time {
    pub hour: u32,
    pub min: u32,
    pub sec: u32,
    pub mil_sec: u32,
}

#[derive(Debug)]
pub struct Timeline {
    pub start: Time,
    pub end: Time,
}

pub fn parse_srt_from_str(input: &str) -> IResult<&str, Vec<Subtitle>> {
    let (input, subs) = many0(parse_block)(input)?;
    Ok((input, subs))
}

fn parse_block(input: &str) -> IResult<&str, Subtitle> {
    let (input, block) = get_until_empty_line(input)?;
    let (input, _) = many0(rm_newline_from_head)(input)?;
    let (block, index) = get_index(block)?;
    let (subs, timeline) = get_timeline(block)?;
    let (_, timeline) = parse_timeline(timeline)?;
    let (subs, _) = rm_newline_from_head(subs)?;
    Ok((
        input,
        Subtitle {
            index,
            timeline,
            subs:subs.to_string(),
        },
    ))
}

fn parse_time(input: &str) -> IResult<&str, Time> {
    let (remaining, (h, _, m, _, s, _, ms)) =
        tuple((digit1, tag(":"), digit1, tag(":"), digit1, tag(","), digit1))(input)?;

    let (_, h) = parse_to_u32(h)?;
    let (_, m) = parse_to_u32(m)?;
    let (_, s) = parse_to_u32(s)?;
    let (_, ms) = parse_to_u32(ms)?;

    Ok((
        remaining,
        Time {
            hour: h,
            min: m,
            sec: s,
            mil_sec: ms,
        },
    ))
}

fn parse_timeline(input: &str) -> IResult<&str, Timeline> {
    let (end, start) = terminated(
        take_while(|c: char| c.is_numeric() || c == ':' || c == ','),
        tag(" --> "),
    )(input)?;
    let (_, start) = parse_time(start)?;
    let (_, end) = parse_time(end)?;
    Ok(("", Timeline { start, end }))
}

fn get_index(input: &str) -> IResult<&str, u32> {
    let (input, index) = get_first_line(input)?;
    let (_, index) = parse_to_u32(index)?;
    Ok((input, index))
}
fn get_timeline(input: &str) -> IResult<&str, &str> {
    let (input, _) = rm_newline_from_head(input)?;
    get_first_line(input)
}

fn parse_to_u32(input: &str) -> IResult<&str, u32> {
    Ok(map_res(recognize(digit1), |s: &str| s.parse::<u32>())(
        input,
    )?)
}


fn get_first_line(input: &str) -> IResult<&str, &str> {
    Ok(alt((take_until("\r\n"), take_until("\n")))(input)?)
}

fn get_until_empty_line(input: &str) -> IResult<&str, &str> {
    Ok(alt((take_until("\n\n"), take_until("\r\n\r\n")))(input)?)
}

fn rm_newline_from_head(input: &str) -> IResult<&str, &str> {
    Ok(alt((tag("\r\n"), tag("\n")))(input)?)
}
