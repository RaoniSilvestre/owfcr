= Implementação

A implementação dos algoritmos foi realizada em Rust, utilizando a biblioteca `petgraph` para a representação
e manipulação de grafos. O grafo do parque eólico é modelado como uma matriz de adjacência completa,
na qual a posição `m[i][j]` armazena a distância euclidiana entre os nós `i` e `j`. Essas distâncias são pré-computadas
no momento do carregamento da instância, permitindo acesso em tempo O(1) a qualquer aresta durante a execução dos algoritmos.

Para uma comparação justa, cada algoritmo foi modelado como um tipo que implementa a interface `Algorithm`. Um único método relevante
é necessário para implementar essa interface:

$ f_"algo"(P) = S, quad P in "Problem", S in "Solution" $

Onde um problema é composto pelo grafo completo, os nós de turbinas, o nó da subestação e os cabos disponíveis.

Uma solução é representada pelo custo total da solução e a lista das arestas selecionadas. Para cada aresta selecionada, também é
armazenado qual cabo foi selecionado e o custo financeiro do segmento.

== Heurística Sweeping

A heurística Sweeping opera em três etapas: ordenação angular, agrupamento e construção da árvore.

=== Ordenação Angular

Na primeira etapa, todas as turbinas são ordenadas de acordo com seu ângulo polar em relação à subestação, calculado
por meio da função `atan2`. O resultado é uma lista circular das turbinas ao redor do ponto de coleta.

=== Agrupamento

Na etapa seguinte, as turbinas são divididas em grupos de tamanho $k = ceil(N \/ "max_connections")$, onde
$N$ é o número total de turbinas e `max_connections` é o limite de portas de conexão da subestação. Os grupos são
formados tomando-se $k$ turbinas consecutivas na ordem angular, de modo que o último grupo pode conter menos turbinas
caso $N$ não seja múltiplo de $k$.

Para cada grupo, executa-se o algoritmo de Prim inicializado na subestação, gerando uma árvore geradora mínima (MST)
que conecta todas as turbinas do grupo ao ponto de coleta. Durante essa etapa, o custo para adicionar uma nova aresta também
inclui o custo de cruzamento. Na prática, isso age como um descarte de qualquer solução que tenha interseções.

=== Construção da solução

Após todos os grupos serem processados e suas árvores unificadas, uma busca em profundidade (DFS) pós-ordem percorre
a topologia da subestação em direção às folhas. Cada turbina contribui com uma unidade de fluxo, que se acumula nos
caminhos ascendentes. Em cada aresta, o fluxo acumulado determina a bitola de cabo necessária, e o custo do segmento
é calculado como o produto do preço do cabo por metro pela distância euclidiana entre os nós.

== Sweeping Multi-start

A versão Multi-start estende a heurística Sweeping por meio de
uma exploração sistemática de diferentes configurações de agrupamento, variando três parâmetros:

- a posição inicial do sweep — a turbina que inicia o agrupamento angular —, iterando sobre todas as $N$ turbinas
  como possíveis pontos de partida;
- a direção da varredura, tanto no sentido horário quanto no sentido anti-horário;
- o tamanho dos grupos, partindo de $ceil(N \/ "max_connections")$ (mínimo necessário para respeitar o limite
  de conexões da subestação) até a capacidade do maior cabo disponível no catálogo.

Para cada combinação desses três parâmetros, o algoritmo Sweeping é executado, e a solução de menor custo total é
retida. O espaço de busca explorado é da ordem de $N times 2 times ("max_tpg" - "min_tpg" + 1)$, em que "tpg"
(turbines per group) é a quantidade de turbinas por grupo. O baixo custo computacional de cada execução individual
do Sweeping — que envolve apenas a construção de MSTs sobre subconjuntos dos nós — mantém o tempo total de execução
extremamente reduzido.

== DIJNOR

O DIJNOR, proposto por Jenne (2020) @jenne2020, processa as turbinas sequencialmente, construindo a rede de cabos
de forma incremental. A cada iteração, um Dijkstra modificado encontra o caminho de menor custo da
turbina corrente até a subestação, considerando os cabos já instalados. O estado da rede é mantido em uma tabela
_hash_ que mapeia cada aresta direcionada $(u, v)$ ao fluxo líquido que a atravessa — o chamado `active_flows`.

No modelo original de Jenne, o SSP requer um único sumidouro, o que motiva a introdução de uma "super-subestação"
artificial conectada a todas as subestações reais por arestas de custo zero. Como as instâncias utilizadas neste
trabalho possuem apenas uma subestação, essa abstração não foi necessária: a própria subestação atua diretamente
como sumidouro da rede de fluxo.

A ordem em que as turbinas são processadas afeta significativamente a solução final @jenne2020. Nesta implementação, foram
testadas quatro estratégias de ordenação:

- *Nearest*: turbinas ordenadas pela distância euclidiana até a subestação, das mais próximas para as mais distantes;
- *Farthest*: ordem inversa, das mais distantes para as mais próximas. Esta estratégia revelou-se problemática,
  gerando soluções que violam restrições do problema em alguns casos;
- *Sweep*: as turbinas são organizadas em anéis concêntricos de distância ao redor da subestação. A espessura de
  cada anel é um percentual (`band`) da amplitude total de distâncias do parque. Dentro de cada anel, as turbinas
  são ordenadas por ângulo polar, varrendo do centro para a periferia;
- *Normal*: as turbinas são processadas na ordem em que aparecem, sem nenhuma ordenação em especial.

O Dijkstra modificado opera sobre o grafo completo, mantendo um conjunto de nós visitados que não podem ser
reexaminados. Para cada aresta candidata $(u, v)$, o custo incremental de enviar uma unidade adicional de fluxo é
computado da seguinte forma: consulta-se o fluxo direto
(`direct_flow`) e o fluxo inverso (`inverse_flow`) na tabela `active_flows`. Se existe fluxo inverso, cancela-se
uma unidade — o custo incremental é a diferença entre o custo do cabo atual (com o fluxo reduzido) e o custo
anterior, resultando em um valor negativo (reembolso). Caso contrário, adiciona-se ao fluxo direto: o custo
incremental é a diferença entre o novo custo do cabo (com fluxo acrescido de 1) e o custo atual, podendo ser
zero se o cabo existente comportar a unidade extra.

Uma vez encontrado o caminho, percorre-se cada aresta da rota atualizando os fluxos em `active_flows`:
se há fluxo inverso, decrementa-se; caso contrário, incrementa-se o fluxo direto. Ao final do processamento
de todas as turbinas, as arestas com fluxo líquido positivo tornam-se os cabos físicos da solução, com os
custos computados via catálogo de cabos para o fluxo acumulado em cada segmento.

Além disso, vale ressaltar que na tese de Jenne, não existe menção ao tratamento de crossings. Entretanto,
evitar cruzamentos é um dos requisitos do problema, e por isso, a solução foi implementada considerando os cruzamentos.
Por causa disso, o desempenho do algoritmo foi prejudicado, visto que para cada iteração do Dijkstra, sempre que ele vai verificar
o custo de uma aresta, é necessário fazer o teste de cruzamento. O resultado disso é que para cada turbina, e para cada etapa do Dijkstra,
é necessário fazer um teste de interseção. Utilizando ferramentas de profiling foi observado que isso se tornou o maior gargalo
do algoritmo.


== Exemplo comparativo

A Figura 1 e a Figura 2 mostram, respectivamente, as soluções geradas pelo Sweeping Multi-start e pelo DIJNOR.
Em ambas as imagens, a subestação é representada pelo quadrado vermelho ao centro, as turbinas pelos círculos
numerados, e as cores das arestas indicam a capacidade do cabo alocado (tons mais escuros para cabos de maior
bitola).

#figure(
  image("../images/n50_s01_t01_w01-SweepingMultistart.png", width: 100%),
  caption: [Solução gerada pelo Sweeping Multi-start. Custo total: R\$ 36.176.224.],
)


#figure(
  image("../images/n50_s01_t01_w01-DIJNOR.png", width: 100%),
  caption: [Solução gerada pelo DIJNOR sobre a instância. Custo total: R\$ 40.566.400.],
)

Visualmente, o Sweeping Multi-start produz topologias com organização radial bem definida:
as turbinas são agrupadas em setores angulares, cada setor forma uma subárvore conectada à subestação por
um cabo tronco, e o fluxo é distribuído uniformemente entre os grupos. A estrutura final é sempre uma árvore.

O DIJNOR, por sua vez, exibe um comportamento distinto. A estratégia de processar cada turbina buscando o
caminho incremental mais barato, levando em conta os cabos já instalados, faz com que turbinas consecutivas
frequentemente se conectem a um mesmo nó intermediário já atendido, concentrando fluxo precocemente. Como
o algoritmo não pode desfazer conexões já estabelecidas — ao contrário do SSP
original, que utilizaria ciclos negativos para realocar fluxo —, essa concentração se torna irreversível.
O resultado é uma topologia que frequentemente se afasta de uma árvore: um ou poucos nós próximos à
subestação precisam bifurcar o fluxo em múltiplos cabos para respeitar os limites de capacidade,
resultando em uma estrutura mais ramificada e onerosa.
