= Conclusão

Este artigo apresentou a implementação e a comparação empírica de duas abordagens heurísticas para o
Wind Farm Cabling Problem (WFCP): as heurísticas construtivas Sweeping e Sweeping Multi-start, ambas
baseadas em agrupamento angular seguido de árvores geradoras mínimas, e o DIJNOR, uma adaptação do
método de caminho mais curto sucessivo para redes de fluxo com função de custo em degrau. Os algoritmos
foram executados sobre o conjunto completo de 220 instâncias de Fischetti e Pisinger, com tamanhos de
50, 80 e 120 turbinas, totalizando mais de seis mil execuções por variante.

Os resultados experimentais são conclusivos. O Sweeping Multi-start obteve o melhor desempenho geral:
GAP médio de apenas 0,0009% em relação à melhor solução conhecida, tendo encontrado essa solução em
99,5% das execuções. O tempo médio de execução foi de 28 ms, com a versão base do Sweeping atingindo
0,02 ms. Além disso, ambas as variantes produzem topologias que são sempre árvores válidas —
conexas, acíclicas e sem cruzamentos entre cabos.

O DIJNOR, em todas as suas variantes de ordenação, apresentou resultados inferiores sob múltiplos
aspectos. A estratégia de processar cada turbina buscando o caminho incremental mais barato — a chamada
"carona" em cabos já instalados — leva a uma concentração excessiva de fluxo em poucos nós centrais.
Como o algoritmo não pode desfazer conexões já estabelecidas ao contrário do SSP original, que
utilizaria ciclos negativos para realocar fluxo, essa concentração se torna irreversível. A topologia
resultante frequentemente não forma uma árvore: longas cadeias de turbinas conectadas em série
forçam bifurcações sucessivas do fluxo para respeitar os limites de capacidade, resultando em
topologias mais complexas e onerosas. A esses problemas estruturais soma-se o custo computacional:
o teste de cruzamento geométrico, executado a cada iteração do Dijkstra para cada aresta candidata,
torna-se o gargalo dominante do algoritmo — conforme confirmado por ferramentas de profiling —,
elevando o tempo médio de execução para mais de 150 ms, mesmo nas instâncias de menor porte.

A mensagem central que emerge deste estudo é que a modelagem do WFCP como um problema de árvore
geradora resolvida pela heurística Sweeping revelou-se não apenas mais simples de implementar, mas
superior em todos os critérios avaliados: qualidade das soluções, tempo de execução e garantias
estruturais. A abordagem de fluxo a custo mínimo adaptada pelo DIJNOR, mostrou-se menos adequada
para o problema, tanto do ponto de vista da eficiência computacional quanto da eficácia das
topologias geradas.

