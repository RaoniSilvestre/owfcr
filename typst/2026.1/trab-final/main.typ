#import "@preview/charged-ieee:0.1.4": ieee

#show: ieee.with(
  title: [Sweeping versus DIJNOR: Uma Comparação Empírica entre Abordagens de Árvore e Fluxo para o Roteamento de Cabos em Parques Eólicos],
  abstract: [#include "sections/00_resumo.typ"],
  authors: (
    (
      name: "Raoni Silva",
      department: [Departamento de Matemática e Informática Aplicada],
      organization: [UFRN],
      location: [Natal, RN],
    ),
  ),
  index-terms: (
    "Wind Farm Cabling Problem",
    "Otimização Combinatória",
    "Heurísticas",
    "Fluxo a Custo Mínimo",
  ),
  bibliography: bibliography("refs.bib"),
  figure-supplement: [Fig.],
)

#include "sections/01_introducao.typ"
#include "sections/02_descricao.typ"
#include "sections/03_modelo.typ"
#include "sections/04_estado_da_arte.typ"
#include "sections/05_revisao_bibliografica.typ"
#include "sections/06_casos_de_teste.typ"
#include "sections/07_implementacao.typ"
#include "sections/08_resultados_empiricos.typ"
#include "sections/09_conclusao.typ"
