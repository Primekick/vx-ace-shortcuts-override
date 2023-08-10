# axer.tech | VX Ace Shortcuts Override

[![Static Badge](https://img.shields.io/badge/README-in_English-blue)](https://github.com/Primekick/vx-ace-shortcuts-override/blob/master/README.md)
[![Static Badge](https://img.shields.io/badge/README-po_polsku-red)](https://github.com/Primekick/vx-ace-shortcuts-override/blob/master/README_pl.md)

VX Ace Shortcuts Override, jak nazwa sugeruje, zapewnia sposób na ominięcie dziwacznych wbudowanych skrótów klawiszowych, które przeszkadzają we wprowadzaniu akcentowanych liter w edytorze RPG Maker VX Ace.

## Jak to działa?
Narzędzie to działa poprzez wstrzyknięcie pliku DLL do aktualnie uruchomionej instancji edytora VXA. Z tego powodu antywirusy najprawdopodobniej uniemożliwią jego uruchomienie lub wprost usuną od razu po pobraniu, zatem w celu zapewnienia poprawnego działania, należy dodać je do listy wyjątków.

## Jak tego użyć?
https://github.com/Primekick/vx-ace-shortcuts-override/assets/48254804/c2473eb7-4ab9-4942-9e30-2aab982fff3b

- umieść zarówno at_vxa_so.dll, jak i plik exe w tym samym katalogu, w którym znajduje się edytor VX Ace
- najpierw włącz edytor
- następnie uruchom vxa_shortcuts_override.exe
- jeśli wszystko poszło dobrze, powinno pojawić się okienko wskazujące, że DLLka została pomyślnie wstrzyknięta

## Ficzery i plany na przyszłość
- [x] wstrzykiwanie DLLki nadpisującej wbudowane skróty klawiszowe
- [ ] wsparcie dla innych języków poza polskim
- [ ] przejście na mniej inwazyjny sposób tworzenia skrótów (tj. nieglobalny)
- [ ] patchowanie edytora na stałe zamiast uruchamiania programu za każdym razem po włączeniu edytora

## Budowanie
### Toolchain
Wymaga targetu _i686-pc-windows-msvc_, kanał _nightly_.
