# axer.tech | VX Ace Shortcuts Override

[![Static Badge](https://img.shields.io/badge/README-in_English-blue)](https://github.com/Primekick/vx-ace-shortcuts-override/blob/master/README.md)
[![Static Badge](https://img.shields.io/badge/README-po_polsku-red)](https://github.com/Primekick/vx-ace-shortcuts-override/blob/master/README_pl.md)

VX Ace Shortcuts Override, jak nazwa sugeruje, zapewnia sposób na ominięcie dziwacznych wbudowanych skrótów klawiszowych, które przeszkadzają we wprowadzaniu akcentowanych liter w edytorze RPG Maker VX Ace.

## Jak to działa?
Narzędzie to działa poprzez wstrzyknięcie pliku DLL do aktualnie uruchomionej instancji edytora VXA. Z tego powodu antywirusy najprawdopodobniej uniemożliwią jego uruchomienie lub wprost usuną od razu po pobraniu, zatem w celu zapewnienia poprawnego działania, należy dodać je do listy wyjątków.

## Jak tego użyć?
https://github.com/Primekick/vx-ace-shortcuts-override/assets/48254804/bf264bf7-34db-459a-b2c1-8291694e5da7

- rozpakuj plik zip z programem w dowolnym miejscu
- uruchom vxa_shortcuts_override.exe
- lokalizacja edytora powinna zostać wykryta automatycznie — jeżeli jednak nie, musisz podać ją ręcznie
- powinno pojawić się powiadomienie systemowe informujące o pomyślnym wstrzyknięciu

## Ficzery i plany na przyszłość
- [x] wstrzykiwanie DLLki nadpisującej wbudowane skróty klawiszowe
- [x] wykrywanie lokalizacji edytora
- [x] automatyczne włączanie edytora ze wstrzykiwaniem
- [ ] wsparcie dla innych języków poza polskim
- [ ] przejście na mniej inwazyjny sposób tworzenia skrótów (tj. nieglobalny)
- [ ] patchowanie edytora na stałe zamiast uruchamiania programu za każdym razem po włączeniu edytora

## Budowanie
### Toolchain
Wymaga targetu _i686-pc-windows-msvc_, kanał _nightly_.
