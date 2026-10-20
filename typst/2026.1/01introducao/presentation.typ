#import "@preview/touying:0.7.4": *
#import themes.metropolis: *

#show: metropolis-theme.with(
  aspect-ratio: "16-9",
  config-info(
    title: "Roteamento de Cabos em Parques Eólicos",
    subtitle: "Uma Revisão do Estado da Arte e Abordagens Heurísticas",
    author: "Raoni Silva",
    institution: "UFRN - Departamento de Matemática e Informática Aplicada",
  ),
)

#set text(font: "Open Sans", size: 20pt)
#show raw: set text(font: "Fira Code", size: 0.8em)
#set par(justify: true)

#title-slide()

= O Desafio da Infraestrutura Eólica

== O Desafio da Infraestrutura Eólica

- A capacidade eólica mundial se expande rapidamente (onshore e offshore).
- O RN é o segundo maior produtor de energia eólica do Brasil (ABEEólica, 2024).
- *O problema financeiro:* A infraestrutura elétrica representa de 15% a 30% dos custos iniciais totais de uma fazenda eólica offshore.
- *Solução:* Otimizar o roteamento (WFCP - _Wind Farm Cabling Problem_) para minimizar custos e perdas elétricas.

== Descrição do Problema (WFCP)

*Objetivo:* Interligar turbinas a uma subestação minimizando custos.

- *Capacidade dos cabos:* Produzidos com especificações e capacidades fixas. Custos não-lineares.
- *Geração:* Cada turbina produz 1 unidade de energia. Um cabo de capacidade $N$ suporta no máximo $N$ turbinas.
- *Restrição da Subestação:* Limite superior de portas disponíveis ($K$) para conexões diretas.
- *Planaridade:* Proibição de cruzamento de cabos no terreno.

== Variações do Problema Base

Para aproximar o modelo matemático do mundo real, a literatura explora:

+ *Múltiplas Subestações:* Balanceamento de carga da rede.
+ *Perdas de Energia (_Wake Effect_/_Thermal_):* Justifica a escolha de cabos mais robustos e caros para evitar dissipação de calor a longo prazo.
+ *Desvio de Obstáculos:* Uso de _Steiner points_ (nós de ramificação artificiais) para contornar áreas restritas.

= Modelagem em Grafos

== Modelagem em Grafos

O problema é definido sobre um grafo direcionado

$ G = (V, A) $

+ *Anti-arborescência e MILP (Fischetti & Pisinger, 2018):*
  - Estrutura de árvore direcionada para a raiz.
  - Utiliza Programação Linear Inteira Mista (MILP).
  - Restrição de fluxo indivisível (_unsplittable flow_).
  - Atua como _baseline_ exato (encontra o ótimo global, mas inviável para instâncias grandes).

#align(center, image("images/diagrama_antiarborescencia.svg", height: 80%))

== Modelagem em Grafos

O problema é definido sobre um grafo direcionado

$ G = (V, A) $

+ *Fluxo a Custo Mínimo (Jenne, 2020):*
  - Rede de fluxo onde as turbinas são fontes injetando 1 unidade.
  - Subestações reais se conectam a um _sink_ artificial global.
  - Step function não-linear.

#align(center, image("images/diagrama_fluxo.svg", height: 80%))

= Estado da Arte: Heurística Construtiva e VNS

== Estado da Arte: Heurística Construtiva e VNS

*Sweeping Multistart + Variable Neighborhood Search (Cazzaro et al., 2020)*

- *_Sweeping Multistart:_* Varredura geométrica (angular) a partir da subestação. Agrupa turbinas e aplica o algoritmo de Prim para gerar soluções radiais iniciais de forma extremamente rápida.
- *VNS:* Modifica sistematicamente vizinhanças (quebra de galhos, troca de cabos) para escapar de ótimos locais.

*Resultado:* A estratégia conjunta (hibridização) superou consistentemente outros algoritmos testados, aproximando-se da solução ótima em tempo recorde.

#align(center, image("images/sweeping.png", height: 80%))

== Estado da Arte: Heurística Construtiva e VNS

*Sweeping Multistart + Variable Neighborhood Search (Cazzaro et al., 2020)*

- *_Sweeping Multistart:_* Varredura geométrica (angular) a partir da subestação. Agrupa turbinas e aplica o algoritmo de Prim para gerar soluções radiais iniciais de forma extremamente rápida.
- *VNS:* Modifica sistematicamente vizinhanças (quebra de galhos, troca de cabos) para escapar de ótimos locais.

*Resultado:* A estratégia conjunta (hibridização) superou consistentemente outros algoritmos testados, aproximando-se da solução ótima em tempo recorde.

#align(center, image("images/comparison.png", height: 80%))

== Abordagem por Fluxo: O Algoritmo DIJNOR

*Desafio do Fluxo a Custo Mínimo no WFCP:*
A função de custo é em degrau (só cobra quando há mudança de bitola) e cria arestas de custo negativo no grafo residual (ganho ao cancelar rotas).

*A Solução DIJNOR (Jenne, 2020):*
- _Dijkstra using normal costs_.
- Como Dijkstra tradicional falha com arestas negativas (ciclos infinitos), o DIJNOR introduz uma trava: nós fechados não podem ser revisitados/relaxados.
- Perde a garantia de otimalidade global, mas torna-se uma heurística incrivelmente veloz e competitiva para o WFCP.

== Casos de Teste

Como validar esses algoritmos?

- Conjunto de *220 instâncias* (Fischetti & Pisinger, 2018).
- Fundamentado em dados operacionais e restrições físicas de parques eólicos reais da Europa.
- Variação de dezenas a centenas de turbinas, com múltiplos catálogos de cabos.
- É o padrão ouro para testar escalabilidade e robustez.

== Conclusão e Próximos Passos

O WFCP exige abordagens que contornem o gargalo temporal do modelo MILP exato. Temos duas modelagens promissoras:

+ A representação estrutural resolvida via *Sweeping Multistart + VNS*.
+ A abstração de rede de fluxo tratada via *DIJNOR*.

*Próximo passo da pesquisa:*
Implementação computacional e análise comparativa direta entre os algoritmos *Sweeping* e *DIJNOR*, utilizando as 220 instâncias reais para confrontar desempenho e qualidade dos resultados.

#focus-slide[Obrigado pela atenção!]
