# 1
## Simulation
- allow open ended plushies
- flooring modes
- error-resistancy
- compile flag for strict but expensive assertions (sanity macro)
- growing one by one stitch

## Patterns
- starting from chains
- marks and gotos
- round repeats

## Evolution
- plushie fitness
- plushie visual comparison
- evolution experiments
- idiot-proof parsing (obligatory for genetic algorithms)

##
- infrastruktura pod eksperymenty genetyczne
  - konwersja: STL -> jednorodna chmura punktowa (.json)
  - generowanie osobnikow, operacje genetyczne
  - ocena populacji przez HTTP request do symulatora
  - funkcja fitnesu
    - dla kazdego punktu w wygenerowanym pluszaku znajdz najblizszy punkt w chmurze punktowej, suma odleglosci to fitness
    - używam [R-tree](https://en.wikipedia.org/wiki/R-tree) dla szukania w czasie O(log n)

- cargo run ws --preset pillar
- npm run dev




















- same eksperymenty jeszcze nie wychodza
  - zatrzymywanie symulacji w odpowiednim momencie
    - elbow method?
  - obrót przy użyciu BLO/FLO
    - refactor szkieletu (generalized cyllinder)
      - [Skeleton Extraction from 3D Point Clouds by Decomposing the Object into Parts](https://arxiv.org/pdf/1912.11932.pdf)
  - wydajność
    - multithreading?
    - gpu?
    - cloud?
    - modyfikacja podczas symulacji?
  - naiwna mutacja


- terminy
  - deklaracji w usos
  - kiedy sa składane i bronione prace
