[comment]: # (auto_md_to_doc_comments segment start A)

# cargo_crev_web_admin

[comment]: # (auto_cargo_toml_to_md start)

**Admin CLI for cargo_crev_web**  
***version: 2022.623.1512 date: 2022-06-23 author: [bestia.dev](https://bestia.dev) repository: [Github](https://github.com/bestia-dev/cargo_crev_web_admin/)***  

[comment]: # (auto_cargo_toml_to_md end)

[comment]: # (auto_lines_of_code start)
[![Lines in Rust code](https://img.shields.io/badge/Lines_in_Rust-814-green.svg)](https://github.com/bestia-dev/cargo_crev_web_admin/)
[![Lines in Doc comments](https://img.shields.io/badge/Lines_in_Doc_comments-119-blue.svg)](https://github.com/bestia-dev/cargo_crev_web_admin/)
[![Lines in Comments](https://img.shields.io/badge/Lines_in_comments-97-purple.svg)](https://github.com/bestia-dev/cargo_crev_web_admin/)
[![Lines in examples](https://img.shields.io/badge/Lines_in_examples-0-yellow.svg)](https://github.com/bestia-dev/cargo_crev_web_admin/)
[![Lines in tests](https://img.shields.io/badge/Lines_in_tests-36-orange.svg)](https://github.com/bestia-dev/cargo_crev_web_admin/)

[comment]: # (auto_lines_of_code end)

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/bestia-dev/cargo_crev_web_admin/blob/main/LICENSE)
[![Rust](https://github.com/bestia-dev/cargo_crev_web_admin/workflows/RustAction/badge.svg)](https://github.com/bestia-dev/cargo_crev_web_admin/)
![Hits](https://bestia.dev/webpage_hit_counter/get_svg_image/911031110.svg)

Hashtags: #rustlang #buildtool #developmenttool #web #admin #cli  
My projects on Github are more like a tutorial than a finished product: [bestia-dev tutorials](https://github.com/bestia-dev/tutorials_rust_wasm).

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

## install cargo-crev and copy data from server

In the development environment inside a container I need the `cargo-crev` binary to run the commands. Fortunately there is a binary release already compiled here:
<https://github.com/crev-dev/cargo-crev/releases/download/v0.23.3/cargo-crev-v0.23.3-x86_64-unknown-linux-musl.tar.gz>
I unzip it and save the binary file cargo-crev in:`cp cargo-crev ~/.cargo/bin`, make it runnable `chmod +x ~/.cargo/bin/cargo-crev`
I like the editor nano more then vim: `git config --global core.editor "nano"`, this will work also for cargo-crev.

Now I need to import the `CrevId` from the server: copy the text from `wfx://FTP/google_vm_bestia_dev/home/luciano_bestia/.config/crev/ids/UpOPNplVEwBS2RhF7SS9gSP3bPJlfg-ZEoZ89gEMDwU.yaml` then on the development machine execute this command `cargo-crev crev id import`, paste the text and press ctrl+D to finish the import.
Check the import with `cargo-crev crev id current`.  

I need the ssh keys from the server to connect to the github remote repository. Download from `wfx://FTP/google_vm_bestia_dev/home/luciano_bestia/.ssh/` the files `web_crev_dev_for_github` and `web_crev_dev_for_github.pub`. Copy inside the container and then copy to .ssh folder :  
`cp web_crev_dev_for_github ~/.ssh/`, `cp web_crev_dev_for_github.pub ~/.ssh/`, make it private `chmod 400 ~/.ssh/web_crev_dev_for_github`
Be careful to not commit any secrets or private keys to github!  
Add the ssh key to the ssh-agent: `ssh-add ~/.ssh/web_crev_dev_for_github`
I need to configure the remote repository: `cargo crev id set-url https://github.com/web-crev-dev/crev-proofs`
To test add a `dpc` as trusted: `cargo-crev crev trust https://github.com/dpc/crev-proofs`

Now check the dir with `cargo-crev crev repo dir`.  
I got: `~/.local/share/crev/proofs/github_com_web-crev-dev_crev-proofs-POHSrDcUUmA6qBxSX6zy1w`
It looks crev changed the dir from ~/.config/crev to ~/.local/share/crev in some version. Be careful!

Inside the `sample_data` folder I copy the files from from the server `web.crev.dev`.  
`blocklisted_repos.json` is copied from `wfx://FTP/google_vm_bestia_dev/var/www/webapps/cargo_crev_web/blocklisted_repos.json`

Folder content of `trust` is copied from `wfx://FTP/google_vm_bestia_dev/home/luciano_bestia/.config/crev/proofs/github_com_cargo-crev-web_crev-proofs-NfdERRQ6ONoBLjIp0YbFVw/UpOPNplVEwBS2RhF7SS9gSP3bPJlfg-ZEoZ89gEMDwU/trust`
Now I need to copy the server trust files to the right folder for the development container:

```bash
mkdir -p ~/.local/share/crev/proofs/github_com_web-crev-dev_crev-proofs-POHSrDcUUmA6qBxSX6zy1w/UpOPNplVEwBS2RhF7SS9gSP3bPJlfg-ZEoZ89gEMDwU/trust
cp ~/rustprojects/cargo_crev_web_admin/sample_data/trust/*.* ~/.local/share/crev/proofs/github_com_web-crev-dev_crev-proofs-POHSrDcUUmA6qBxSX6zy1w/UpOPNplVEwBS2RhF7SS9gSP3bPJlfg-ZEoZ89gEMDwU/trust/
ls ~/.local/share/crev/proofs/github_com_web-crev-dev_crev-proofs-POHSrDcUUmA6qBxSX6zy1w/UpOPNplVEwBS2RhF7SS9gSP3bPJlfg-ZEoZ89gEMDwU/trust
cargo-crev crev id query trusted --high-cost 1 --medium-cost 1 --low-cost 1 --depth 1
```

This should list around 80 directly trusted proof-repos that are used on the server.  

## TODO

Integrity - warnings if a review have incorrect url or ID

## cargo-crev reviews and advisory

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
