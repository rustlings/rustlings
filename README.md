![crab pet](https://i.imgur.com/LbZJgmm.gif)
<!-- ALL-CONTRIBUTORS-BADGE:START - Do not remove or modify this section -->
[![All Contributors](https://img.shields.io/badge/all_contributors-67-orange.svg?style=flat-square)](#contributors-)
<!-- ALL-CONTRIBUTORS-BADGE:END -->

# rustlings 🦀❤️ [![Build status](https://badge.buildkite.com/7af93d81dc522c67a1ec8e33ff5705861b1cb36360b774807f.svg)](https://buildkite.com/mokou/rustlings)

Greetings and welcome to `rustlings`. This project contains small exercises to get you used to reading and writing Rust code. This includes reading and responding to compiler messages!

_...looking for the old, web-based version of Rustlings? Try [here](https://github.com/rust-lang/rustlings/tree/rustlings-1)_

Alternatively, for a first-time Rust learner, there are several other resources:

- [The Book](https://doc.rust-lang.org/book/index.html) - The most comprehensive resource for learning Rust, but a bit theoretical sometimes. You will be using this along with Rustlings!
- [Rust By Example](https://doc.rust-lang.org/rust-by-example/index.html) - Learn Rust by solving little exercises! It's almost like `rustlings`, but online

## Getting Started

_Note: If you're on MacOS, make sure you've installed Xcode and its developer tools by typing `xcode-select --install`._

You will need to have Rust installed. You can get it by visiting https://rustup.rs. This'll also install Cargo, Rust's package/project manager.

## MacOS/Linux

Just run:

```bash
curl -L https://git.io/rustlings | bash
# Or if you want it to be installed to a different path:
curl -L https://git.io/rustlings | bash -s mypath/
```

This will install Rustlings and give you access to the `rustlings` command. Run it to get started!

## Windows

In PowerShell, set `ExecutionPolicy` to `RemoteSigned`:

```ps
Set-ExecutionPolicy RemoteSigned
```

Then, you can run:

```ps
Start-BitsTransfer -Source https://git.io/rustlings-win -Destination $env:TMP/install_rustlings.ps1; Unblock-File $env:TMP/install_rustlings.ps1; Invoke-Expression $env:TMP/install_rustlings.ps1
```

To install Rustlings. Same as on MacOS/Linux, you will have access to the `rustlings` command after it.

## Browser:

[Run on Repl.it](https://repl.it/github/rust-lang/rustlings)

[Open in Gitpod](https://gitpod.io/#https://github.com/rust-lang/rustlings)

## Manually

Basically: Clone the repository, checkout to the latest tag, run `cargo install`.

```bash
git clone https://github.com/rust-lang/rustlings
cd rustlings
git checkout tags/4.1.0 # or whatever the latest version is (find out at https://github.com/rust-lang/rustlings/releases/latest)
cargo install --force --path .
```

If there are installation errors, ensure that your toolchain is up to date. For the latest, run:
```bash
rustup update
```

Then, same as above, run `rustlings` to get started.

## Doing exercises

The exercises are sorted by topic and can be found in the subdirectory `rustlings/exercises/<topic>`. For every topic there is an additional README file with some resources to get you started on the topic. We really recommend that you have a look at them before you start.

The task is simple. Most exercises contain an error that keeps them from compiling, and it's up to you to fix it! Some exercises are also run as tests, but rustlings handles them all the same. To run the exercises in the recommended order, execute:

```bash
rustlings watch
```

This will try to verify the completion of every exercise in a predetermined order (what we think is best for newcomers). It will also rerun automatically every time you change a file in the `exercises/` directory. If you want to only run it once, you can use:

```bash
rustlings verify
```

This will do the same as watch, but it'll quit after running.

In case you want to go by your own order, or want to only verify a single exercise, you can run:

```bash
rustlings run myExercise1
```

In case you get stuck, you can run the following command to get a hint for your
exercise:

``` bash
rustlings hint myExercise1
```

### Using an IDE
**TL,DR**: open the `exercises/` folder in your editor while running `rustlings watch`.

There are several editors and plugins that are able to analyze Rust code for a full blown IDE (or IDE-like) experience. The official language plugin of the Rust project is [rust-analyzer](rust-analyzer), but there are others, like [IntelliJ Rust](intellij) or [rls](rls).

These tools usually rely on the default structure of a Cargo project to perform their analyses. Conversely, Rustlings has a _atypical_ structure: it's a collection of isolated exercises. Your editor might not be able to properly analyze the code of the exercises.

However, if you run `rustlings watch`, we auto-generate a "fake" project manifest in `exercises/Cargo.toml` that points to the currently active exercise. If you open the `exercises/` folder with your Rust-enabled editor, you should be able to enjoy its capabilities. Note that we update the manifest so it always points to the current exercise, so ideally your editor should be able to refresh the project metadata automatically.

[rust-analyzer]: https://rust-analyzer.github.io/
[rls]: https://github.com/rust-lang/rls
[intellij]: https://intellij-rust.github.io/

## Testing yourself

After every couple of sections, there will be a quiz that'll test your knowledge on a bunch of sections at once. These quizzes are found in `exercises/quizN.rs`.

## Continuing On

Once you've completed Rustlings, put your new knowledge to good use! Continue practicing your Rust skills by building your own projects, contributing to Rustlings, or finding other open-source projects to contribute to.

If you'd like to uninstall Rustlings, you can do so by invoking cargo and removing the rustlings directory:

```bash
cargo uninstall rustlings
rm -r rustlings/ # or on Windows: rmdir /s rustlings
```

## Completion

Rustlings isn't done; there are a couple of sections that are very experimental and don't have proper documentation. These include:

- Errors (`exercises/errors/`)
- Option (`exercises/option/`)
- Result (`exercises/result/`)
- Move Semantics (could still be improved, `exercises/move_semantics/`)

Additionally, we could use exercises on a couple of topics:

- Structs
- Better ownership stuff
- `impl`
- ??? probably more

If you are interested in improving or adding new ones, please feel free to contribute! Read on for more information :)

## Contributing

See [CONTRIBUTING.md](./CONTRIBUTING.md).

## Contributors ✨

Thanks goes to these wonderful people ([emoji key](https://allcontributors.org/docs/en/emoji-key)):

<!-- ALL-CONTRIBUTORS-LIST:START - Do not remove or modify this section -->
<!-- prettier-ignore-start -->
<!-- markdownlint-disable -->
<table>
  <tr>
    <td align="center"><a href="http://carol-nichols.com"><img src="https://avatars2.githubusercontent.com/u/193874?v=4" width="100px;" alt=""/><br /><sub><b>Carol (Nichols &#124;&#124; Goulding)</b></sub></a><br /><a href="https://github.com/rust-lang/rustlings/commits?author=carols10cents" title="Code">💻</a> <a href="#content-carols10cents" title="Content">🖋</a></td>
    <td align="center"><a href="https://twitter.com/QuietMisdreavus"><img src="https://avatars2.githubusercontent.com/u/5217170?v=4" width="100px;" alt=""/><br /><sub><b>QuietMisdreavus</b></sub></a><br /><a href="https://github.com/rust-lang/rustlings/commits?author=QuietMisdreavus" title="Code">💻</a> <a href="#content-QuietMisdreavus" title="Content">🖋</a></td>
    <td align="center"><a href="https://github.com/robertlugg"><img src="https://avatars0.githubusercontent.com/u/6054540?v=4" width="100px;" alt=""/><br /><sub><b>Robert M Lugg</b></sub></a><br /><a href="#content-robertlugg" title="Content">🖋</a></td>
    <td align="center"><a href="https://hynek.me/about/"><img src="https://avatars3.githubusercontent.com/u/41240?v=4" width="100px;" alt=""/><br /><sub><b>Hynek Schlawack</b></sub></a><br /><a href="https://github.com/rust-lang/rustlings/commits?author=hynek" title="Code">💻</a></td>
    <td align="center"><a href="https://spacekookie.de"><img src="https://avatars0.githubusercontent.com/u/7669898?v=4" width="100px;" alt=""/><br /><sub><b>Katharina Fey</b></sub></a><br /><a href="https://github.com/rust-lang/rustlings/commits?author=spacekookie" title="Code">💻</a></td>
    <td align="center"><a href="https://github.com/lukabavdaz"><img src="https://avatars0.githubusercontent.com/u/9624558?v=4" width="100px;" alt=""/><br /><sub><b>lukabavdaz</b></sub></a><br /><a href="https://github.com/rust-lang/rustlings/commits?author=lukabavdaz" title="Code">💻</a> <a href="#content-lukabavdaz" title="Content">🖋</a></td>
    <td align="center"><a href="http://vestera.as"><img src="https://avatars2.githubusercontent.com/u/4187449?v=4" width="100px;" alt=""/><br /><sub><b>Erik Vesteraas</b></sub></a><br /><a href="https://github.com/rust-lang/rustlings/commits?author=evestera" title="Code">💻</a></td>
    <td align="center"><a href="https://github.com/Delet0r"><img src="https://avatars1.githubusercontent.com/u/23195618?v=4" width="100px;" alt=""/><br /><sub><b>delet0r</b></sub></a><br /><a href="https://github.com/rust-lang/rustlings/commits?author=Delet0r" title="Code">💻</a></td>
  </tr>
  <tr>
    <td align="center"><a href="http://phinary.ca"><img src="https://avatars1.githubusercontent.com/u/10522375?v=4" width="100px;" alt=""/><br /><sub><b>Shaun Bennett</b></sub></a><br /><a href="https://github.com/rust-lang/rustlings/commits?author=shaunbennett" title="Code">💻</a></td>
    <td align="center"><a href="https://github.com/abagshaw"><img src="https://avatars2.githubusercontent.com/u/8594541?v=4" width="100px;" alt=""/><br /><sub><b>Andrew Bagshaw</b></sub></a><br /><a href="https://github.com/rust-lang/rustlings/commits?author=abagshaw" title="Code">💻</a></td>
    <td align="center"><a href="https://ai6ua.net/"><img src="https://avatars2.githubusercontent.com/u/175578?v=4" width="100px;" alt=""/><br /><sub><b>Kyle Isom</b></sub></a><br /><a href="https://github.com/rust-lang/rustlings/commits?author=kisom" title="Code">💻</a></td>
    <td align="center"><a href="https://github.com/ColinPitrat"><img src="https://avatars3.githubusercontent.com/u/1541863?v=4" width="100px;" alt=""/><br /><sub><b>Colin Pitrat</b></sub></a><br /><a href="https://github.com/rust-lang/rustlings/commits?author=ColinPitrat" title="Code">💻</a></td>
    <td align="center"><a href="https://zacanger.com"><img src="https://avatars3.githubusercontent.com/u/12520493?v=4" width="100px;" alt=""/><br /><sub><b>Zac Anger</b></sub></a><br /><a href="https://github.com/rust-lang/rustlings/commits?author=zacanger" title="Code">💻</a></td>
    <td align="center"><a href="https://github.com/mgeier"><img src="https://avatars1.githubusercontent.com/u/705404?v=4" width="100px;" alt=""/><br /><sub><b>Matthias Geier</b></sub></a><br /><a href="https://github.com/rust-lang/rustlings/commits?author=mgeier" title="Code">💻</a></td>
    <td align="center"><a href="https://github.com/cjpearce"><img src="https://avatars1.githubusercontent.com/u/3453268?v=4" width="100px;" alt=""/><br /><sub><b>Chris Pearce</b></sub></a><br /><a href="https://github.com/rust-lang/rustlings/commits?author=cjpearce" title="Code">💻</a></td>
    <td align="center"><a href="https://yvan-sraka.github.io"><img src="https://avatars2.githubusercontent.com/u/705213?v=4" width="100px;" alt=""/><br /><sub><b>Yvan Sraka</b></sub></a><br /><a href="https://github.com/rust-lang/rustlings/commits?author=yvan-sraka" title="Code">💻</a></td>
  </tr>
  <tr>
    <td align="center"><a href="https://github.com/dendi239"><img src="https://avatars3.githubusercontent.com/u/16478650?v=4" width="100px;" alt=""/><br /><sub><b>Denys Smirnov</b></sub></a><br /><a href="https://github.com/rust-lang/rustlings/commits?author=dendi239" title="Code">💻</a></td>
    <td align="center"><a href="https://github.com/eddyp"><img src="https://avatars2.githubusercontent.com/u/123772?v=4" width="100px;" alt=""/><br /><sub><b>eddyp</b></sub></a><br /><a href="https://github.com/rust-lang/rustlings/commits?author=eddyp" title="Code">💻</a></td>
    <td align="center"><a href="http://about.me/BrianKung"><img src="https://avatars1.githubusercontent.com/u/2836167?v=4" width="100px;" alt=""/><br /><sub><b>Brian Kung</b></sub></a><br /><a href="https://github.com/rust-lang/rustlings/commits?author=briankung" title="Code">💻</a> <a href="#content-briankung" title="Content">🖋</a></td>
    <td align="center"><a href="https://rcousineau.gitlab.io"><img src="https://avatars3.githubusercontent.com/u/281039?v=4" width="100px;" alt=""/><br /><sub><b>Russell</b></sub></a><br /><a href="https://github.com/rust-lang/rustlings/commits?author=miller-time" title="Code">💻</a></td>
    <td align="center"><a href="http://danwilhelm.com"><img src="https://avatars3.githubusercontent.com/u/6137185?v=4" width="100px;" alt=""/><br /><sub><b>Dan Wilhelm</b></sub></a><br /><a href="https://github.com/rust-lang/rustlings/commits?author=danwilhelm" title="Documentation">📖</a></td>
    <td align="center"><a href="https://github.com/Jesse-Cameron"><img src="https://avatars3.githubusercontent.com/u/3723654?v=4" width="100px;" alt=""/><br /><sub><b>Jesse</b></sub></a><br /><a href="https://github.com/rust-lang/rustlings/commits?author=Jesse-Cameron" title="Code">💻</a> <a href="#content-Jesse-Cameron" title="Content">🖋</a></td>
    <td align="center"><a href="https://github.com/MrFroop"><img src="https://avatars3.githubusercontent.com/u/196700?v=4" width="100px;" alt=""/><br /><sub><b>Fredrik Jambrén</b></sub></a><br /><a href="https://github.com/rust-lang/rustlings/commits?author=MrFroop" title="Code">💻</a></td>
    <td align="center"><a href="https://github.com/petemcfarlane"><img src="https://avatars3.githubusercontent.com/u/3472717?v=4" width="100px;" alt=""/><br /><sub><b>Pete McFarlane</b></sub></a><br /><a href="#content-petemcfarlane" title="Content">🖋</a></td>
  </tr>
  <tr>
    <td align="center"><a href="https://github.com/nkanderson"><img src="https://avatars0.githubusercontent.com/u/4128825?v=4" width="100px;" alt=""/><br /><sub><b>nkanderson</b></sub></a><br /><a href="https://github.com/rust-lang/rustlings/commits?author=nkanderson" title="Code">💻</a> <a href="#content-nkanderson" title="Content">🖋</a></td>
    <td align="center"><a href="https://github.com/ajaxm"><img src="https://avatars0.githubusercontent.com/u/13360138?v=4" width="100px;" alt=""/><br /><sub><b>Ajax M</b></sub></a><br /><a href="https://github.com/rust-lang/rustlings/commits?author=ajaxm" title="Documentation">📖</a></td>
    <td align="center"><a href="https://dylnuge.com"><img src="https://avatars2.githubusercontent.com/u/118624?v=4" width="100px;" alt=""/><br /><sub><b>Dylan Nugent</b></sub></a><br /><a href="#content-Dylnuge" title="Content">🖋</a></td>
    <td align="center"><a href="https://github.com/vyaslav"><img src="https://avatars0.githubusercontent.com/u/1385427?v=4" width="100px;" alt=""/><br /><sub><b>vyaslav</b></sub></a><br /><a href="https://github.com/rust-lang/rustlings/commits?author=vyaslav" title="Code">💻</a> <a href="#content-vyaslav" title="Content">🖋</a></td>
    <td align="center"><a href="https://join.sfxd.org"><img src="https://avatars1.githubusercontent.com/u/17297466?v=4" width="100px;" alt=""/><br /><sub><b>George</b></sub></a><br /><a href="https://github.com/rust-lang/rustlings/commits?author=gdoenlen" title="Code">💻</a></td>
    <td align="center"><a href="https://github.com/nyxtom"><img src="https://avatars2.githubusercontent.com/u/222763?v=4" width="100px;" alt=""/><br /><sub><b>Thomas Holloway</b></sub></a><br /><a href="https://github.com/rust-lang/rustlings/commits?author=nyxtom" title="Code">💻</a> <a href="#content-nyxtom" title="Content">🖋</a></td>
    <td align="center"><a href="https://github.com/workingjubilee"><img src="https://avatars1.githubusercontent.com/u/46493976?v=4" width="100px;" alt=""/><br /><sub><b>Jubilee</b></sub></a><br /><a href="https://github.com/rust-lang/rustlings/commits?author=workingjubilee" title="Code">💻</a></td>
    <td align="center"><a href="https://github.com/WofWca"><img src="https://avatars1.githubusercontent.com/u/39462442?v=4" width="100px;" alt=""/><br /><sub><b>WofWca</b></sub></a><br /><a href="https://github.com/rust-lang/rustlings/commits?author=WofWca" title="Code">💻</a></td>
  </tr>
  <tr>
    <td align="center"><a href="https://github.com/jrvidal"><img src="https://avatars0.githubusercontent.com/u/1636604?v=4" width="100px;" alt=""/><br /><sub><b>Roberto Vidal</b></sub></a><br /><a href="https://github.com/rust-lang/rustlings/commits?author=jrvidal" title="Code">💻</a> <a href="https://github.com/rust-lang/rustlings/commits?author=jrvidal" title="Documentation">📖</a> <a href="#ideas-jrvidal" title="Ideas, Planning, & Feedback">🤔</a> <a href="#maintenance-jrvidal" title="Maintenance">🚧</a></td>
    <td align="center"><a href="https://github.com/jensim"><img src="https://avatars0.githubusercontent.com/u/3663856?v=4" width="100px;" alt=""/><br /><sub><b>Jens</b></sub></a><br /><a href="https://github.com/rust-lang/rustlings/commits?author=jensim" title="Documentation">📖</a></td>
    <td align="center"><a href="http://rahatah.me/d"><img src="https://avatars3.githubusercontent.com/u/3174006?v=4" width="100px;" alt=""/><br /><sub><b>Rahat Ahmed</b></sub></a><br /><a href="https://github.com/rust-lang/rustlings/commits?author=rahatarmanahmed" title="Documentation">📖</a></td>
    <td align="center"><a href="https://github.com/AbdouSeck"><img src="https://avatars2.githubusercontent.com/u/6490055?v=4" width="100px;" alt=""/><br /><sub><b>Abdou Seck</b></sub></a><br /><a href="https://github.com/rust-lang/rustlings/commits?author=AbdouSeck" title="Code">💻</a> <a href="#content-AbdouSeck" title="Content">🖋</a> <a href="https://github.com/rust-lang/rustlings/pulls?q=is%3Apr+reviewed-by%3AAbdouSeck" title="Reviewed Pull Requests">👀</a></td>
    <td align="center"><a href="https://codehearts.com"><img src="https://avatars0.githubusercontent.com/u/2885412?v=4" width="100px;" alt=""/><br /><sub><b>Katie</b></sub></a><br /><a href="https://github.com/rust-lang/rustlings/commits?author=codehearts" title="Code">💻</a></td>
    <td align="center"><a href="https://github.com/Socratides"><img src="https://avatars3.githubusercontent.com/u/27732983?v=4" width="100px;" alt=""/><br /><sub><b>Socrates</b></sub></a><br /><a href="https://github.com/rust-lang/rustlings/commits?author=Socratides" title="Documentation">📖</a></td>
    <td align="center"><a href="https://github.com/gnodarse"><img src="https://avatars3.githubusercontent.com/u/46761795?v=4" width="100px;" alt=""/><br /><sub><b>gnodarse</b></sub></a><br /><a href="#content-gnodarse" title="Content">🖋</a></td>
    <td align="center"><a href="https://github.com/harrisonmetz"><img src="https://avatars1.githubusercontent.com/u/7883408?v=4" width="100px;" alt=""/><br /><sub><b>Harrison Metzger</b></sub></a><br /><a href="https://github.com/rust-lang/rustlings/commits?author=harrisonmetz" title="Code">💻</a></td>
  </tr>
  <tr>
    <td align="center"><a href="https://github.com/TorbenJ"><img src="https://avatars2.githubusercontent.com/u/9077102?v=4" width="100px;" alt=""/><br /><sub><b>Torben Jonas</b></sub></a><br /><a href="https://github.com/rust-lang/rustlings/commits?author=TorbenJ" title="Code">💻</a> <a href="#content-TorbenJ" title="Content">🖋</a></td>
    <td align="center"><a href="http://paulbissex.com/"><img src="https://avatars0.githubusercontent.com/u/641?v=4" width="100px;" alt=""/><br /><sub><b>Paul Bissex</b></sub></a><br /><a href="https://github.com/rust-lang/rustlings/commits?author=pbx" title="Documentation">📖</a></td>
    <td align="center"><a href="https://github.com/sjmann"><img src="https://avatars0.githubusercontent.com/u/6589896?v=4" width="100px;" alt=""/><br /><sub><b>Steven Mann</b></sub></a><br /><a href="https://github.com/rust-lang/rustlings/commits?author=sjmann" title="Code">💻</a> <a href="#content-sjmann" title="Content">🖋</a></td>
    <td align="center"><a href="https://smmdb.net/"><img src="https://avatars2.githubusercontent.com/u/5855071?v=4" width="100px;" alt=""/><br /><sub><b>Mario Reder</b></sub></a><br /><a href="https://github.com/rust-lang/rustlings/commits?author=Tarnadas" title="Code">💻</a> <a href="#content-Tarnadas" title="Content">🖋</a></td>
    <td align="center"><a href="https://keybase.io/skim"><img src="https://avatars0.githubusercontent.com/u/47347?v=4" width="100px;" alt=""/><br /><sub><b>skim</b></sub></a><br /><a href="https://github.com/rust-lang/rustlings/commits?author=sl4m" title="Code">💻</a></td>
    <td align="center"><a href="https://github.com/sanjaykdragon"><img src="https://avatars1.githubusercontent.com/u/10261698?v=4" width="100px;" alt=""/><br /><sub><b>Sanjay K</b></sub></a><br /><a href="https://github.com/rust-lang/rustlings/commits?author=sanjaykdragon" title="Code">💻</a> <a href="#content-sanjaykdragon" title="Content">🖋</a></td>
    <td align="center"><a href="http://www.rohanjain.in"><img src="https://avatars1.githubusercontent.com/u/343499?v=4" width="100px;" alt=""/><br /><sub><b>Rohan Jain</b></sub></a><br /><a href="https://github.com/rust-lang/rustlings/commits?author=crodjer" title="Code">💻</a></td>
    <td align="center"><a href="https://www.saidaspen.se"><img src="https://avatars1.githubusercontent.com/u/7727687?v=4" width="100px;" alt=""/><br /><sub><b>Said Aspen</b></sub></a><br /><a href="https://github.com/rust-lang/rustlings/commits?author=saidaspen" title="Code">💻</a> <a href="#content-saidaspen" title="Content">🖋</a></td>
  </tr>
  <tr>
    <td align="center"><a href="https://github.com/uce"><img src="https://avatars3.githubusercontent.com/u/1756620?v=4" width="100px;" alt=""/><br /><sub><b>Ufuk Celebi</b></sub></a><br /><a href="https://github.com/rust-lang/rustlings/commits?author=uce" title="Code">💻</a></td>
    <td align="center"><a href="https://github.com/lebedevsergey"><img src="https://avatars2.githubusercontent.com/u/7325764?v=4" width="100px;" alt=""/><br /><sub><b>lebedevsergey</b></sub></a><br /><a href="https://github.com/rust-lang/rustlings/commits?author=lebedevsergey" title="Documentation">📖</a></td>
    <td align="center"><a href="https://github.com/avrong"><img src="https://avatars2.githubusercontent.com/u/6342851?v=4" width="100px;" alt=""/><br /><sub><b>Aleksei Trifonov</b></sub></a><br /><a href="#content-avrong" title="Content">🖋</a></td>
    <td align="center"><a href="https://drn.ie"><img src="https://avatars2.githubusercontent.com/u/411136?v=4" width="100px;" alt=""/><br /><sub><b>Darren Meehan</b></sub></a><br /><a href="#content-Darrenmeehan" title="Content">🖋</a></td>
    <td align="center"><a href="https://github.com/jihchi"><img src="https://avatars1.githubusercontent.com/u/87983?v=4" width="100px;" alt=""/><br /><sub><b>Jihchi Lee</b></sub></a><br /><a href="#content-jihchi" title="Content">🖋</a></td>
    <td align="center"><a href="https://github.com/bertonha"><img src="https://avatars3.githubusercontent.com/u/1225902?v=4" width="100px;" alt=""/><br /><sub><b>Christofer Bertonha</b></sub></a><br /><a href="#content-bertonha" title="Content">🖋</a></td>
    <td align="center"><a href="https://github.com/apatniv"><img src="https://avatars2.githubusercontent.com/u/22565917?v=4" width="100px;" alt=""/><br /><sub><b>Vivek Bharath Akupatni</b></sub></a><br /><a href="https://github.com/rust-lang/rustlings/commits?author=apatniv" title="Code">💻</a> <a href="https://github.com/rust-lang/rustlings/commits?author=apatniv" title="Tests">⚠️</a></td>
    <td align="center"><a href="https://github.com/DiD92"><img src="https://avatars3.githubusercontent.com/u/6002416?v=4" width="100px;" alt=""/><br /><sub><b>Dídac Sementé Fernández</b></sub></a><br /><a href="https://github.com/rust-lang/rustlings/commits?author=DiD92" title="Code">💻</a> <a href="#content-DiD92" title="Content">🖋</a></td>
  </tr>
  <tr>
    <td align="center"><a href="https://github.com/wrobstory"><img src="https://avatars3.githubusercontent.com/u/2601457?v=4" width="100px;" alt=""/><br /><sub><b>Rob Story</b></sub></a><br /><a href="https://github.com/rust-lang/rustlings/commits?author=wrobstory" title="Code">💻</a></td>
    <td align="center"><a href="https://github.com/siobhanjacobson"><img src="https://avatars2.githubusercontent.com/u/28983835?v=4" width="100px;" alt=""/><br /><sub><b>Siobhan Jacobson</b></sub></a><br /><a href="https://github.com/rust-lang/rustlings/commits?author=siobhanjacobson" title="Code">💻</a></td>
    <td align="center"><a href="https://www.linkedin.com/in/evancarroll/"><img src="https://avatars2.githubusercontent.com/u/19922?v=4" width="100px;" alt=""/><br /><sub><b>Evan Carroll</b></sub></a><br /><a href="#content-EvanCarroll" title="Content">🖋</a></td>
    <td align="center"><a href="http://www.jawaadmahmood.com"><img src="https://avatars3.githubusercontent.com/u/95606?v=4" width="100px;" alt=""/><br /><sub><b>Jawaad Mahmood</b></sub></a><br /><a href="#content-jmahmood" title="Content">🖋</a></td>
    <td align="center"><a href="https://github.com/GaurangTandon"><img src="https://avatars1.githubusercontent.com/u/6308683?v=4" width="100px;" alt=""/><br /><sub><b>Gaurang Tandon</b></sub></a><br /><a href="#content-GaurangTandon" title="Content">🖋</a></td>
    <td align="center"><a href="https://github.com/dev-cyprium"><img src="https://avatars1.githubusercontent.com/u/6002628?v=4" width="100px;" alt=""/><br /><sub><b>Stefan Kupresak</b></sub></a><br /><a href="#content-dev-cyprium" title="Content">🖋</a></td>
    <td align="center"><a href="https://github.com/greg-el"><img src="https://avatars3.githubusercontent.com/u/45019882?v=4" width="100px;" alt=""/><br /><sub><b>Greg Leonard</b></sub></a><br /><a href="#content-greg-el" title="Content">🖋</a></td>
    <td align="center"><a href="https://ryanpcmcquen.org"><img src="https://avatars3.githubusercontent.com/u/772937?v=4" width="100px;" alt=""/><br /><sub><b>Ryan McQuen</b></sub></a><br /><a href="https://github.com/rust-lang/rustlings/commits?author=ryanpcmcquen" title="Code">💻</a></td>
  </tr>
  <tr>
    <td align="center"><a href="https://github.com/AnnikaCodes"><img src="https://avatars3.githubusercontent.com/u/56906084?v=4" width="100px;" alt=""/><br /><sub><b>Annika</b></sub></a><br /><a href="https://github.com/rust-lang/rustlings/pulls?q=is%3Apr+reviewed-by%3AAnnikaCodes" title="Reviewed Pull Requests">👀</a></td>
    <td align="center"><a href="https://darnuria.eu"><img src="https://avatars1.githubusercontent.com/u/2827553?v=4" width="100px;" alt=""/><br /><sub><b>Axel Viala</b></sub></a><br /><a href="https://github.com/rust-lang/rustlings/commits?author=darnuria" title="Code">💻</a></td>
    <td align="center"><a href="https://sazid.github.io"><img src="https://avatars1.githubusercontent.com/u/2370167?v=4" width="100px;" alt=""/><br /><sub><b>Mohammed Sazid Al Rashid</b></sub></a><br /><a href="#content-sazid" title="Content">🖋</a> <a href="https://github.com/rust-lang/rustlings/commits?author=sazid" title="Code">💻</a></td>
  </tr>
</table>

<!-- markdownlint-enable -->
<!-- prettier-ignore-end -->
<!-- ALL-CONTRIBUTORS-LIST:END -->

This project follows the [all-contributors](https://github.com/all-contributors/all-contributors) specification. Contributions of any kind welcome!
