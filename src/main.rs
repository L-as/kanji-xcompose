#![type_length_limit = "99999999999"]
use std::{
	collections::HashMap,
	convert::{TryFrom, TryInto},
	io::{self, Read, Write},
};

#[derive(Debug)]
struct Kanji {
	codepoint: char,
	freq:      u16,
}

fn katakana_voiced_to_unvoiced(codepoint: char) -> char {
	let codepoint = codepoint as u32;
	let codepoint = match codepoint {
		0x30ac..=0x30c2 => ((codepoint + 1) & !1) - 1,
		0x30c5..=0x30c9 => codepoint & !1,
		0x30cf..=0x30dd => codepoint - ((codepoint + 0x30cf) % 3),
		_ => codepoint,
	};
	codepoint.try_into().unwrap()
}

const KATAKANA_HIRAGANA_OFFSET: u32 = 'ア' as u32 - 'あ' as u32;

fn number_to_key(n: u8) -> char {
	match n {
		1 => 'あ',
		2 => 'い',
		3 => 'う',
		4 => 'え',
		5 => 'お',
		6 => 'な',
		7 => 'に',
		8 => 'ぬ',
		9 => 'ね',
		10 => 'の',
		11 => 'や',
		12 => 'わ',
		13 => 'か',
		14 => 'き',
		15 => 'く',
		16 => 'け',
		17 => 'こ',
		18 => 'は',
		19 => 'ひ',
		20 => 'ふ',
		21 => 'へ',
		22 => 'ほ',
		23 => 'ゆ',
		24 => 'を',
		25 => 'さ',
		26 => 'し',
		27 => 'す',
		28 => 'せ',
		29 => 'そ',
		_ => unreachable!("too many strokes! {}", n),
	}
}

fn ending_to_key(ending: &str, voiced: bool) -> char {
	if voiced {
		match ending {
			"イ" => 'さ',
			"ウ" => 'し',
			"キ" => 'す',
			"ク" => 'せ',
			"チ" => 'そ',
			"ツ" => 'ま',
			"ャ" => 'み',
			"ャク" => 'む',
			"ュ" => 'め',
			"ュウ" => 'た',
			"ュク" => 'ち',
			"ュツ" => 'つ',
			"ュン" => 'て',
			"ョ" => 'と',
			"ョウ" => 'ら',
			"ョク" => 'り',
			"ン" => 'る',
			"" => 'れ',
			_ => unreachable!("unexpected ending {}", ending),
		}
	} else {
		match ending {
			"イ" => 'あ',
			"ウ" => 'い',
			"キ" => 'う',
			"ク" => 'え',
			"チ" => 'お',
			"ツ" => 'な',
			"ャ" => 'に',
			"ャク" => 'ぬ',
			"ュ" => 'ね',
			"ュウ" => 'か',
			"ュク" => 'き',
			"ュツ" => 'く',
			"ュン" => 'け',
			"ョ" => 'こ',
			"ョウ" => 'は',
			"ョク" => 'ひ',
			"ン" => 'ふ',
			"" => 'へ',
			_ => unreachable!("unexpected ending {}", ending),
		}
	}
}

const XCOMPOSE_BASE: &'static str = include_str!("XCompose-base");

fn main() {
	let mut buffer = String::new();
	io::stdin()
		.read_to_string(&mut buffer)
		.expect("couldn't read from stdin");

	let doc = roxmltree::Document::parse(&buffer).expect("couldn't parse stdin");
	let root = doc.root_element();

	let kanjis = root.children().filter(|n| {
		n.has_tag_name("character")
			&& n.children()
				.find(|n| n.has_tag_name("misc"))
				.map_or(false, |n| {
					n.children()
						.any(|n| n.has_tag_name("freq") || n.has_tag_name("jlpt") || n.has_tag_name("grade"))
				})
	});

	let mut map = HashMap::<(&str, u8), Vec<Kanji>>::new();

	'kanji: for kanji in kanjis {
		let mut pronunciation: Option<&str> = None;
		let mut codepoint: Option<char> = None;
		let mut stroke_count: Option<u8> = None;
		let mut freq = 2501u16;
		for n in kanji.children() {
			match n.tag_name().name() {
				"reading_meaning" if pronunciation.is_none() => {
					pronunciation = n
						.children()
						.find(|n| n.has_tag_name("rmgroup"))
						.and_then(|n| {
							n.children().find(|n| {
								n.has_tag_name("reading") && n.attribute("r_type") == Some("ja_on")
							})
						})
						.and_then(|n| n.text());
				},
				"literal" => {
					codepoint = n.text().and_then(|str| str.chars().next());
				},
				"misc" => {
					for n in n.children() {
						match n.tag_name().name() {
							"stroke_count" if stroke_count.is_none() => {
								stroke_count = n.text().and_then(|s| s.parse().ok());
							},
							"freq" => {
								if let Some(n) = n.text().and_then(|s| s.parse().ok()) {
									freq = n;
								}
							},
							"grade" => {
								if let Some(grade) = n.text().and_then(|s| s.parse::<u8>().ok()) {
									if grade > 8 {continue 'kanji}
								}
							}
							_ => {},
						}
					}
				},
				_ => {},
			}
		}
		if let (Some(codepoint), Some(pronunciation), Some(stroke_count)) =
			(codepoint, pronunciation, stroke_count)
		{
			let v = if let Some(v) = map.get_mut(&(pronunciation, stroke_count)) {
				v
			} else {
				map.insert((pronunciation, stroke_count), Vec::new());
				map.get_mut(&(pronunciation, stroke_count)).unwrap()
			};
			v.push(Kanji { codepoint, freq });
			v.sort_by(|a, b| a.freq.cmp(&b.freq));
		}
	}

	io::stdout().write_all(XCOMPOSE_BASE.as_bytes()).unwrap();
	for ((pronunciation, stroke_count), kanjis) in map {
		let beginning = pronunciation.chars().next().unwrap();
		let ending = &pronunciation[beginning.len_utf8()..];
		let voiced = katakana_voiced_to_unvoiced(beginning) != beginning;

		let sel1: char = (katakana_voiced_to_unvoiced(beginning) as u32 - KATAKANA_HIRAGANA_OFFSET)
			.try_into()
			.unwrap();
		let sel2 = ending_to_key(ending, voiced);
		let sel3 = number_to_key(stroke_count);

		let len = kanjis.len();
		if len > 1 {
			for (i, kanji) in kanjis.into_iter().enumerate() {
				println!(
					"<UFFE0> <U{:04X}> <U{:04X}> <U{:04X}> <U{:04X}> : \"{}\"",
					sel1 as u32,
					sel2 as u32,
					sel3 as u32,
					number_to_key(u8::try_from(i).unwrap() + 1) as u32,
					kanji.codepoint
				);
			}
		} else {
			let kanji = &kanjis[0];
			println!(
				"<UFFE0> <U{:04X}> <U{:04X}> <U{:04X}> : \"{}\"",
				sel1 as u32, sel2 as u32, sel3 as u32, kanji.codepoint
			);
		}
	}
}
