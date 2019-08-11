Viikko 3
===

Useamman tunnin säätämisen jälkeen sain pystyyn travis ci ja coveralls.
Nyt testikattavauutta pääsee tarkastelemaan sieltä. Lisäksi dokumentaatiota voi nyt tarkastella docs.rs:ssä.
Ohjelman ydinauleet eivät ole edenneet tällä viikolla
mutta tulevalla viikolla tarkoitkuseni on parantaa lzw algoritmia siten että se käyttäisi 24-bitin koodausta.
Aion myös korjata bugin jossa koodaus ei toimi kun tiedosto on liian iso. Aion myös laajentaa testausta
_frontendin_ puolelle ja testata lzw kattavammin.

Mitä määrittelydokumenttiin tulee niin en täysin ymmärrä miten sitä voisi laajentaa. Aika- ja tilavaativuudet
ovet niitä mitä LZW ja Huffmanille on määritelty. En osaa sanoa niistä sen enempää. Tässä vaiheessa tiedän että
LZW pakkauksella menee ison(10Mb) tiedoston pakkamiseen muutamia sekuntteja ja pakatun tiedoston koko
riipppuu siitä kuinka toistuvaa dataa tiedostossa on. Esimerkiksi täysin satunnaista dataa sisältävä tiedosto
kasvoi kooltaan pakatessa. Tämä korjaantuu aiemmin mainitsemallani 24-bitin koodauksella.
