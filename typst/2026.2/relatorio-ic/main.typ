#import "template.typ": project

#set text(lang: "pt", region: "BR")


#show: project.with(
  title: "Relatório de estudos",
  subtitle: "Relato sobre estudos relacionados ao problema de roteamento de cabos em fazendas eólicas offshore",
  abstract: [
    Esse texto busca relatar todo o processo de pesquisa na Iniciação Científica na qual estou participando com guia da professora Sílvia Maia. O tema central da pesquisa de Iniciação Científica é investigar o funcionamento de heurísticas baseadas em fluxo a custo mínimo em detrimento das heurísticas baseadas em árvore geradora mínima para o problema de roteamento de cabos em fazendas eólicas offshore.
  ],
  subject: "PROJ-CS881 PROJETO - III",
  degree: "Bacharelado em Tecnologia da Informação",
  stream: "Ciência da Computação",
  guide: (
    name: "Dra. Sílvia Maia",
    designation: "Professora Associada",
    department: "Departamento de Informática e Matemática Aplicada",
  ),
  authors: (
    (
      name: "Raoni Silva",
      department: "Tecnologia da Informação",
      email: "raoni.silvestre@hotmail.com",
    ),
  ),
  department: "Departamento de Informática e Matemática Aplicada",
  institute: "Universidade Federal do Rio Grande do Norte",
)

= Introdução

Com o aquecimento global se tornando uma realidade cada vez mais presente no nosso mundo, se reúnem esforços para minimizar seus impactos, buscando alternativas aos principais causadores desse problema no sistema em que vivemos. Nesse contexto, as fontes de energias renováveis são uma peça fundamental para a transição do mundo para o uso de energia limpa, e a energia eólica se tornou um dos principais substitutos nesse contexto, aliado com a energia solar.

Dessa forma, existe uma grande necessidade de se instalar novas fazendas de energia eólica por todo o mundo, e a instalação é a parte mais cara do processo, existem estudos (buscar fontes dos artigos) que mostram que cerca de 30% do custo de uma fazenda eólica se dá pela sua instalação (ou é 30% do custo de instalação é proveniente de instalação de cabos).

Com isso, dois problemas principais começaram a ser estudados nas últimas décadas para resolver esse problema: o problema de roteamento de cabos e o problema de posicionamento de turbinas. Nessa IC (Iniciação Científica), focaremos no problema de roteamento de cabos.

Com isso, nesse texto busca-se relatar tudo que foi estudado e trazer os resultados destrinchados, além de indicar melhorias futuras.

== Objetivos

Comparar algoritmos de fluxo a custo mínimo, propostos pelo grupo de Sasha Gritzbach @jenne2020 com algoritmos de árvore geradora mínima, proposto pelo grupo do David Cazzaro @cazzaro2020.

Isso se deve pois existe uma lacuna experimental dessa abordagem inicial proposto pelo grupo de Sasha com o bem estabelecido estado da arte implementado por cazzaro.

O objetivo final, então, é colocar os algoritmos num fight para ver quem se sai melhor, porquê se sai melhor e indicativos relevantes sobre os dois.

= Trabalhos Relacionados

De relevante e extremamente relacionado, temos o trabalho de Martina Fischetti @fischetti2018 sobre roteamento de cabos considerando perda de potência, onde ela descreve formalmente usando programação inteira linear mista o problema, suas restrições e sua função objetivo.

Após isso, temos o artigo de Cazzaro @cazzaro2020 que propõe uma heurística construtiva extremamente simples, eficiente e que gera sempre soluções viáveis em um curto espaço de tempo. Além de utilizar essa heurística com diferentes meta-heurísticas, como Simulated Annealing, Busca Tabu, VNS, gerando ótimas soluções iniciais e melhorias significativas após a execução dos testes.

Concomitantemente ao artigo de Cazzaro, o pesquisador Marc Jenne @jenne2020 faz uma revisão dos algoritmos de fluxo à custo mínimo aplicados ao OWFCR, onde ele propõe uma versão modificada do algoritmo Sucessive Shortest Path(SSP) que gera soluções para o problema.

Em 2023, o pesquisador Sasha Gritzbach propõe um algoritmo de Negative Cycle Canceling @gritzbach2023engineeringnegativecyclecanceling como heurística de melhoria para soluções do OWFCR. Em seu artigo, ele mostra que a utilização do método consegue melhorar soluções em um curto espaço de tempo, para algumas instâncias.

== Revisão da Literatura

- Ver sobre Berzan, et al.
- Ver sobre peréz rua

= Modelagem

A modelagem definida a seguir é baseada no trabalho de @fischetti2018, com simplificações relacionadas a perda de potência ao longo do tempo.

Dado que existe uma solução para o problema de posicionamento de turbinas, o problema que resta é definir uma conexão ótima de cabos entre todas as turbinas, dadas as subestações, minimizando o custo da seleção de cabos. Além disso, são definidas as seguintes restrições:

- O fluxo de energia saindo de uma turbina deve ser suportado por um único cabo.
- Existem diferentes cabos, com diferentes custos e capacidades estão disponíveis.
- O fluxo de energia de cada conexão não pode superar a capacidade do cabo instalado.
- Existe um limite superior $C$ para a quantidade de conexões diretas a cada subestação.
- Cruzamentos entre cabos deve ser evitado.

Com essas restrições, seja $G = (V, A)$ um grafo direcionados, sem loops e completo, com $V_t$ definido pelas posições das turbinas e $V_s$ as posições das subestações, com isso, teremos $V = V_t union V_s$. Todos os nós tem coordenadas associadas em um plano, dessa forma é possível verificar se duas arestas no grafo se cruzam, pois teremos os dois segmentos de reta definido pelo nó inicial e final de cada aresta.

Seja $T$ o conjunto de cabos disponíveis utilizados. Para todo tipo de cabo $t in T$, existe uma capacidade $k_t >= 0$ e um custo unitário $u_t >= 0$. Cada arco custa $C(t, i, j)$, onde

$ C(t, i , j) = u_t dot "dist"(i, j) $

e ele é definido para todo arco $(i, j) in A$ e para cada cabo $t in T$, onde $"dist"(i, j)$ é a distância euclidiana entre os nós $i$ e $j$.

Para cada arco $(i, j) in A$, é utilizada uma variável contínua $f_(i, j) >= 0$ para representar o fluxo de energia de $i$ para $j$. Para cada $t in T, (i, j) in A$, é definido a variável binária $x_(i, j)^t$, com o seguinte significado:


#align(center, table(
  columns: (auto, auto),
  stroke: none,
  align: (left, left),
  [
    $
      x_(i, j)^t = cases(
        1 & "se" (i, j) "é construido com " t ".",
        0 & "caso contrário",
      )
    $
  ],
  [$(i, j) in A, t in T$],
))


Por fim, seja $y_(i, j)$ a variável binária que indica se algum cabo foi selecionado para aquele arco, ou seja,

#align(center, table(
  columns: (auto, auto),
  stroke: none,
  align: (left, right),
  [$ y_(i, j) = sum_(t in T) x_(i, j)^t, $], [$(i, j) in A.$],
))

Além disso, definimos um conjunto genérico $C subset.eq A times A$ de pares de arcos que se cruzam, com a propriedade de que dois arcos $(i, j)$ e $(h, k)$ não podem ser construídos simultaneamente se $((i, j), (h, k)) in C$.


== Função objetivo

Dessa forma, a função objetivo do problema é definida por

#align(center, $min sum_((i, j) in A) sum_(t in T) c_(i, j)^t x_(i, j)^t$)

Ou seja, buscamos minimizar o somatório das arestas selecionadas, escolhendo um cabo para cada aresta.

== Restrições

Além disso, as seguinte restrições se aplicam para o problema:


#align(center, $limits(sum)_(t in T) x_(i,j)^t = y_(i, j), (i, j) in A$)

Como $y_(i, j)$ é binária, apenas um dos tipos de cabo pode ser selecionado.

#align(center, $limits(sum)_(i in V: i != h) (f_(h, i) - f_(i, h)) = P_h, h in V_t union V_s$)

O fluxo saindo de um nó é igual a quantidade de energia entrando no nó, adicionada da energia do próprio nó ($h$). Essa restrição corresponde a conservação de energia em um nó.

#align(center, $limits(sum)_(t in T) k_t x_(i, j)^t >= f_(i, j), (i, j) in A$)

O fluxo não excede a capacidade do cabo instalado.

#align(center, $limits(sum)_(j in V: j != h) y_(h, j) = 1, h in V_t$)

Apenas um tipo de cabo pode sair da turbina.

#align(center, $limits(sum)_(j in V: j != h) y <= 1, h in V_s$)

Nenhum cabo pode sair da subestação.

#align(center, $limits(sum)_(i in V: i != h) y_(i, h) <= C, h in V_s$)

O máximo de cabos que pode chegar na subestação deve ser menor ou igual a $C$.

#align(center, $y_(i, j) + y_(j, i) + y_(h, k) + y_(k, h) <= 1, ({i, j}, {h, k}) in C$)

O cruzamento entre arestas é proibido.

#align(center, $x_(i, j)^t in {0, 1}, (i, j) in A, t in T$)
#align(center, $y_(i, j) in {0, 1}, (i, j) in A$)
#align(center, $f_(i, j) >= 0, (i, j) in A.$)

Os últimos três definem quais valores cada variável pode assumir. Ou seja, $y_(i, j)$ e $x_(i, j)^t$ são variáveis binárias e $f_(i, j)$ é positivo.

== Complexidade

Em @fischetti2018, é demonstrado que o problema é da classe NP-árduo.

= Implementações

A implementação de todos os algoritmos foi feita em Rust, na versão 1.95 com features do canal nightly do rust (para ajudar no SIMD).

== Casos de teste

Os casos de teste utilizados foram aqueles disponibilizados em @cazzaro2020, via contato direto com o grupo de pesquisa e repassado para mim. Dessa forma, temos 220 casos de testes ao todo. Cada caso de teste é um par de arquivos de texto `.turb` e `.cable`, onde são definidos no `.cable`:

1. A capacidade do cabo, exemplo: cabo que suporta a energia proveniente de 5 turbinas
2. A metragem do cabo, por exemplo: O cabo custa R\$500,00 reais o metro.
3. A quantidade disponível. Esse valor sempre é 999 para indicar que pode-se utilizar qualquer um dos cabos qualquer quantidade de vezes.

E no `.turb` temos:

1. Uma lista de `[x, y, capacidade]`, onde $(x, y)$ formam a posição no plano dessa turbina ou subestação e `capacidade` diz quantas unidades de energia aquele nó entrega/recebe. Capacidade é sempre 1 quando é uma turbina e um valor negativo indica que é uma subestação, onde `-capacidade` é o limite d e conexões para essa subestação.

Nos casos de teste disponíveis, são todas formadas com a primeira linha do `.turb` sendo a subestação e as linhas seguintes sendo as turbinas.

Além disso, existem casos de teste de com 50, 80 e 120 turbinas. Cada caso de teste é o resultado de uma análise considerando diferentes regiões, modelos de turbinas, situações do vento, baseado em locais que realmente existem.

Na descrição dos casos de teste é explicado que a posição da subestação pode ser alterada livremente para fins de teste, nos resultados mostrados nesse relatório, consideramos a subestação centralizada no meio pois o SSP se comporta melhor nesses casos. (TODO: MOSTRAR QUE O SSP FUNCIONA MELHOR COM SUBESTAÇÃO NO MEIO)

== Pré-processamento

As duas coisas interessantes sobre esse problema que nos permitem otimizar antes de iniciar os algoritmos são as distâncias, que são colocadas em uma matriz $M : |V|times|V|$, onde $M_(i, j) = "dist"(i, j)$, e as interseções entre arestas.

=== Inteseções de arestas

Uma das restrições centrais do OWFCR é a planaridade: os cabos não podem se cruzar no terreno Por isso, toda solução precisa garantir que nenhum par de segmentos se intersecte. Isso vale para toda aresta que entra na solução, a pergunta que se repete milhões de vezes é a mesma:

- "Se eu ligar o nó u ao nó v, esse segmento cruza alguma aresta que já está na solução?"

Responder a pergunta acima não é barato, porque o parque é modelado como um grafo completo: todo nó pode, em princípio, conectar-se a todo outro nó. Num parque com $n$ nós, o número de pares não-ordenados de vértices é

$ E = (n(n-1))/2 $

Para $n = 120$ turbinas, $E = 7140$ arestas possíveis. A cada tentativa de inserir uma aresta, o jeito ingênuo é percorrer todas as arestas já ativas e rodar um teste geométrico contra cada uma. Como isso acontece dentro dos loops mais apertados dos algoritmos, o teste de interseção se tornou o gargalo dominante de toda a implementação.

A implementação passou por três fases.
- Versão 1 — testar na hora (ingênua). Cada consulta percorre as arestas ativas e aplica o teste geométrico uma a uma. O custo de cada inserção cresce com o tamanho da solução corrente ($O(k)$, com $k$ até $E$), e o mesmo teste é refeito incontáveis vezes ao longo da execução.
- Versão 2 — pré-computar numa matriz de booleanos. A observação-chave: os cruzamentos são estáticos. O grafo é conhecido desde a carga da instância, e "a aresta $A$ cruza a aresta $B$?" é uma pergunta cuja resposta não muda nunca. Então por que recalcular?
A solução foi responder a essa pergunta uma única vez, no pré-processamento, e guardar todas as respostas numa matriz. Para isso, cada aresta precisava de um número compacto e único. Como as arestas não são direcionadas (o par $\{u,v\}$ é o mesmo que $\{v,u\}$), usou-se uma função de pareamento que mapeia cada par não-ordenado para um inteiro entre $0$ e $E-1$:
$ "id"(u, v) = (max(u, v)(max(u, v) - 1)) \/ 2 + min(u, v) $
Com isso, a matriz $M$ guarda, na posição $(i,j)$, se a aresta $i$ cruza a aresta $j$. A consulta virou um lookup $O(1)$.
- Versão 3 — matriz de bits + SIMD (a atual). A matriz de booleanos ainda era grande e, pior, cada consulta lia a linha inteira. A versão final empacota cada linha em blocos de 64 bits, transformando a consulta de interseção numa operação de AND bit-a-bit entre a linha da aresta candidata e uma máscara com as arestas ativas — e ainda vetorizada com SIMD.

O ponto de partida de toda essa estrutura é o teste geométrico que decide, para um único par de segmentos, se eles se cruzam. Dados três pontos $a$, $b$ e $c$, o teste de orientação responde se a tripla descreve uma curva no sentido anti-horário, comparando o sinal do produto vetorial — ou seja, da área assinada do triângulo que os três pontos determinam:

$ "ccw"(a, b, c) = (c_y - a_y)(b_x - a_x) > (b_y - a_y)(c_x - a_x) $

Dois segmentos $(p_1, q_1)$ e $(p_2, q_2)$ se cruzam exatamente quando as extremidades de cada um ficam em lados opostos da reta definida pelo outro, o que se traduz em orientações distintas:

$ "ccw"(p_1, p_2, q_2) != "ccw"(q_1, p_2, q_2) quad "e" quad "ccw"(p_1, q_1, p_2) != "ccw"(p_1, q_1, q_2) $

Segmentos que compartilham uma extremidade — cabos adjacentes — são descartados antes do teste, pois não configuram cruzamento.

Resolvido o teste de um par isolado, resta numerar as arestas para guardar todas as respostas em matriz. A função de pareamento faz exatamente isso. Para um parque com $n = 4$ nós existem $E = 6$ arestas, numeradas na ordem $(0, 1)$, $(0, 2)$, $(1, 2)$, $(0, 3)$, $(1, 3)$ e $(2, 3)$, recebendo os identificadores de $0$ a $5$: uma bijeção sem buracos, exatamente o que a indexação exige. No carregamento, percorre-se cada par de arestas aplicando o teste acima e marca-se, na linha da aresta $i$, o bit correspondente à aresta $j$. Como cada linha é empacotada em blocos de 64 bits, a aresta $j$ ocupa o bloco $j \/ 64$ e o bit $j mod 64$. No exemplo com seis arestas, cada linha cabe em um único bloco: se a aresta 0 cruza apenas a aresta 5, sua linha é um inteiro cujo único bit aceso é o de posição 5. No caso real, com $n = 120$, são $ceil(7140 \/ 64) = 112$ blocos por linha, formando uma matriz de cerca de 6,4 MB, construída uma única vez ao custo de $O(E^2)$ testes geométricos.

Durante a execução, a solução carrega uma máscara de bits em que o bit $i$ indica se a aresta $i$ está presente. Inserir ou remover uma aresta reduz-se a ligar ou desligar um único bit, e a pergunta original — "essa aresta cruza algo que já está na solução?" — vira um AND bit-a-bit entre a linha pré-computada da aresta candidata e a máscara: se o resultado for não nulo, há cruzamento. O que era um laço sobre até $E$ arestas ativas com testes geométricos passou a ser uma operação vetorial sobre 112 inteiros, na prática $O(E \/ 64)$.

O passo final explora o SIMD disponível no Rust nightly: processando quatro blocos por instrução com `u64x4`, cada iteração compara 256 arestas de uma só vez. Há duas variantes dessa consulta. A primeira, `any_crossings`, faz um OR acumulado e retorna no primeiro bit aceso, servindo para validar movimentos; a segunda, `count_crossings`, soma todos os bits acesos, servindo para contabilizar o custo exato de penalidade na função objetivo. Com isso, o teste de cruzamento, ainda é custoso, mas diminuiu em muito o custo computacional dos algoritmos implementados.

Resta um último detalhe que decorre da escolha do identificador canônico. Como cada par de arestas que se cruza é contado duas vezes, uma do ponto de vista de cada aresta, o total de cruzamentos de uma solução é dividido por dois. Além disso, embora o fluxo seja representado internamente de forma direcionada — com $u -> v$ e $v -> u$ ocupando índices distintos —, o cruzamento é uma propriedade do par não-ordenado, de modo que a matriz e a máscara usam sempre o identificador canônico da função de pareamento, nunca o índice direcionado.

= Algoritmos Heurísticos

Os algoritmos heurísticos implementados são o Sweeping Multistart, baseado no artigo @cazzaro2020 do Cazzaro, o Sucessive Shortest Path baseado na tese @jenne2020 de Marc Jenne e o Negative Cycle Canceling proposto por @gritzbach2023engineeringnegativecyclecanceling Sasha Gritzbach.

== Sweeping Multistart

O algortimo de sweeping multistart se baseia em uma clusterização angular dos nós das turbinas em relação a subestação, após isso teremos $k$ grupos disjuntos de turbinas. Para cada grupo de turbinas se executa o algoritmo de prim para recuperar a MST daquele grupo + a subestação. A quantidade de grupos vai separar de modo que cada grupo tenha uma quantidade de turbinas menor do que a capacidade do maior cabo possível e também de modo que a quantidade de grupos seja menor do que o limite da subestação.

Com isso, cada árvore gerada no cluster é uma subárvore válida para o problema, e como os conjuntos são disjuntos em um espaço métrico, temos que não haverá cruzamento entre turbinas de nós diferentes.

Esse algoritmo altera seu resultado de acordo com a turbina inicial escolhida, a direção escolhida e a quantidade mínima e máxima dos grupos possível. Logo o sweeping é uma execução considerando uma direção, uma turbina e um tamanho de grupo, o multistart itera por todas as variantes possiveis e retorna a melhor escolha possível(com custo menor).

== Sucessive Shortest Path

O sucessive Shortest Path se baseia em continuamente computar o caminho mais curto (via dijkstra) até a subestação. Uma coisa interessante é que ele se utiliza de cabos previamente criados para verificar os novos caminhos, e como adicionar uma unidade de energia em um cabo que não está estourado implica que o custo de passar por ali é 0, o algoritmo tende a criar longas cadeias de turbinas. Na tese dele existem algumas variantes, e denota uma como a melhor variante, chamada DIJNOR: Dijkstra com pesos normais, essa é a variante implementada.

O algoritmo clássico não funciona aqui devido a natureza do problema, em que o custo de uma áresta não é linear, mas se comporta como uma step-function. Devido a isso, o algoritmo se perderia facilmente em ciclos negativos, gerando loops infinitos. Devido a isso, é imposta uma restrição no algoritmo: para cada iteração do dijkstra, só é possível visitar um nó uma única vez. Com essa restrição o algoritmo garante a finalização, mesmo qque viole algumas outras limitações.

=== Variantes

Uma coisa em aberto na tese de Jenne, é que a ordem das turbinas importa para a solução final, mas isso não foi investigado na tese. Por isso, três variantes foram implementadas para esse problema:

1. Sem ordenação específica: A ordem de leitura dos nós é a ordem em que é feito o caminho mais curto.
2. O mais próximo primeiro: A ordem é em relação a distância euclidiana, do menor pro maior.
3. O mais longe primeiro: A ordem é em relação a distância euclidiana, do maior para o menor.

Em linhas gerais, o mais longe primeiro se mostrou pior do que os outros, enquanto o primeiro e o segundo modo ficaram parelhos em linhas gerais.

Por fim, a implementação e modelagem proposta por Jenne não considera que a solução precisa ser uma árvore, e isso não é mencionado em sua tese, como consequência, o algoritmo implementado não se preocupa com isso e gera soluções inválidas.
Como meio para contornar isso, foi implementada uma variante que obriga o algoritmo a gerar árvores obrigatoriamente, pulando escolhas que gerariam bifurcações na soluções.
Isso faz com que grande parte das soluções acabem com cruzamento, mas são árvores válidas, muito diferentes das geradas pelo sweeping e que os cruzamentos são removíveis facilmente em processos meta-heurísticos de melhoria da solução.

== Negative Cycle Canceling

= Algoritmos Meta-heurísticos

== Variable Neighbourhood Search

== Genético

== Memético

== Transgenético

= Experimentos Computacionais

== Comparativo de heurísticas

== Comparativo de meta-herurística

= Conclusão e Trabalhos Futuros


#bibliography("refs.bib")
