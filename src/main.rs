//! Program to calculate the ${Foo}day of the month, for every month in a given
//! year.
use chrono::prelude::*;
use chrono::{NaiveDate, Weekday};
use clap::Parser;

/// Accumulate into a vec, with index being month. Replace lower values with
/// higher ones
fn folder( mut acc: Vec<NaiveDate>, date: NaiveDate ) -> Vec<NaiveDate> {
	let month = date.month0() as usize;
	match acc.get(month) {
		None => acc.insert(month, date),
		Some(d2) if date > *d2 => acc[month] = date,
		_ => ()
	}
	acc
}

/// Takes a year and get the first `${Foo}day`. Then calculate every
/// `${Foo}day` of the given year. Fold this into a Vec of length 12 (one for
/// each month) representing the last `${Foo}day`.
fn get_last_day_of_months(yr: i32, day: Weekday) -> Vec<NaiveDate> {
	NaiveDate::from_weekday_of_month(yr, 1, day, 1)
		.iter_weeks()
		.take_while( |d| d.year() == yr ) // && d.month() == month )
		.fold( Vec::with_capacity(12), &folder )
}

/// Simple program to display the last given day of the week for every month in
/// a given year.
///   ex\ ./lastdayofmonth --day Sunday --year 2022
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
   /// The day of the week
   #[clap(short, long)]
   day: String,

   /// Year to calculate the last day of every month
	 // https://github.com/chronotope/chrono/issues/743
   #[clap(short, long, default_value_t = 2022)]
   year: i32
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let args = Args::parse();
	let day: Weekday = args.day.parse().unwrap();
	for day in get_last_day_of_months(args.year, day) {
		println!("{}", day);
	}
	Ok(())
}

#[cfg(test)]
mod test {
	use super::*;
	use std::iter::zip;
	#[test]
	fn last_sundays_in_2022 () {
		// taken from https://github.com/manwar/perlweeklychallenge-club/blob/master/challenge-175/mohammad-anwar/perl/ch-1.pl
    let correct = vec!["2022-01-30", "2022-02-27", "2022-03-27", "2022-04-24",
    "2022-05-29", "2022-06-26", "2022-07-31", "2022-08-28",
    "2022-09-25", "2022-10-30", "2022-11-27", "2022-12-25"].to_owned();
		let days: Vec<String> = get_last_day_of_months(2022,chrono::Weekday::Sun)
			.iter()
			.map(|d| d.to_string())
			.collect();
		assert!( days.len() == 12 );
		for e in zip( correct.iter(), days ) {
			assert!( e.0.to_owned() == e.1 );
		}
	}
}
