Määritelmä
==========
Stomper pakkaustyökalu käyttää Lemepel-Ziv-Welch(LZW) pakkausalgoritmia ja Huffman koodausta tiedostojen pakkaamisessa.
LZW edellyttää _sanakirjan_ käyttämistä tiedon paukkaukseen. Käytännössä tämä toteutetaan hajautustauluna.
Huffman koodausta varten täytyy sen sijaan luoda puu-tietorakennetta. 

Ohjelman tarkoitus on siis ottaa mikä vastaan mikä tahansa tiedosto ja palauttaa siitä pienikokoisempi tiedosto.
Ohjelma voi myös purkaa pakatun tiedoston alkuperäsen muotoon. 
Tiedontiivistys ja salaus ovat minulle kiinnostavia aiheita, joista haluaisin oppia lisää.
Tämä kurssi antaa minulle hyvän tilaisuuden perehtyä niihin.
Tietorakenteet ja algoritmit -kurssilla ei myöskään käyty aihetta kovinkaan paljon läpi ja se lisäsi kiinnostusta.

Ohjelma saa syötteenä polun pakattavaan(tai purettavaan) tiedostoon ja pakkausmetodin(LZW tai Huffman).
Vaihtoehtoisesti käyttäjä voi määrittää output-tiedoston jolloin alkuperäistä tiedosta ei muokata vaan luodaan uusi tiedosto.
Tiedoston purkamiseen käytetään -d vipua.

LZW pakkaa ja purkaa lineaariajassa. Huffman sen sijaan vie O(nlogn) aikaa. 
Tavoitteena on luoda tehokkaat toteutukse molemmista algoritmeista.
