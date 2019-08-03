Viikko 2
===
### Maanantai-Tiistai 
Alkuviikko kului perehtyessäni tarkemmin rustin rajapinta systeemiin. Viikonloppuna tätä kirjoittaessani
voin sanoa että ohjelman yleisrakenne on päätetty ja se on pysyvä. Kokeilin erilaisia vaihtoehtoja
mutta rustin pakkomielle muistin hallinan kanssa tuottivat hankaluuksia. Erityisesti se että
kaikkien muuttujien koko on tiedettävä käännösaikana on välillä vaikeaa kun työskentelee
geneerisen tyypityksen kanssa.

### Torstai-Lauantai
Torstaina aloitin itse ohjelman toimminan toteuttamisen. Vastaan tuli monia mutkia. Ensimmäisenä 
loin Lempel-Ziv-Welchin pakkaus algoritmin. Alun suuri ongelma oli se kun 8 bitin piti koodata
32 bitin dataksi. Ongelmana on siis että koodataanko data _Little endian_ vai _Big endian_ muodossa.
Itse päätin että ohjelma lukee ja kirjottaa sisäisesti little endian dataa. Datan kirjoittamiseen
jouduin käyttämään [byteorder kirjastoa](https://docs.rs/byteorder/1.3.2/byteorder). Vastaan tuli
myös monta pienempää rustiin liittyvää haastetta mutta selvisin niistä ja olen paljon parempi
rust-ohjelmoija kuin viikko sitten. Nyt launtaihin mennessä olen saanut ensimmäisen toimivan version
ohjelmastani. Kirjoitushetkellä ohjelma pakkaa ja purkaa onnistuneesti minkä tahansa tiedoston olettaen
että tiedosto on tarpeeksi pieni. En ole vielä käsitellyt ongelmaa jossa _sanakirjan_ koodit loppuvat kesken.
Lisäksi 32-bitin koodaus saattaa olla liian iso eikä alle 10mb tiedostoissa tiivistä riittävästi.
Satunnaista binääridataa sisältävän tiedoston se sai jopa alkuperäistä suurempaan muotoon pakkauksen jälkeen.
Seuraavalla viikolla aion kirjoittaa testit lzw algoritmille ja korjata edellämainitut bugit. Ohjelman 
toteutuksessa suurena apuna oli Mark Nelsonin kirjoittama pitkä LZW liittyvä [artikkeli](https://marknelson.us/posts/2011/11/08/lzw-revisited.html). Oma algoritmini tapailee hyvin läheisesti Nelsonin algoritmia. Toivottavasti tämä ei ole ongelma.
