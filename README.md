# VIdeas

## Summary 
> Your blog, using your VI/m editor (yeah that weird you ~and I~ are). 

Using one of the fastest, most efficient and secure frameworks, ~Ruby on Rails (haha kidding)~ Actix web (Rust-based web framework), with simple Rust crates to manage the DB and the templates.

This project is basically a crud that will show static content using a `jinja`-like template engine. Having the ability to use VI or VIm as your blog entries editor as **one of the most interesting _"features"_**.

## Features
- Use Vim and Markdown, forget writing HTML/CSS or using [these](https://wordpress.org/support/files/2018/10/add-new-post.png) ugly posts editors. Use what we've been using as devs.
- Get it running in minutes
- Fly.io-ready deploy (you may just need to change some configuration)
- Simple enough. Easy for contributors to join.
- You just worry on writing your content. A magic bash script will do the rest of stuff for you.*
- And more coming soon...
    

\* More stuff coming (tags, abstract, covers, etc)

## Running

### Docker
I recommend using `Docker` in order to save time. Even though these docker configurations are not the most efficient you'll see they get the things running. I plan to create a `distroless iamge` after completing the main features of the blog.

```bash
docker-compose up -d
```
Alternatively you can use this utility called `docker-app`: [diegoacuna/docker-app](https://github.com/diegoacuna/docker-app)

### Cargo
If you prefer to use cargo directly then run these commands (same you'll find on the Dockerfile):
```bash
cargo install diesel_cli --no-default-features --features sqlite # ORM
# Then
diesel setup # Migrations
# Finally
cargo run
```
#### Cargo watch
During development it's better to use `cargo watch` in order to save time when making changes and trying to testing them in the browser. You can find the repo [here](https://github.com/watchexec/cargo-watch)
```bash
cargo watch -c --why -d 5 -x run # this is the one I use
```
> These instructions might be not entirely accurate, please open an issue if you face any trouble.

## Customisation
To add your picture, name, links, and so forth, you can go to `templates/config`. You'll find a set of HTML files ready for customisation. The only thing you won't find here is the title of the main page, which can be edited in `templates/base.html`.

Remember, this project is open source. Meaning that you can navigate through the code to customise whatever you want or need.

## Writing a post
To create a post it's simple as opening the `write_entry` script:
```bash
bash write_entry.sh
```
This will open a vim editor. And that's it, once you finish writing your post using MD it should be saved into the DB.

> Please take in consideration that this project is still in progress. You may find vulnerabilities and issues in the process of creating a post. In any case you can refer to the `tests/bash` folder to check the status of the work regarding bash scripts. (you can run `bash tests/bash/runAll.sh` from the root directory.

## Personal note from me
I started this project one night I got anxious because I lost my Scribe stylus and had to write down my daily report (therapy stuff). And after 10 minutes I got a funcional (albeit ugly version running on my home server). My objectives were:
1. Don't use too complicated stuff (as I intented to design at the beginning)
2. Use Vim
3. Use MD (I love MD)
4. make it interesting (use Rust and other cool stuff; learn it on the fly)

That said, please bear with any mistakes my unconscious self may have made. I plan to clean up this project to ensure that setting up your blog is a matter of minutes.

> If you find your work (or parts of it) and it's not being properly used (wrong use of the license for instance), please refer to <hello@itsgerard.com>.
