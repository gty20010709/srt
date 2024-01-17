## SRT

simple lib to read srt subtitle.

        -- from a new Rustance.



TODO:
    - parse error.
    - add test.


### How to use?

```Cargo.toml

[dependencies]
srt = {git = "https://github.com/gty20010709/srt.git"}

```


### Example

```rs

use srt;

fn main() {
    let file_content = r#"1
00:00:33,843 --> 00:00:38,097
地球上只有3%的水是淡水

2
00:00:40,641 --> 00:00:44,687
Changing subtitle data is easy!
PS: If you knwo how.

3
00:00:57,908 --> 00:01:03,414
所有陆地生命归根结底都依赖於淡水

"#;

    let (_, subs) = srt::parse_srt_from_str(&file_content).unwrap();

    println!("{:?}", subs);
}


/* > cargo run

[Subtitle { index: 1, timeline: Timeline { start: Time { hour: 0, min: 0, sec: 33, mil_sec: 843 }, end: Time { hour: 0, min: 0, sec: 38, mil_sec: 97 } }, subs: "地球上只有3% 
的水是淡水" }, Subtitle { index: 2, timeline: Timeline { start: Time { hour: 0, min: 0, sec: 40, mil_sec: 641 }, end: Time { hour: 0, min: 0, sec: 44, mil_sec: 687 } }, subs: "Changing subtitle data is easy!\nPS: If you knwo how." }, Subtitle { index: 3, timeline: Timeline { start: Time { hour: 0, min: 0, sec: 57, mil_sec: 908 }, end: Time { hour: 0, min: 1, sec: 3, mil_sec: 414 } }, subs: "所有陆地生命归根结底都依赖於淡水" }]

*/


```