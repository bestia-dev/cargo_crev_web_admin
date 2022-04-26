[comment]: # (auto_md_to_doc_comments segment start A)

# cargo_crev_web_admin

[comment]: # (auto_cargo_toml_to_md start)

**Admin CLI for cargo_crev_web**  
***version: 2022.128.1226  date: 2022-01-28 author: [bestia.dev](https://bestia.dev) repository: [GitHub](https://github.com/bestia-dev/cargo_crev_web_admin)***  

[comment]: # (auto_cargo_toml_to_md end)

[comment]: # (auto_lines_of_code start)
[![Lines in Rust code](https://img.shields.io/badge/Lines_in_Rust-819-green.svg)](https://github.com/bestia-dev/cargo_crev_web_admin/)
[![Lines in Doc comments](https://img.shields.io/badge/Lines_in_Doc_comments-120-blue.svg)](https://github.com/bestia-dev/cargo_crev_web_admin/)
[![Lines in Comments](https://img.shields.io/badge/Lines_in_comments-93-purple.svg)](https://github.com/bestia-dev/cargo_crev_web_admin/)
[![Lines in examples](https://img.shields.io/badge/Lines_in_examples-0-yellow.svg)](https://github.com/bestia-dev/cargo_crev_web_admin/)
[![Lines in tests](https://img.shields.io/badge/Lines_in_tests-36-orange.svg)](https://github.com/bestia-dev/cargo_crev_web_admin/)

[comment]: # (auto_lines_of_code end)

[![Licence](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/bestia-dev/cargo_crev_web_admin/blob/main/LICENSE)
[![Rust](https://github.com/bestia-dev/cargo_crev_web_admin/workflows/RustAction/badge.svg)](https://github.com/bestia-dev/cargo_crev_web_admin/)
[![Hits](https://hits.seeyoufarm.com/api/count/incr/badge.svg?url=https%3A%2F%2Fgithub.com%2Fbestia-dev%2Fcargo_crev_web_admin&count_bg=%2379C83D&title_bg=%23555555&icon=&icon_color=%23E7E7E7&title=hits&edge_flat=false)](https://hits.seeyoufarm.com)

Hashtags: #rustlang #buildtool #developmenttool #web #admin #cli

## cargo_crev_web_admin CLI

The web app cargo_crev_web on <https://web.crev.dev> fetches all proof repos it can find and shows the crate reviews online.  
Some admin tasks are needed and I don't want them to be accessible on the web.  
This will be a CLI app that can be used when logged on the linux terminal over SSH.  
So is sure that only an admin, who can log in on to the server, can use this tasks.

Some tasks need the crev passphrase. Put it in the env variable before starting the CLI:  
`$  export CREV_PASSPHRASE=xxx`  
Add a space before the command to avoid to be saved in the bash history.  

## Development

I use [cargo-auto](https://crates.io/crates/cargo-auto) for automation tasks in rust language. Install it:

```bash
cargo install cargo-auto
```

List user-defined automation tasks in `automation_tasks_rs`:

```bash
cargo auto
```

## bash auto-completion

This executable is prepared for auto-completion in bash.  
Run this command to define auto-completion in bash for the current session:  
Or add it to `.bashrc` file to be executed n every session start.

```bash
complete -C "cargo_crev_web_admin completion" cargo_crev_web_admin
```

To make it permanent add this command to the file `~/.bashrc` or some other file that runs commands on bash initialization.  

## TODO

Integrity - warnings if a review have incorrect url or ID

## cargo crev reviews and advisory

Please, spread this info !\
Open source code needs a community effort to express trustworthiness.\
Start with reading the reviews of the crates on [web.crev.dev](https://web.crev.dev/rust-reviews/crates). \
Then install the GUI [cargo_crev_reviews](https://crates.io/crates/cargo_crev_reviews) or the CLI [cargo-crev](https://github.com/crev-dev/cargo-crev)\.  
Your personal reviews are most important. If you have a boss, he will sooner or later ask you to show him your reviews for all the dependencies you use. With [cargo_crev_reviews](https://crates.io/crates/cargo_crev_reviews) you have a basic tool to do that. \
Write your reviews! Describe the crates you trust and why. Or warn about the crate versions you think are dangerous. Publish and share your opinion with other developers.\

## open-source and free as a beer

My open-source projects are free as a beer (MIT license).  
I just love programming.  
But I need also to drink. If you find my projects and tutorials helpful,  
please buy me a beer donating on my [paypal](https://paypal.me/LucianoBestia).  
You know the price of a beer in your local bar ;-)  
So I can drink a free beer for your health :-)  
[Na zdravje!](https://translate.google.com/?hl=en&sl=sl&tl=en&text=Na%20zdravje&op=translate) [Alla salute!](https://dictionary.cambridge.org/dictionary/italian-english/alla-salute) [Prost!](https://dictionary.cambridge.org/dictionary/german-english/prost) [Nazdravlje!](https://matadornetwork.com/nights/how-to-say-cheers-in-50-languages/) 🍻

[comment]: # (auto_md_to_doc_comments segment end A)
