# wpskim
A CLI tool that retrieves external links and e-mails from a WordPress site by gettings its pages and posts from its default REST API. 

![wpkskim](https://user-images.githubusercontent.com/6660327/228938554-42b19b5e-8c4f-448c-8c2e-b99abbc49210.gif)


## Basic use example:

`wpskim https://wordpress.org`

Outputs an ordered list of links and e-mails found in all visible pages and posts of wordpress.org, excluding those links that refer to wordpress.org itself. This list can be piped into other shell commands.


## Examples for other options:

All options can be found by using `wpskim -h`

You can list only links with `wpskim links -u https://wordpress.org`

You can download a `pages.json` and `posts.json` file with `wpskim -d -u https://wordpress.org`

If you run `wpskim` in a folder that contains these files, it wll instead read from them instead of making the corresponding requests.  


## Learning Rust?

It's an easy workflow if you want to build this tool for yourself. Get started by following [here](https://www.rust-lang.org/learn/get-started) the instructions to get rustup. This will install cargo in your system. Clone this repository and check that everything is working with `cargo build`, `cargo run https://wordpress.org` and `cargo test`. To build a binary you can use, the command is `cargo build --release` and you will find it in the `/target/release/` folder.

This is my first real Rust project. I stopped in the middle of reading the book to get something done that would help me learn it. This was also my first excuse to get to know a few of the Rust crates. Thanks for checking this out. 
