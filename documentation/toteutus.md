Toteutusdokumentti
===
Ohjelma on jaettu kahteen erilliseen `crate`-pakkaukseen stomper ja libstomper, joista stomper toimittaa käyttöliittymälogiikan
ja libstomper toteutetut algoritmit. 
## stomper
### main.rs
Main tiedosto sisältää ohjelman main metodin. Main metodi on vastuussa ohjelman käynnistämisestä 
ja virheraporttien tulostamisesta ja formatoinnista. Main metodi siis kerää parametrit ja antaa ne eteenpäin käynnistäjä metodille.
### lib.rs
Lib tiedosto sisältää komentoriviparametrien määrittelyn ja parsimisen (josta suurin osa on ulkopuolisen kirjaston tekemää) sekä
kutsuu niiden perusteella kirjastopuolen metodeja.

## libstomper