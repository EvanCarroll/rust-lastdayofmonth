lastdayofmonth
====


Ever want to find all the last Sundays in 2022?

```sh
lastdayofmonth --year 2022 --day Sunday
```

This is an application to get all the last days of all months for a given a
year.


Building
----

```sh
cargo build --release
```

**Don't benchmark a testing suite, ever.** But if you insist, the test ported
1-to-1 as found in other languages in the Perl Weekly Challange #175. You can
run it like this.


```sh
cargo test --relesae
```

Also remember, the first run or build should be discarded as it _may_ included
compilation if not already done.

History
----

Originally created to get a Rust implementation for Mohammad Sajid Anwar
challange 175 for the Perl Weekly Challange.

You can find the [Facebook Post on the Perl Community here](https://www.facebook.com/groups/perlcommunity/posts/1265844890889801).
