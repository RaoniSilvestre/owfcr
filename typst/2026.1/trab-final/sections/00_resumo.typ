Este artigo apresenta a implementação e a comparação empírica de duas abordagens heurísticas para o
Problema de Roteamento de Cabos em Fazendas Eólicas (Wind Farm Cabling Problem — WFCP), um desafio
de otimização combinatória que impacta diretamente os custos de infraestrutura em projetos de geração
renovável. São implementadas e avaliadas as heurísticas Sweeping e Sweeping Multi-start, baseadas em
agrupamento angular com árvores geradoras mínimas, e o algoritmo DIJNOR, adaptação do método de caminho
mais curto sucessivo para redes de fluxo com custo em degrau. Os algoritmos são executados sobre o
conjunto de 220 instâncias de Fischetti e Pisinger, com tamanhos de 50, 80 e 120 turbinas, totalizando
mais de seis mil execuções por variante. Os resultados mostram que o Sweeping Multi-start supera
consistentemente as demais abordagens, com GAP médio de 0,0009% em relação à melhor solução conhecida
e tempo de execução de 28 ms, enquanto o DIJNOR apresenta limitações estruturais e maior custo
computacional.
