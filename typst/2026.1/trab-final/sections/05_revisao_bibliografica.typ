= Revisão bibliográfica

O trabalho de Berzan et al. (2011) @berzan2011 destaca-se como uma das referências fundamentais na formalização do Wind Farm Cabling Problem (WFCP).
Ao focar especificamente no desafio do roteamento elétrico interno sob restrições de capacidade, o autor trata o layout como um problema
de otimização combinatória em larga escala. Este artigo é um pontapé inicial essencial, pois propõe algoritmos eficientes para minimizar
os custos de cabeamento e instalação, assumindo que o posicionamento das turbinas já está definido, o que permite isolar e resolver a
complexidade da rede elétrica.

No que diz respeito à modelagem formal exata, Fischetti e Pisinger (2018) @fischetti2018 consolidaram a principal referência na aplicação de Programação Linear Inteira Mista
(MILP) para o design de parques eólicos. Essa formulação forneceu, pela primeira vez, uma base matemática rigorosa para tratar o problema sob uma ótica de otimização global.
Contudo, devido à natureza NP-difícil do problema, a modelagem exata exige um tempo computacional proibitivo para instâncias de grande porte. Por isso, na prática, o modelo MILP
atua majoritariamente como um benchmark padrão — uma linha de base para validar a eficiência e a proximidade da otimalidade de novos algoritmos e abordagens híbridas, abrindo
espaço para o desenvolvimento de heurísticas especializadas.


== Abordagens Heurísticas: Sweeping Multistart e Variable Neighbour Search

Dada a inviabilidade de métodos exatos para grandes instâncias em tempo hábil, a literatura tem investido fortemente em heurísticas construtivas e meta-heurísticas.
Cazzaro et al. (2020) @cazzaro2020 apresentam um avanço significativo nesse sentido ao introduzir a heurística Sweeping Multistart para o WFCP.

O método baseia-se em uma varredura geométrica (angular) do parque eólico a partir da subestação, agrupando as turbinas em setores para construir topologias radiais
iniciais de forma extremamente rápida. Uma vez estabelecidos os grupos, aplica-se o algoritmo de Prim em cada setor, sempre conectando-o à subestação, para gerar uma Árvore
Geradora Mínima (Minimum Spanning Tree - MST) local. A união dessas subárvores compõe uma solução viável para o parque. Como esse processo construtivo possui baixíssimo
custo computacional, a heurística adota uma estratégia de múltiplas partidas (multistart): a varredura é executada iterativamente, variando o tamanho de cada grupo entre
limites pré-definidos (lower-bound e upper-bound). Ao final de todas as iterações, a topologia de menor custo é selecionada como a solução do algoritmo.

Para refinar soluções e alcançar resultados mais competitivos, o mesmo artigo explora a meta-heurística Variable Neighborhood Search (VNS). O VNS fundamenta-se na mudança
sistemática de estruturas de vizinhança durante a fase de busca local, o que permite ao algoritmo escapar de ótimos locais. No contexto de roteamento de cabos, isso traduz-se
em operações de perturbação na topologia, como a quebra e reconexão de galhos, a troca de cabos entre turbinas de ramos diferentes ou a realocação de subárvores.

Além de propor a heurística construtiva, o estudo realiza um benchmarking extensivo, comparando o VNS com outras meta-heurísticas consolidadas (Simulated Annealing,
Tabu Search, Ant Colony e Algoritmos Genéticos). O grande diferencial destacado pelos autores é a consolidação de uma abordagem híbrida: a utilização de uma rodada do
Sweeping Multistart não como a resposta final, mas como um mecanismo altamente eficiente para a geração de uma solução inicial robusta. Ao alimentar o VNS com essa topologia
pré-otimizada, a combinação acelera drasticamente a convergência e melhora a exploração do espaço de busca, estabelecendo-se como uma estratégia altamente eficaz para a
resolução do WFCP.

== Modelagem baseada em Fluxo a Custo Mínimo

Enquanto a maioria das abordagens modela o WFCP estritamente como um problema de árvore ou floresta geradora, investigações alternativas tratam a rede elétrica como um problema
de fluxo de custo mínimo. Em @jenne2020 é explorado essa vertente, investigando como algoritmos clássicos de fluxo a custo mínimo podem ser adaptados às restrições de capacidade
do WFCP. Nesta formulação, cada turbina atua como um nó fonte injetando uma unidade de fluxo na rede, enquanto uma "super-subestação" artificial é introduzida como sumidouro
global, absorvendo a demanda das subestações reais.

Apesar da extensa análise de diversos métodos, a pesquisa destaca que apenas uma modificação do algoritmo Successive Shortest Path (SSP) obteve êxito prático na resolução do
problema. A grande complexidade dessa adaptação reside na função de custo em degrau (step function): enviar fluxo por um cabo só gera custo se exigir uma bitola maior;
do contrário, o custo adicional é nulo. Mais crítico ainda, o escoamento de fluxo cria arestas reversas no grafo residual com custo negativo, representando o ganho financeiro
obtido ao se cancelar ou redirecionar rotas subótimas estabelecidas em iterações
anteriores.

A variação mais eficiente proposta pelo autor para lidar com esse cenário é denominada DIJNOR (Dijkstra using normal costs). Em vez de utilizar potenciais de
nó para tentar mascarar pesos negativos — técnica que gera inconsistências com a função em degrau —, o DIJNOR opera com os custos reais (normais) diretamente na rede
residual. Como a presença de arestas negativas inviabiliza matematicamente o algoritmo de Dijkstra tradicional (podendo prendê-lo em ciclos negativos infinitos), Jenne
introduziu uma trava de relaxamento no motor de busca.

Nesta versão modificada, uma vez que um vértice é visitado e "fechado" (removido da fila de prioridade), ele é
sumariamente ignorado em qualquer tentativa futura de relaxamento de arestas. Essa proibição restrita de revisitas impede a formação de ciclos negativos e viabiliza a execução
do algoritmo em tempo hábil. Como consequência dessa restrição, o algoritmo abdica da garantia de otimalidade. Ao abrir mão da exploração de rotas de realocação
para assegurar uma terminação rápida, o DIJNOR atua, na prática, como uma heurística para o WFCP.
