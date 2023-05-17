# gcmt

> Git Commit with Conventional Commit messages enforced.

## Why?

For years I have messed up with my commit messages. I did not know how to write
them, and I just wrote them in a way that felt intuitive to me. Often times,
words like `lmao` appear in my commit messages. I did not really find this to
be much problematic though. 

Things changed when I was building the app [meowly](www.meowly.app) with my
friend [salty-flower](https://github.com/salty-flower). He found my commits to
be very problematic. He told me about the 
[Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/). I tried
to write it, but I find it very hard for me to get the commits right every
time, especially given that I had been writing commits in a "freestyle" fashion
for a long time.

As a result, I decided to create a CLI to help me enforce this convention. I
name it gcmt, in short for `git commit`. I hope that it will be helpful to me
and help enforce better commit standards in my team.

gcmt at the current stage can be used, but it still lacks a lot of nice-to-have
features and may still have some bugs. I will make it better along the way.

## Install

To begin with, make sure that you have [`git`](https://git-scm.com) installed
(of course, what do you expect :-)

Then, you can download the latest `gcmt` binary from CI build (under the
[actions](https://github.com/wxxedu/gcmt/actions)) tab or from the
[release](https://github.com/wxxedu/gcmt/releases) tab. Note that for now, the
CI part is probably going to be more up-to-date because I still could not have
the auto-release CI set up correctly.

If you have [cargo](https://github.com/rust-lang/cargo) installed, then you can
also clone the repository and build for yourself. It should be as simple as
running the `cargo build --release` command.

After downloading the binary, you want to add it to your path. This depends on
your platform and the shell that you are using. Once this is done, you can
start using gcmt by typing `gcmt` in the terminal.

## Future Plans

There are several things that I wish to do in the future:

- **Custom Config**: I wish to add custom config to GCMT such that if you were 
  to use this app, you can custom define some of the currently hard-coded 
  values. Much of the structure is there for this change, but I still need to 
  get the implementation done. I wish to use `toml` for the config files as 
  they are easy to parse and easy to read.

- **Semantic Versioning**: I wish to add support for changing the semantic
  versioning automatically before each commit is done. Because I am a Rust,
  Flutter, and (future) React developer, I wish to get the semantic versioning
  done at least for these three languages/frameworks, and I wish this system
  could be designed in a way such that it is extensible / customizable by user
  config files.

- **GPT Integration**: I wish I could let GPT help me write my commit messages.
  Ideally, this would read from an environment variable your GPT token and use
  that to query OpenAI's api. I also wish that this system could be designed 
  in a way that allows for the use of other LLMs (or even locally hosted ones) 
  for writing commit messages.

## Why Rust & Disclaimer

I wrote this helper tool in Rust because I am currently learning rust. I am
amazed by its intricate type system. Because I am still a very new programmer
and very much new to Rust, much of the program that I write here may not be to
a very good quality. For example, I wanted to only use `&'a str`
references, but I could not get the life-time sorted out. 

