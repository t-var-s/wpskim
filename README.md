# wpskim
A CLI tool that retrieves external links, e-mails and documents from a WordPress site by gettings its pages, posts and media from its default REST API. 

![wpskim](https://github.com/user-attachments/assets/38d35236-8bae-42a1-a36b-a3dd18b3aa88)


## Basic use example:

`wpskim https://wordpress.org`

Outputs an ordered list of links, e-mails and documents found in all visible pages, posts and media of wordpress.org, excluding those links that refer to wordpress.org itself. This list can be piped into other shell commands.


## Examples for other options:

All options can be found by using `wpskim -h`

You can list only links with `wpskim links -u https://wordpress.org`

You can download `pages.json`, `posts.json` and `documents.json` files with `wpskim -d -u https://wordpress.org`

If you run `wpskim` in a folder that contains these files, it wll instead read from them instead of making the corresponding requests.  


## Released Builds

```
brew tap t-var-s/tap
brew install wpskim
```

## Cargo build it

It's an easy workflow if you want to build this tool for yourself. Get started by following [here](https://www.rust-lang.org/learn/get-started) the instructions to get rustup. This will install cargo in your system. Clone this repository and check that everything is working with `cargo build`, `cargo run https://wordpress.org` and `cargo test`. To build a binary you can use, the command is `cargo build --release` and you will find it in the `/target/release/` folder.
