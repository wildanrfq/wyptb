type Error = Box<dyn std::error::Error>;
type Result<T, E = Error> = std::result::Result<T, E>;

use std::io;

use num_format::{Locale, ToFormattedString};
use scraper::{Html, Selector};

fn main() -> Result<()> {
    loop {
        const BASE_URL: &str = "https://willyoupressthebutton.com";
        let cond_selector = Selector::parse("div.rect").unwrap();
        let req = reqwest::blocking::get(BASE_URL)?.text()?;
        let data = Html::parse_document(req.as_str());
        let conds = data.select(&cond_selector);
        let mut cond_texts = vec![];
        for cond in conds {
            cond_texts.push(cond.inner_html().replace("\n    ", ""))
        }
        cond_texts[1] =
            (cond_texts[1][..1].to_ascii_lowercase() + &cond_texts[1][1..] + ".").to_string();
        let cond_text = cond_texts.join(" but ");
        println!("Will You Press The Button? {cond_text}\n[y] Yes\n[n] No\n[q] Quit\nType your answer below (case insensitive):");
        let mut answer = String::new();
        io::stdin()
            .read_line(&mut answer)
            .expect("Failed to process the answer. Please try again.");
        let answer = answer.trim().to_ascii_lowercase();
        if [String::from("y"), String::from("n")].contains(&answer) {
            let stats_selector = Selector::parse(r#"a[id="yesbtn"]"#).unwrap();
            let stats_url = BASE_URL.to_string()
                + data
                    .select(&stats_selector)
                    .next()
                    .unwrap()
                    .value()
                    .attr("href")
                    .unwrap();
            let stats_req = reqwest::blocking::get(stats_url)?.text()?;
            let stats_data = Html::parse_document(stats_req.as_str());
            let pressed_selector = Selector::parse("span.statsBarLeft").unwrap();
            let didnt_pressed_selector = Selector::parse("span.statsBarRight").unwrap();
            let raw_pressed = stats_data
                .select(&pressed_selector)
                .next()
                .unwrap()
                .inner_html();
            let raw_didnt_pressed = stats_data
                .select(&didnt_pressed_selector)
                .next()
                .unwrap()
                .inner_html();
            let pressed = raw_pressed.split_whitespace().collect::<Vec<_>>();
            let pressed_count = pressed[0]
                .parse::<i32>()
                .unwrap()
                .to_formatted_string(&Locale::en);
            let didnt_pressed = raw_didnt_pressed.split(" ").collect::<Vec<_>>();
            let didnt_pressed_count = didnt_pressed[0]
                .parse::<i32>()
                .unwrap()
                .to_formatted_string(&Locale::en);
            if answer == "y" {
                println!("You pressed this button along with {pressed_count} {} other people, while {didnt_pressed_count} {} other people did not.\n", pressed[1], didnt_pressed[1]);
            } else {
                println!("You didn't pressed this button along with {didnt_pressed_count} {} other people, while {pressed_count} {} other people did.\n", didnt_pressed[1], pressed[1]);
            }
        } else {
            println!("Goodbye!");
            break Ok(());
        }
    }
}
