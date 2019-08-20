[![Build Status](https://travis-ci.org/S4ndyk/stomper.svg?branch=master)](https://travis-ci.org/S4ndyk/stomper)
[![Coverage Status](https://coveralls.io/repos/github/S4ndyk/stomper/badge.svg)](https://coveralls.io/github/S4ndyk/stomper)

Stomper
===
**Käyttöohje vertaisarvioijalle**
- [Stomper](#Stomper)
    - [Asennus](#Asennus)
    - [Käyttöohje](#K%C3%A4ytt%C3%B6ohje)
      - [Esimerkki](#Esimerkki)


**Dokumentaatio**
* [Määrittely](/documentation/maaritelma.md)
* [stomper documentaatio](https://docs.rs/stomper/)
* [libstomper dokumentaatio](https://docs.rs/libstomper/)

**Viikkoraportit**
* [Viikko 1](/documentation/viikkoraportit/viikko1.md)
* [Viikko 2](/documentation/viikkoraportit/viikko2.md)
* [Viikko 3](/documentation/viikkoraportit/viikko3.md)
* [Viikko 4](/documentation/viikkoraportit/viikko4.md)

### Asennus
1. Seuraa rustin [asennuohjeita](https://www.rust-lang.org/tools/install) rustin kotisivuilla
2. Kloonaa stomper githubista.
3. Aja `cargo run` kloonatussa kansiossa testataksesi käynnistyykö ohjelma. Terminaaliin pitäisi ilmestyä virheilmoitus puuttuvista parametreistä.

### Käyttöohje
Ajamalla `cargo run --help` saat automaattisen uninx-tyylisen käyttöohjeen.

Ohjelmalle annetaan parametreinä pakkaustapa ja pakattava/purettava tiedosto
#### Esimerkki
>pakkaa teksti.txt Lempel-Ziv-Welch algoritmillä tiedostoon pakattu.txt
>
>`cargo run lzw teksti.txt -o pakattu.txt` 
>
>//purkaa pakattu.txt tiedostoon purettu.txt
>
>`cargo run lzw pakattu.txt -d -o purettu.txt` 