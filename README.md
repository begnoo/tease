# tease
Tease je aplikacija za kontrolu i praćenje verzija izvornog koda
# Funkcionalnosti
Glavna funkcionalnost ove aplikacije je čuvanje različitih verzija i razlika izmedju samih izvornih kodova.

### Neregistrovani korisnik
Neregistrovani korisnik ima mogućnost pregleda sadržaja repozitorijuma drugih korisnika.

### Registrovani korisnik
Registrovani korisnik ima mogućnost da stvori repozitorijum u kojem može da čuva, ažurira i naknadno da preuzima svoj kod.
Funkcionalnosti koje će registrovanom korisniku biti dostupne su:
* _create_ -
kreiranje repozitorijuma sa jedinstvenim imenom i opisom
* _clone_ -
preuzimanje repozitorijuma
* _branch_ -
kreiranje grane unutar repozitorijuma, koja ima zejedničku početnu tačku sa podrazumevanom glavnom granom ili sa specifičnom granom repozitorijuma
* _commit_ - 
Lokalno čuvanje izmena nad izvornom kodu u repozitorijumu
* _push_ -
ažuriranje repozitorijuma kroz slanje novih sačuvanih lokalnih izmena
* _pull_ -
povlačenje ažuriranog stanja repozitorijuma, ili povlačenje drugih grana repozitorijuma
* _merge_ -
spajanje različitih grana
* _show-diff_ -
prikaz razlika između određenih ažuriranja ili određenih grana

# Arhitektura sistema
* CLI aplikacija za korisničko rukovanje sa repozitorijumom - Rust
* Korisnički mirkoservis - Go
* Mikroservis za rukovanje repozitorijumima - Go
* Email mikroservis - Go ili Python
* Veb interfejs - TypeScript, React

Kao potencijalno proširenje moguće je implementirati mikroservis koji bi nadgledao korisničku aktivnost i frekvenciju ažuriranja repozitorijuma. Takođe je moguće proširenje u vidu repozitorijuma specifično predviđenih za slike, video snimke ili binarne podatke.
