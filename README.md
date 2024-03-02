[//]: # (auto_md_to_doc_comments segment start A)

# cargo_crev_web_admin

[//]: # (auto_cargo_toml_to_md start)

**Admin CLI for cargo_crev_web**  
***version: 2023.608.746 date: 2023-06-08 author: [bestia.dev](https://bestia.dev) repository: [Github](https://github.com/bestia-dev/cargo_crev_web_admin/)***  

 ![maintained](https://img.shields.io/badge/maintained-green)
 ![ready_for_use](https://img.shields.io/badge/ready_for_use-green)

[//]: # (auto_cargo_toml_to_md end)

[//]: # (auto_lines_of_code start)
[![Lines in Rust code](https://img.shields.io/badge/Lines_in_Rust-1113-green.svg)](https://github.com/bestia-dev/cargo_crev_web_admin/)
[![Lines in Doc comments](https://img.shields.io/badge/Lines_in_Doc_comments-207-blue.svg)](https://github.com/bestia-dev/cargo_crev_web_admin/)
[![Lines in Comments](https://img.shields.io/badge/Lines_in_comments-126-purple.svg)](https://github.com/bestia-dev/cargo_crev_web_admin/)
[![Lines in examples](https://img.shields.io/badge/Lines_in_examples-0-yellow.svg)](https://github.com/bestia-dev/cargo_crev_web_admin/)
[![Lines in tests](https://img.shields.io/badge/Lines_in_tests-36-orange.svg)](https://github.com/bestia-dev/cargo_crev_web_admin/)

[//]: # (auto_lines_of_code end)

 [![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/bestia-dev/cargo_crev_web_admin/blob/main/LICENSE)
 [![Rust](https://github.com/bestia-dev/cargo_crev_web_admin/workflows/RustAction/badge.svg)](https://github.com/bestia-dev/cargo_crev_web_admin/)
 ![cargo_crev_web_admin](https://bestia.dev/webpage_hit_counter/get_svg_image/911031110.svg)

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

## Prepare development environment

In the development environment inside a container I need the `cargo-crev` binary to run the commands. Fortunately there is a binary release already compiled:

```bash
curl -L -s https://github.com/crev-dev/cargo-crev/releases/download/v0.23.3/cargo-crev-v0.23.3-x86_64-unknown-linux-musl.tar.gz --output /tmp/cargo-crev.tar.gz
tar -xzv --no-same-owner --strip-components=1 -C ~/.cargo/bin -f /tmp/cargo-crev.tar.gz cargo-crev-v0.23.3-x86_64-unknown-linux-musl/cargo-crev
rm /tmp/cargo-crev.tar.gz
chmod +x ~/.cargo/bin/cargo-crev
git config --global core.editor "nano"
```

Now I need to import the `CrevId` from the server (ssh agent already has my ssh identity to connect to the server):  

```bash
scp luciano_bestia@bestia.dev:/home/luciano_bestia/.config/crev/ids/UpOPNplVEwBS2RhF7SS9gSP3bPJlfg-ZEoZ89gEMDwU.yaml .
# Connecting standard input to the file with <
cargo-crev crev id import <UpOPNplVEwBS2RhF7SS9gSP3bPJlfg-ZEoZ89gEMDwU.yaml
cargo-crev crev id current
rm UpOPNplVEwBS2RhF7SS9gSP3bPJlfg-ZEoZ89gEMDwU.yaml
```

I need the ssh keys from the server to connect to the github remote repository.  

```bash
scp luciano_bestia@bestia.dev:/home/luciano_bestia/.ssh/web_crev_dev_for_github.pub ~/.ssh/
scp luciano_bestia@bestia.dev:/home/luciano_bestia/.ssh/web_crev_dev_for_github ~/.ssh/
# Be careful to not commit any secrets or private keys to github! 
chmod 400 ~/.ssh/web_crev_dev_for_github
# Add the ssh key to your running ssh-agent
ssh-add ~/.ssh/web_crev_dev_for_github
# configure the remote repository 
cargo-crev crev id set-url https://github.com/web-crev-dev/crev-proofs
# To test add a `dpc` as trusted
cargo-crev crev trust https://github.com/dpc/crev-proofs
# Now check the dir with 
cargo-crev crev repo dir
```

I got: `~/.local/share/crev/proofs/github_com_web-crev-dev_crev-proofs-POHSrDcUUmA6qBxSX6zy1w`
It looks crev changed the dir from ~/.config/crev to ~/.local/share/crev in some version. Be careful!

On every session I will need to add the ssh key to the running ssh-agent:

```bash
ssh-add ~/.ssh/web_crev_dev_for_github
```

Copy the new crev data from the server for developing and debugging. The web.crev.dev has a special crev-id and should not interfere with other crev-ids on the system.  

```bash
rm ~/.local/share/crev/proofs/github_com_web-crev-dev_crev-proofs-POHSrDcUUmA6qBxSX6zy1w/UpOPNplVEwBS2RhF7SS9gSP3bPJlfg-ZEoZ89gEMDwU/blocklisted_repos.json
scp luciano_bestia@bestia.dev:/var/www/webapps/cargo_crev_web/blocklisted_repos.json ~/.local/share/crev/proofs/github_com_web-crev-dev_crev-proofs-POHSrDcUUmA6qBxSX6zy1w/UpOPNplVEwBS2RhF7SS9gSP3bPJlfg-ZEoZ89gEMDwU/blocklisted_repos.json
ls -l ~/.local/share/crev/proofs/github_com_web-crev-dev_crev-proofs-POHSrDcUUmA6qBxSX6zy1w/UpOPNplVEwBS2RhF7SS9gSP3bPJlfg-ZEoZ89gEMDwU

rm -r ~/.local/share/crev/proofs/github_com_web-crev-dev_crev-proofs-POHSrDcUUmA6qBxSX6zy1w/UpOPNplVEwBS2RhF7SS9gSP3bPJlfg-ZEoZ89gEMDwU/trust/

scp -r luciano_bestia@bestia.dev:/home/luciano_bestia/.local/share/crev/proofs/github_com_web-crev-dev_crev-proofs-POHSrDcUUmA6qBxSX6zy1w/UpOPNplVEwBS2RhF7SS9gSP3bPJlfg-ZEoZ89gEMDwU/trust/ ~/.local/share/crev/proofs/github_com_web-crev-dev_crev-proofs-POHSrDcUUmA6qBxSX6zy1w/UpOPNplVEwBS2RhF7SS9gSP3bPJlfg-ZEoZ89gEMDwU/
ls -l ~/.local/share/crev/proofs/github_com_web-crev-dev_crev-proofs-POHSrDcUUmA6qBxSX6zy1w/UpOPNplVEwBS2RhF7SS9gSP3bPJlfg-ZEoZ89gEMDwU/trust/
# list only the directly trusted repos
cargo-crev crev id query trusted --high-cost 1 --medium-cost 1 --low-cost 1 --depth 1
```

This should list around 80 directly trusted proof-repos that are used on the server.  

## TODO

Integrity - warnings if a review have incorrect url or ID

## Open-source and free as a beer

My open-source projects are free as a beer (MIT license).  
I just love programming.  
But I need also to drink. If you find my projects and tutorials helpful, please buy me a beer by donating to my [PayPal](https://paypal.me/LucianoBestia).  
You know the price of a beer in your local bar ;-)  
So I can drink a free beer for your health :-)  
[Na zdravje!](https://translate.google.com/?hl=en&sl=sl&tl=en&text=Na%20zdravje&op=translate) [Alla salute!](https://dictionary.cambridge.org/dictionary/italian-english/alla-salute) [Prost!](https://dictionary.cambridge.org/dictionary/german-english/prost) [Nazdravlje!](https://matadornetwork.com/nights/how-to-say-cheers-in-50-languages/) 🍻

[//bestia.dev](https://bestia.dev)  
[//github.com/bestia-dev](https://github.com/bestia-dev)  
[//bestiadev.substack.com](https://bestiadev.substack.com)  
[//youtube.com/@bestia-dev-tutorials](https://youtube.com/@bestia-dev-tutorials)  

[//]: # (auto_md_to_doc_comments segment end A)
