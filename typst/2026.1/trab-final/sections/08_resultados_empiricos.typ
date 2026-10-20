= Resultados Empíricos

== Configuração experimental

Os experimentos foram executados em uma máquina com processador Intel Core i3-1115G4 @ 3.00 GHz,
20 GB de RAM e sistema operacional Fedora 44, utilizando o compilador Rust 1.96. Cada algoritmo foi executado 30 vezes por instância, totalizando 6.600
execuções por variante.

Foram avaliadas seis configurações: Sweeping, Sweeping Multi-start, DIJNOR (sem ordenação), DIJNOR Nearest,
DIJNOR Sweep 2% e DIJNOR Sweep 20%.

== Comparação global

A Tabela I apresenta o resumo global dos resultados. Para cada configuração, são exibidos o melhor custo
encontrado, o custo médio, o tempo médio de execução e o número de execuções.

#figure(
  table(
    columns: (auto, auto, auto, auto, auto),
    stroke: 0.5pt,
    align: center,
    [*Algoritmo*], [*Melhor (K)*], [*Médio (K)*], [*Tempo*], [*Execuções*],
    [SweepingMultiStart], [29.555], [73.419], [28 ms], [6.600],
    [Sweeping], [31.032], [78.900], [0,02 ms], [6.600],
    [DIJNOR], [34.678], [84.847], [155 ms], [6.600],
    [DIJNOR Nearest], [30.963], [87.137], [162 ms], [6.600],
    [DIJNOR Sweep 20%], [31.123], [88.584], [160 ms], [6.600],
    [DIJNOR Sweep 2%], [31.433], [91.948], [164 ms], [6.600],
  ),
  kind: table,
  caption: [Resumo global dos resultados. "K" denota milhares de reais (R\$).],
)

O Sweeping Multi-start obteve o menor custo médio (R\$ 73,4 milhões), seguido pelo Sweeping (R\$ 78,9
milhões). As variantes do DIJNOR apresentaram custos médios superiores, entre R\$ 84,8 e R\$ 91,9 milhões.
Em termos de tempo de execução, o Sweeping é o mais rápido (0,02 ms), enquanto o Sweeping Multi-start
demanda 28 ms em média. As variantes do DIJNOR têm tempos próximos entre si (155–164 ms), refletindo seu
custo computacional mais elevado devido ao teste de cruzamento durante o Dijkstra.

O DIJNOR Farthest não foi incluido pois não garantia corretude em algumas instâncias, além de ter um resultado
consideravelmente pior que as outras implementações do DIJNOR.

== Análise de competitividade

Para cada execução, calculou-se o GAP relativo à melhor solução conhecida (Best Known Solution — BKS) para
aquela instância, definido por $("custo" - "BKS") / "BKS" times 100%$. A Tabela II apresenta o GAP médio
e o número de vitórias (execuções em que o algoritmo igualou a BKS) para cada configuração.

#figure(
  table(
    columns: (auto, auto, auto),
    stroke: 0.5pt,
    align: center,
    [*Algoritmo*], [*GAP Médio*], [*Vitórias*],
    [SweepingMultiStart], [0,0009%], [6.571],
    [Sweeping], [7,47%], [0],
    [DIJNOR], [14,77%], [0],
    [DIJNOR Sweep 20%], [19,31%], [0],
    [DIJNOR Nearest], [21,26%], [0],
    [DIJNOR Sweep 2%], [23,37%], [2],
  ),
  kind: table,
  caption: [Análise de GAP relativo à melhor solução conhecida.],
)

O Sweeping Multi-start domina a competição: GAP médio de apenas 0,0009% e 6.571 vitórias em 6.601
execuções — ou seja, encontrou a melhor solução conhecida em 99,5% dos casos. As demais configurações
apresentaram GAPs significativamente maiores, variando de 7,47% (Sweeping) a 23,37% (DIJNOR Sweep 2%),
e praticamente não obtiveram vitórias.

== Perfil de desempenho

A Figura 3 apresenta o perfil de desempenho (performance profile) das configurações avaliadas. O gráfico
mostra, para cada algoritmo, a proporção de instâncias resolvidas dentro de um fator $tau$ da melhor
solução conhecida.

#figure(
  image("../images/performance_profile.png", width: 100%),
  caption: [Perfil de desempenho: proporção de instâncias resolvidas dentro de um fator $tau$ da BKS.],
)

Observa-se que o Sweeping Multi-start domina as demais configurações para todos os fatores $tau$: para
$tau = 1$, ele já resolve cerca de 70% das instâncias no GAP zero, e converge rapidamente para 100%
conforme $tau$ aumenta. As outras configurações apresentam curvas significativamente abaixo,
confirmando a superioridade do Sweeping Multi-start.

== Análise de escalabilidade

As Figuras 4 e 5 apresentam, respectivamente, a evolução do custo médio e do tempo médio de execução em
função do tamanho da instância (50, 80 e 120 turbinas).

#figure(
  image("../images/scalability_cost.png", width: 100%),
  caption: [Custo médio por quantidade de turbinas.],
)

#figure(
  image("../images/scalability_time.png", width: 100%),
  caption: [Tempo médio de execução por quantidade de turbinas.],
)

O custo médio cresce de forma aproximadamente linear com o número de turbinas para todas as
configurações. O Sweeping Multi-start mantém-se consistentemente abaixo das demais em todos os
portes. Em termos de tempo de execução, o Sweeping é o mais eficiente (abaixo de 0,1 ms mesmo para
120 turbinas), seguido pelo Sweeping Multi-start (55 ms para 120 turbinas). As variantes do DIJNOR
apresentam crescimento mais acentuado, atingindo cerca de 600 ms para instâncias de 120 turbinas.
