Testausdokumentti
===
## Lempel-Ziv-Welch 
LZW-algoritmin testit ovat suoraviivaisia. Osa testeistä testaa pakatun/puretun tiedon oikeellisuuta (esim. onko alkuperäinen ja purettu tiedosto sama) ja osa pakkauksen tehokkuutta. Lisäksi kaksi testiä tarkistaa että sanakirja rakennetaan oikein.
#### Oikeellisuus
Ensimmäinen test suoritetaan tekstitiedostolla joka sisältää merkkijonon _ABBABBBABBA_. Testi aluksi pakkaa tiedoston väliaikaiseen
tiedostoon jonka jälkeen se purkaa sen. Sen jälkeen puretun tiedoston sisältöä verrataan alkuperäiseen sisältöön. Toinen oikeellisuutta
testaava testi toimii muuten samalla tavalla mutta nyt kyseessä on hieman pitempi teksti joka on kaapattu kirjasta Moby Dick.
#### Tehokkuus
Tehokkuutta testataan hyvin löyhällä tavalla. Jos pakattu tiedosto on pienempi kuin alkuperäinen tiedosto niin testi menee läpi.
Tarkemmin algoritmin tehokkuuddesta lukee suorituskyky raportissa.

## Huffman
### Huffman
Huffmanin koodausta testataan samalla tavalla kuin lzw-algoritmia. Samat testit oikeellisuudelle ja tehokkuudelle ovat käytössä
huffmanille kuin lzw.
#### Puun luominen
Testi testaa onko syöttellä _ABAABC_ puu luotu halutulla tavalla. Eli jos pakattava tiedosto sisältäisi kyseisen merkkijonon
niin olisiko merkkijonon perusteella rakennettu puu halutunlainen. Testi käy läpi jokaisen puun solmun ja tarkistaa onko
kyseessä haluttu arvo.
#### Taulun luominen
Testi aluksi luo huffmanin puun syötteelle _ABAABC_ jonka jälkeen puu syötetään taulun luomismetodille. Sen jälkeen testi käy läpi
jokaisen taulun avaimen ja tarkistaa että tiettyä merkkiä vastaa oikea hufffmanin koodaus.
### Node
Node sisältää yhden testin joka tarkistaa että tämän structin ilmentymät järjestetään suurimmasta pienimpään prob kentän perusteella.
### BitWriter
Bittien kirjoittajaa testataan neljällä testillä. Kolme näistä liittyvät kirjoittamiseen ja yksi siihen että bitwriter ei kaadu jos
kirjoitettavien bittien määrä on suuri. Ensimmäinen testi syöttää kirjoittajalle alle tavun kokoisen jonon bittejä.
Sen jälkeen kuten muissakin tämän tyyppisissä testeissä tarkistetaan että toiseen writeriin kirjoitetut bitit todella
kirjoitettiin halutulla tavalla. Toinen testi testaa samaa vähän pitemmällä bittijonolla ja kolmas testa tilannetta jossa
bittijonot tulevat osissa. Viimeisen testin tarkoitus on varmistaa että bittien kirjoitusta jatkettiin siitä mihin jäätiin.
Neljäs testi varmistaa että bitwriter ei räjähdä kun sille syötetään suuri määrä bittijonoja. Testi antaa bitwriterille
3072 bittiä joista joka kolmas on 0 ja loput 1.
### Bitreader
Bitreaderillä on kaksi testiä jotka tarkistavat että luetut bitit ovat samoja mitä annetussa readerissä. Ensimmäinen testi on tavun pituinen ja tarkistaa jokaisen bitin kohdalla että arvo on oikea. Toisessa testissä on kolme tavua bittejä ja jälleen tarkistetaan että jokainen bitti luetaan oikein. Kolmas testi tarkistaa että luettujen bittien määrä vastaa readerin kokoa. Aluksi määritellään 20000
tavun kokoinen vektori ja sitä luetaan kunnes bitreader palauttaa None. Samalla ollaan kirjattu ylös kuinka monta bittiä on luettu
ja lopuksi tarkistetaan että luettujen bittien määrä on 8*20000.
## Main
Fronttia ei ole testattu mutta se toimii niin kuin pitääkin.
## Suorituskyvystä
Riippuen hieman tiedostosta niin molemmat algoritmit saavuttavan 45%-55% prosenttia pienemmän tiedoston alkuperäisestä. En
saanut suorituskykytestaus työkaluja toimimaan joten loin manuaalisesti pienin raportin joka kirjaa kuinka monen prosentin
pakkaus saatiin aikaan. Raportit löytyvät huffmanille [täältä](huffman_stats.txt) ja lzw 
[täältä](lzw_stats.txt). Raportti luodaan aina kun testit ajetaan.
