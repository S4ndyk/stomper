Toteutusdokumentti
===
Ohjelma on jaettu kahteen erilliseen `crate`-pakkaukseen stomper ja libstomper, joista stomper toimittaa käyttöliittymälogiikan
ja libstomper toteutetut algoritmit. 
## stomper
### [main.rs](../stomper/src/main.rs)
Main tiedosto sisältää ohjelman main metodin. Main metodi on vastuussa ohjelman käynnistämisestä 
ja virheraporttien tulostamisesta ja formatoinnista. Main metodi siis kerää parametrit ja antaa ne eteenpäin käynnistäjä metodille.
### [ lib.rs ](../stomper/src/lib.rs)
Lib tiedosto sisältää komentoriviparametrien määrittelyn ja parsimisen (josta suurin osa on 
[ulkopuolisen kirjaston](https://crates.io/crates/structopt) tekemää) sekä
kutsuu niiden perusteella kirjastopuolen metodeja. run metodi määrittelee ensin tiedostonimen johon käsiteltävä tiedosto pakataan/puretaan. Sen jälkeen oikean metodin valinnan käsittelee choose_compression metodi. 
## libstomper
#### [lib.rs](../libstomper/src/lib.rs)
lib.rs sisältää Compressor traitin määrittelyn. libstomperille pakkaajan pitää toteuttaa kaksi metodia; purkaminen ja pakkaminne.
Molemmat metodit ottavat parametreinään sisääntulon ja ulostulon eli mikä tiedosto pakataan/puretaan ja mihin se pakataan/puretaan.
Käytännön syistä parametrien pitää toteuttaa myös Seek traitti.
### Huffman koodaus
#### [mod.rs](../libstomper/src/huffman/mod.rs)
mod.rs on huffmanin koodauksen päätiedosto jossa huffmanin koodauksen toteutus ja testaus sijaitsee. Se sisältää metodit
huffmanin puun luonnille, pakkamisessa auttavan kooditaulun luonnille, pakkaamisen ja purkamisen sekä metodit jotka kirjoittavat ja
tulkitsevat huffmanin puun tidostosta. Huffmanin puun luominen vie aikaa n log(n). Aluksi annettu tiedosto analysoidaan merkitsemällä
jokaisen merkin esiintymiskerrat hajautustauluun. Tämän jälkeen hajautustaulun parit lisätään maksimikekoon joka vie aikaa log(n).
Keon perusteella muodostetaan puu jonka juurisolmun metodi palauttaa. 

Puu kirjoitetaan tiedostoon niin että se käydään läpi syvyyshaulla. Ensin kirjoitetaan solmun sisältä merkki ja sen esiintyvyyskerrat.
Sitten jos solmulla on vasen lapsi kirjoitetaan tiedostoon numero yksi jonka jälkeen lapsen tiedot kirjataan. Jos lasta ei ole
kirjoitetaan nolla. Puu luetaan tiedostosta vastaavalla tavalla syvyyshaun avulla. Tähän väliin merkataan montako merkkiä
alkuperäisessä tiedostossa niin että purkaja tietää montako merkkiä täytyy lukea.

Koodauksen kirjoittamista varten huffmanin puu käydään läpi syvyyshaulla ja merkataan hajautustauluun minkä koodauksen mikäkin merkki 
saa. Tämä ei ole kovin hyvä tapa mutta en näissä puitteissa keksinyt muuta. Sen jälkeen alkuperäinen tiedosto käydään läpi uudestaan ja 
kirjoitetaan ulostulotiedostoon vastaavat huffmankoodit.

Purkaessa aluksi luetaan huffmanin puu aiemmin kuvatulla tavalla jonka jälkeen luetaan bittejä bitreaderilla niin kauan kuin merkkejä 
riittää. 
#### [bitwriter.rs](../libstomper/src/huffman/bitwriter.rs)
Bitwriter on tarkoitettu yksittäisten bittien kirjoittamiseen annetuun writeriin. Bitwriterin toiminta on yksinkertainen. Se kirjoittaa
bufferiin yksittäisiä bittejä muokkaamalla tavuja bittioperaatioden avulla. Kun bufferi on kirjoitettu täyteen sen kirjoittaa bufferin
annettuun writeriin ja nollaa bufferin. Bitwriterilla on myös metodi jolla voidaan kirjoittaa merkkijonoja biteiksi. Eli jos annetussa merkkijonossa on '1' niin bitwriter kirjoittaa ykkösen. Tämä helpotta huffman koodien kirjoittamisessa.
#### [bitreader.rs](../libstomper/src/huffman/bitreader.rs)
Bitreader on kuin bitwriter muttta lukija. Sillä luetaan bittejä yksi kerrallaan. Bitwriter ottaa annetusta readeristä tavuja bufferriin
ja next_bit metodia kutsuttaessa antaa seuraavan bitin tiedostossa. Palauttaa true jos bitti on 1 ja false jos bitti on 0.
Bitin arvo saadaan bittioperaatioilla.
#### [node.rs](../libstomper/src/huffman/node.rs)
Node kuvaa solmua huffmanin puussa. Koska tämä structi on tarkoitettu yksinomaan huffmanin koodaukselle, jossa käytetään hyväksi
maksimikekoa, node structit järjestetään suurimmasta pienimpään.
## Lempel-Ziv-Welch
### [lzw.rs](../libstomper/src/lzw.rs)
Tänne on toteutettu Lempel-Ziv-Welchin algoritmi. LZW käy annetun tiedoston läpi ja kirjoittaa sen ulostuloon O(n) ajassa.
Tämä toteututs käyttää 24-bitin koodausta. Toisin sanottuna jokaista tiivistettyä merkkijonoa vastaa 24-bitin koodi. Huomasin että 
32-bitin koodaus ei pakkaa tarpeeksi tehokkaasti ja 16-bitin koodiissa on liian vähän arvoja. Koodeja voi siis olla maksimissaa
16777216 kappaletta. Kun koodien määrä ylittyy niin alustetaan uusi hajautustaulu johon tulee uudet koodit. Tämä LZW toteutus
nojaa [byteorder](https://docs.rs/byteorder/1.3.2/byteorder/) kirjastoon jonka avulla eri mittaisia tietotyyppejä voidaan kirjoittaa
tiedostoon.
## Muita mietteitä
Projekti osottautui odotettua haastavammaksi. Tämä ei niinkään johtunut aiheesta vaan omasta kokemuksen puutteestani rust-kielen kanssa.
Tähän projektiin ei esimerkiksi ole tehty omia tietorakenteita sillä yksinkertaistenkin tietorakentein toteuttaminen (esim. lista)
on varsin haastavaa rustilla. Rustin käyttö on muutenkin hidastanut projektin etenemistä monissa kohdissa minkä takia se
saattaa vaikuttaa puutteelliselta. En yllättyisi jos saisin tältä kurssilta hylsyn. En myöskään saanut criterion 
suorituskykytestaus-kirjastoa toimimaan projektissani. Kaikenkaikkiaan tämän mittakaavan projektin tekeminen rustilla
on minun taitojeni ulottumattomissa. Perustoiminnot ovat kuitenkin paikallaan ja ohjelma on täysin käytettävä. Projektia
voisi optimoida esimerkiksi huffmanin koodauksen kohdalla esim. keksimällä paremman tavan pakata huffmanin puun tiedoston alkuun.