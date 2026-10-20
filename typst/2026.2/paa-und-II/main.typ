#import "@preview/charged-ieee:0.1.4": ieee
#import "@preview/lovelace:0.3.1": *


#show: ieee.with(
  title: [Heurística sweeping multi-start para o roteamento de cabos em fazendas eólicas],
  abstract: [
    Este artigo apresenta uma revisão da literatura sobre heurísticas construtivas para o problema de roteamento de cabos em fazendas eólicas, conhecido como OWFCR (Offshore Wind Farm Cable Routing), um problema de otimização combinatória presente no processo de instalação de fazendas eólicas, onde melhorias nos algoritmos podem levar a grandes economias nesse processo. Este artigo revisa o estado da arte e implementa a heurística construtiva Sweeping Multistart, proposta por Cazzaro et al. @cazzaro2020. O algoritmo foi testado sobre as 220 instâncias de Fischetti e Pisinger, avaliando qualidade da solução e tempo de execução.
  ],
  authors: (
    (
      name: "Raoni Silva",
      department: [Departamento de Matemática e Informática Aplicada],
      organization: [UFRN],
      location: [Natal, RN],
    ),
    (
      name: "Vinicius Lima",
      department: [Departamento de Matemática e Informática Aplicada],
      organization: [UFRN],
      location: [Natal, RN],
    ),
    (
      name: "Lucas Manoel",
      department: [Departamento de Matemática e Informática Aplicada],
      organization: [UFRN],
      location: [Natal, RN],
    ),
  ),
  index-terms: (
    "Wind Farm Cabling Problem",
    "Otimização Combinatória",
    "Heurísticas Construtivas",
  ),
  bibliography: bibliography("refs.bib"),
  figure-supplement: [Fig.],
)

= Introdução

A transição energética global tem impulsionado fortemente a adoção de fontes renováveis para mitigar os impactos climáticos. A energia eólica, em particular,
apresenta um crescimento acelerado e contínuo. Conforme apontado pelo Global Wind Energy Council @gwec2025, a capacidade eólica mundial se expande de forma expressiva,
consolidando a tecnologia como um pilar fundamental tanto em instalações terrestres (onshore) quanto marítimas (offshore).

No cenário brasileiro, essa matriz energética possui um papel estratégico de enorme destaque. Segundo a Associação Brasileira de Energia Eólica @abeeolica2024,
em 2024, o estado do Rio Grande do Norte figurou como o segundo maior produtor de energia eólica do Brasil, evidenciando o seu protagonismo na transição energética
do país e a relevância da eficiência técnica e financeira dos projetos instalados na região.

Apesar da expansão do setor, o planejamento e a construção de uma fazenda eólica são processos complexos que exigem investimentos massivos. A infraestrutura elétrica,
necessária para interligar as turbinas e transportar a energia gerada até a rede, consome uma parcela crítica do orçamento do projeto. De acordo com González et al.
@gonzalez2014, as despesas com a infraestrutura elétrica chegam a representar de 15% a 30% dos custos iniciais totais de uma fazenda eólica offshore. Logo, otimizar o
roteamento e a conexão de cabos entre as turbinas é um passo indispensável para minimizar os custos de instalação.

Diante dessa necessidade de eficiência orçamentária, destaca-se o problema de roteamento de cabos em fazendas eólicas, formalmente conhecido na literatura como Offshore Wind Farm Cable Routing (OWFCR). O presente artigo tem como objetivo implementar e analisar uma abordagem heurística para o OWFCR: Sweeping Multi-start, baseada em clustering geométrico e árvore geradora mínima. A análise é realizada sobre o conjunto de 220 instâncias de Fischetti e Pisinger, examinando a qualidade das soluções, o tempo de execução e as características estruturais da topologia gerada.


= Descrição do Problema

O Offshore Wind Farm Cable Routing (OWFCR) surge da necessidade de interligar as turbinas de um parque gerador a uma subestação de coleta, minimizando os custos totais de instalação.
Como a infraestrutura e os materiais envolvidos na implementação de parques eólicos são extremamente caros, a otimização da topologia da rede e a alocação correta dos recursos
representam uma economia financeira substancial no orçamento final do projeto.

Na formulação base, os cabos disponíveis para a conexão das turbinas são produzidos e comercializados em um número limitado de especificações, possuindo capacidades máximas de transmissão de energia fixas. O custo de aquisição e instalação de cada cabo cresce de acordo com a sua capacidade de transmissão. Dessa forma, o dimensionamento inadequado — como a escolha de cabos mais robustos e caros do que o necessário para um trecho específico — gera desperdício financeiro direto. O objetivo final é decidir, de forma ótima, quais cabos conectarão quais turbinas até a subestação.

Em linhas gerais, considera-se que todas as turbinas operam com potência idêntica, sendo a geração de cada unidade normalizada para o valor 1. Sob essa premissa, a capacidade de transmissão de cada tipo de cabo indica quantas turbinas ele consegue atender de forma acumulada. Além disso, a subestação possui um limite de portas para o recebimento de conexões diretas.

Por fim, a construção real impõe restrições físicas e topológicas. A principal delas é a restrição de não cruzamento de cabos. Em ambientes reais, cruzar um cabo por cima de outro exige um maior cuidado e é mais propenso a problemas de funcionamento com o tempo, então normalmente essa é uma das restrições para o problema.

== Variações do Problema

A formulação base do OWFCR atende às premissas iniciais de viabilidade, no entanto, a literatura apresenta diversas variações que buscam aproximar a modelagem das complexidades encontradas no mundo real. As principais extensões incluem:

1. *Múltiplas Subestações:* A formulação base pode ser estendida para considerar o roteamento direcionado a mais de uma subestação coletora. Essa variação adiciona ao modelo a complexidade de decidir previamente ou simultaneamente qual turbina deve ser alocada a qual subestação, com o objetivo de balancear a carga da rede e minimizar o custo total de cabeamento.

2. *Perdas de Energia:* Além do custo de capital da instalação, algumas modelagens incorporam a perda de potência dissipada na forma de calor ao longo dos cabos durante toda a vida útil do parque. Conforme demonstrado em @fischetti2018, essa variação pode justificar a escolha de um cabo mais caro e de maior capacidade de passagem de energia no presente para garantir eficiência energética e maior retorno financeiro a longo prazo.

3. *Desvio de Obstáculos:* Considera a presença de zonas restritas no terreno, como reservas naturais, outros parques eólicos ou cabos já existentes e irregularidades severas no terreno. Modelagens mais completas, como a proposta em @fischetti2018, exigem que o roteamento desvie dessas áreas, frequentemente utilizando a inserção de nós de ramificação artificiais (Steiner points) para encontrar caminhos viáveis.

== Modelagem em grafos

A modelagem definida a seguir é baseada no trabalho de @fischetti2018, com simplificações relacionadas à perda de potência ao longo do tempo.

Dado que existe uma solução para o problema de posicionamento de turbinas, o problema que resta é definir uma conexão ótima de cabos entre todas as turbinas, dadas as subestações, minimizando o custo da seleção de cabos. Além disso, são definidas as seguintes restrições:

- O fluxo de energia saindo de uma turbina deve ser suportado por um único cabo.
- Estão disponíveis diferentes tipos de cabo, com custos e capacidades distintos.
- O fluxo de energia de cada conexão não pode superar a capacidade do cabo instalado.
- Existe um limite superior $C$ para a quantidade de conexões diretas a cada subestação.
- Cruzamentos entre cabos devem ser evitados.

Com essas restrições, seja $G = (V, A)$ um grafo direcionado, sem loops e completo, com $V_t$ definido pelas posições das turbinas e $V_s$ as posições das subestações, com isso, teremos $V = V_t union V_s$. Todos os nós têm coordenadas associadas em um plano, dessa forma é possível verificar se duas arestas no grafo se cruzam, pois teremos os dois segmentos de reta definidos pelo nó inicial e final de cada aresta.

Seja $T$ o conjunto de cabos disponíveis. Para todo tipo de cabo $t in T$, existe uma capacidade $k_t > 0$ e um custo unitário $u_t > 0$. Cada arco custa $c_(i, j)^t$, onde

$ c_(i, j)^t = u_t dot "dist"(i, j) $

e ele é definido para todo arco $(i, j) in A$ e para cada cabo $t in T$, onde $"dist"(i, j)$ é a distância euclidiana entre os nós $i$ e $j$.

Para cada arco $(i, j) in A$, é utilizada uma variável contínua $f_(i, j) >= 0$ para representar o fluxo de energia de $i$ para $j$. Para cada $t in T, (i, j) in A$, define-se a variável binária $x_(i, j)^t$, com o seguinte significado:


$
  x_(i, j)^t = cases(
    1 & "se" (i, j) "é construído com " t ".",
    0 & "caso contrário"
  )
$
$(i, j) in A, t in T.$

Por fim, seja $y_(i, j)$ a variável binária que indica se algum cabo foi selecionado para aquele arco, ou seja,

$ y_(i, j) = sum_(t in T) x_(i, j)^t, (i, j) in A. $

Além disso, definimos o conjunto $Omega$ de pares de arestas que se cruzam, com a propriedade de que duas arestas ${i, j}$ e ${h, k}$ não podem ser construídas simultaneamente se $({i, j}, {h, k}) in Omega$.


=== Função objetivo

Dessa forma, a função objetivo do problema é definida por

$ min sum_((i, j) in A) sum_(t in T) c_(i, j)^t x_(i, j)^t $

Ou seja, buscamos minimizar o somatório dos custos das arestas selecionadas, escolhendo um cabo para cada aresta.

=== Restrições

Além disso, as seguintes restrições se aplicam para o problema:

$ limits(sum)_(t in T) x_(i,j)^t = y_(i, j), (i, j) in A $

Como $x$ é binária, a igualdade garante no máximo um tipo de cabo por arco.

$ limits(sum)_(i in V: i != h) (f_(h, i) - f_(i, h)) = P_h, h in V_t union V_s $

em que $P_h = 1$ se $h in V_t$ e $P_h = -|V_t|$ se $h in V_s$. A diferença entre o fluxo que sai e o que entra em um nó é igual à sua geração líquida $P_h$, o que corresponde à conservação de energia.

$ limits(sum)_(t in T) k_t x_(i, j)^t >= f_(i, j), (i, j) in A $

O fluxo não excede a capacidade do cabo instalado.

$ limits(sum)_(j in V: j != h) y_(h, j) = 1, h in V_t $

Exatamente uma aresta sai de cada turbina.

$ limits(sum)_(j in V: j != h) y_(h, j) = 0, h in V_s $

Nenhum cabo pode sair da subestação.

$ limits(sum)_(i in V: i != h) y_(i, h) <= C, h in V_s $

O máximo de cabos que pode chegar na subestação deve ser menor ou igual a $C$.

$ y_(i, j) + y_(j, i) + y_(h, k) + y_(k, h) <= 1, ({i, j}, {h, k}) in Omega $

O cruzamento entre arestas é proibido.

$ x_(i, j)^t in {0, 1}, (i, j) in A, t in T $

$ y_(i, j) in {0, 1}, (i, j) in A $

$ f_(i, j) >= 0, (i, j) in A. $

Os últimos três definem quais valores cada variável pode assumir. Ou seja, $y_(i, j)$ e $x_(i, j)^t$ são variáveis binárias e $f_(i, j)$ é não negativo.

== Heurísticas construtivas encontradas na literatura

Diversas abordagens foram utilizadas para resolver o OWFCR, entre elas destacam-se o trabalho de @fischetti2018, onde o problema é formalmente modelado em programação inteira linear mista (MILP), possibilitando o uso de solvers prontos como o Gurobi para resolver instâncias pequenas ou médias do problema de forma exata, mas ainda se mostra inviável de solucionar em tempo hábil instâncias maiores devido à NP-dificuldade do problema. Em @vehiclerouting2015 é investigada uma heurística baseada na heurística de savings de Clarke and Wright com bons resultados. Em 2020 @cazzaro2020 é apresentada a heurística construtiva Sweeping Multistart, onde é investigado seu funcionamento junto de algoritmos meta-heurísticos. Por fim, @jenne2020 modela o OWFCR como um problema de fluxo à custo mínimo e investiga diversos algoritmos clássicos adaptados para as restrições do problema, nessa tese é mostrado que o Successive Shortest Path serve como heurística construtiva para o OWFCR. Por fim, em 2023 @Pérez-Rúa03102023 propõe uma heurística composta por três etapas, sendo a primeira, chamada de Two-Step Heuristic como heurística construtiva, sendo passada para uma etapa de remoção de cruzamentos de outra heurística chamada Cable Crossing Repair Heuristic (CCRH) e por fim uma etapa de melhoria por meio de uma heurística de Negative Cycle Canceling Refining Heuristic (NCCRH), que utiliza do clássico método de cancelamento de ciclos negativos para melhorar a solução da etapa anterior.


= Casos de testes

Para a avaliação e validação de algoritmos voltados ao WFCP, a literatura consolidou o uso de um conjunto de 220 instâncias de teste, formulado e disponibilizado sob
demanda por Fischetti e Pisinger (2018), como descrito em @fischetti2018. Esse benchmark destaca-se no estado da arte por ser estritamente fundamentado em dados operacionais
e restrições físicas de parques eólicos offshore reais da indústria.

O banco de dados é construído a partir de variações topológicas de cinco parques em operação na
Europa. A ampla diversidade dessas 220 instâncias é gerada pela combinação dessas disposições geométricas — que escalam de dezenas a centenas de turbinas — com
diferentes limites rigorosos de conexão nas subestações e múltiplos catálogos de cabos comerciais. Esse conjunto oferece um panorama completo e estabelece o ambiente
de validação rigoroso e padronizado necessário para atestar a competitividade, a escalabilidade e a robustez de novas heurísticas e modelos propostos para o problema.

== Formato das instâncias

Cada instância é composta por dois arquivos: um arquivo `.turb`, que descreve as coordenadas e o tipo de cada nó
do parque, e um arquivo `.cable`, que especifica o catálogo de cabos disponíveis.

O arquivo `.turb` contém uma linha por nó, com três colunas separadas por espaços: coordenada $x$ (leste),
coordenada $y$ (norte) e peso $w$. A primeira linha sempre representa a subestação, identificada por um peso
negativo ($w < 0$). O valor absoluto de $w$ — por exemplo, $|w| = 6$ — determina o limite máximo de conexões
diretas que a subestação pode receber. As linhas subsequentes representam as turbinas, todas com peso $w = 1$,
indicando que cada turbina gera uma unidade normalizada de potência.

O arquivo `.cable` contém exatamente três linhas, uma para cada tipo de cabo do catálogo. Cada linha especifica
três valores: a capacidade de transmissão (número máximo de turbinas cuja potência acumulada o cabo suporta),
o preço por metro e um indicador de disponibilidade. Os cabos são ordenados por capacidade crescente — um
catálogo típico oferece cabos para 5, 7 e 12 turbinas, com preços por metro proporcionais à capacidade.

= Implementação

A implementação foi realizada em Rust e está disponível em @owfcr2026, especificamente, o código relacionado ao método Sweeping está em #link("https://github.com/RaoniSilvestre/owfcr/blob/main/src/algorithms/sweep/base.rs", underline("base.rs")) e Sweeping Multistart em #link("https://github.com/RaoniSilvestre/owfcr/blob/main/src/algorithms/sweep/multistart.rs", underline("multistart.rs")).

O parque eólico é modelado como um grafo
completo, em que a distância euclidiana entre cada par de nós é pré-computada no
carregamento da instância, permitindo acesso em tempo $O(1)$ a qualquer aresta.

A solução é representada por dois vetores centrais. O vetor `parents` define a
topologia final: para cada turbina $v$, `parents[v]` é o nó para o qual ela envia
sua energia, de modo que a aresta incidente a $v$ é exatamente $(v, "parents"[v])$;
a subestação $s$ é a raiz e não possui pai. O vetor `flows` guarda o fluxo que
atravessa cada nó em direção ao pai: `flows[v]` inicia em 1 para cada turbina (uma
unidade normalizada de geração) e é acumulado das folhas em direção à subestação —
`flows[parents[v]] += flows[v]` — até que `flows[v]` seja o número de turbinas cuja
energia escoa pela aresta $(v, "parents"[v])$. Dessa forma, o tipo de cabo de cada
aresta é determinado diretamente por `flows[v]`.

Como a restrição de não cruzamento é verificada a cada aresta candidata — operação
que se repete milhões de vezes —, ela é resolvida uma única vez, na carga da
instância, e armazenada em uma matriz de inteiros. Cada aresta não direcionada
$\{u, v\}$, com $u != v$, recebe um identificador único e sem buracos por uma função
de pareamento:

$ "id"(u, v) = (max(max - 1)) / 2 + min $

$"onde" max = max(u, v), min = min(u, v).$

Em um grafo completo com $n$ nós há $E = n(n-1) \/ 2$ arestas, numeradas de $0$ a
$E - 1$; para uma instância de 120 turbinas ($n = 121$ nós), são $E = 7260$ arestas.
O cruzamento entre dois segmentos é verificado pelo método *counter-clockwise*
(ccw), que checa se as extremidades de cada segmento ficam em lados opostos da reta
definida pelo outro; arestas que compartilham uma extremidade são descartadas antes
do teste, pois cabos adjacentes não configuram cruzamento

A matriz é construída percorrendo todo par de arestas $(i, j)$ e marcando o bit $j$
da linha $i$ sempre que a aresta $i$ cruza a aresta $j$. Cada linha é um vetor de
palavras de 64 bits: o bit $j$ ocupa a palavra $j "div" 64$, posição $j mod 64$. Para
$n = 121$, cada linha tem $ceil(7260 \/ 64) = 114$ palavras e a matriz ocupa cerca de
6,6 MB, construída ao custo de $O(E^2)$ testes geométricos. Durante a execução, a
solução mantém uma máscara de bits em que o bit $i$ indica se a aresta $i$ está
ativa; a pergunta "a aresta $(u, v)$ cruza algo já ativo?" reduz-se a um AND bit a
bit entre a linha de $(u, v)$ e a máscara. Como a verificação passa a ser apenas um
AND bitwise, ela pode ser acelerada por SIMD.

== Heurística Sweeping

A heurística base recebe três parâmetros — a turbina inicial do Sweeping, o tamanho do
grupo e a direção da varredura — e constrói uma solução em três etapas: ordenação
angular, agrupamento e construção das árvores. Primeiro, as turbinas são ordenadas
pelo ângulo polar em relação à subestação, formando uma lista circular. Em seguida,
são particionadas em grupos de $g$ turbinas consecutivas a partir da turbina inicial,
na direção escolhida; o último grupo pode conter menos turbinas. Para cada grupo,
executa-se o algoritmo de Prim enraizado na subestação, que preenche `parents[v]`
evitando, arestas que cruzariam as já inseridas no grupo. Por
fim, o fluxo é acumulado das folhas em direção à subestação em `flows`, e o custo é
somado por aresta a partir de `flows[v]`.

#figure(
  kind: "algorithm",
  supplement: [Algorithm],
  pseudocode-list(booktabs: true, numbered-title: [Sweeping])[
    + *Entrada:* grafo $G$, subestação $s$, catálogo de cabos, turbina inicial $"start"$, tamanho de grupo $g$, direção $"dir"$
    + $L <-$ turbinas ordenadas pelo ângulo em relação a $s$
    + grupos $<-$ particiona $L$ em grupos de $g$ turbinas consecutivas, a partir de $"start"$, na direção $"dir"$
    + $"parents"[v] <- s$, para todo $v != s$
    + *para* cada grupo em grupos *faça*
      + $"parents" <- "parents" union "Prim"("grupo" union {s}, "raiz" = s)$
    + $"flows"[v] <- 1$, para toda turbina $v$
    + processa as folhas em direção a $s$: $"flows"["parents"[v]] <- "flows"["parents"[v]] + "flows"[v]$
    + $"custo" <- 0$
    + *para* cada $v != s$ *faça*
      + $"cabo" <-$ menor cabo com capacidade $>= "flows"[v]$
      + $"custo" <- "custo" + "preço"("cabo") dot "dist"(v, "parents"[v])$
    + $"custo" <- "custo" +$ penalidades por cruzamento
    + $"custo" <- "custo" +$ penalidades por limite da subestação
    + *retorna* solução $("parents", "flows", "custo")$
  ],
) <alg:sweeping>

== Sweeping Multistart

O Sweeping Multistart repete a heurística base variando sistematicamente os três
parâmetros. O tamanho do grupo varia de $min_g = ceil(N \/ C)$ — o mínimo necessário
para respeitar o limite de portas da subestação — até a capacidade do maior cabo
disponível; a turbina inicial percorre todas as $N$ turbinas; e a direção alterna
entre horário e anti-horário. Para cada combinação, executa-se o Sweeping e retém-se
a solução de menor custo. Como cada execução é barata — apenas a construção de MSTs
sobre subconjuntos dos nós —, o custo total permanece reduzido.

#figure(
  kind: "algorithm",
  supplement: [Algorithm],
  pseudocode-list(booktabs: true, numbered-title: [Sweeping Multistart])[
    + *Entrada:* grafo $G$, subestação $s$ com limite $C$, catálogo de cabos
    + $min_g <- ceil(N \/ C)$
    + $max_g <-$ capacidade do maior cabo
    + $"melhor" <- infinity$
    + *para* $"start"$ de $0$ a $N - 1$ *faça*
      + *para* $"dir" in {"horário", "anti-horário"}$ *faça*
        + *para* $g$ de $min_g$ a $max_g$ *faça*
          + $S <- "Sweeping"("start", g, "dir")$
          + *se* $S."custo" < "melhor"."custo"$ *então* $"melhor" <- S$
    + *retorna* $"melhor"$
  ],
) <alg:sweeping-multistart>

= Complexidade em tempo

Sejam $N$ o número de turbinas e $G$ o tamanho dos grupos. A ordenação angular custa
$O(N log N)$ e o agrupamento, $O(N)$. Em seguida, executa-se um Prim em cada grupo:
um Prim bem implementado sobre $G$ nós de um grafo completo custa $O(G^2)$ e, como os
grupos particionam as $N$ turbinas, o custo total das árvores é
limitado por $O(N^2)$, equivalente a um único Prim por todo o grafo. A acumulação de fluxo e o
cálculo do custo percorrem cada nó uma única vez, em $O(N)$. Uma execução do
Sweeping é, portanto, $O(N^2)$, o custo do próprio Prim.

O Multistart executa $N dot 2 dot T$ varreduras, onde T é a quantidade tamanho de grupos possíveis para a instância.
Como $O(N dot 2 dot T) = O(N)$, logo a complexidade total do algoritmo é $O(N^3)$

Nessa análise, o teste de cruzamento é tratado como custo constante, por ser
pré-computado e vetorizado por SIMD. Estritamente, cada consulta percorre
$B = ceil(E \/ 64) = O(N^2)$ palavras, o que elevaria o pior caso a $O(N^4)$; na
prática, porém, os cruzamentos são raros e o AND bit a bit tem constante pequena, de
modo que o comportamento observado acompanha o $O(N^3)$.

= Aplicação do algoritmo

O algoritmo foi executado trinta vezes para cada instância, e conseguiu gerar soluções válidas em todos os casos. Devido à sua natureza determinística, não foi observada nenhuma alteração no comportamento entre as trinta execuções do algoritmo em cada instância. Segue a visualização de resultados nos casos de teste com uma instância com 50 turbinas:

#figure(
  supplement: [Fig.],
  grid(
    columns: (1fr, 1fr),
    gutter: 6pt,
    image("images/sweep-w50.png", width: 100%), image("images/sweep-multistart-w50.png", width: 100%),
  ),
  caption: [Sweeping (esquerda) e Sweeping Multistart (direita) — 50 turbinas.],
) <fig:comp-50>

#figure(
  supplement: [Fig.],
  grid(
    columns: (1fr, 1fr),
    gutter: 6pt,
    image("images/sweep-w80.png", width: 100%), image("images/sweep-multistart-w80.png", width: 100%),
  ),
  caption: [Sweeping (esquerda) e Sweeping Multistart (direita) — 80 turbinas.],
) <fig:comp-80>

#figure(
  supplement: [Fig.],
  grid(
    columns: (1fr, 1fr),
    gutter: 6pt,
    image("images/sweep-w120.png", width: 100%), image("images/sweep-multistart-w120.png", width: 100%),
  ),
  caption: [Sweeping (esquerda) e Sweeping Multistart (direita) — 120 turbinas.],
) <fig:comp-120>


= Resultados

A Tabela I resume os resultados agregados das duas heurísticas sobre as 220
instâncias, cada uma executada trinta vezes (6.600 execuções por algoritmo).

#figure(
  table(
    columns: 5,
    stroke: 0.5pt,
    align: center,
    [*Algoritmo*], [*Melhor*], [*Média*], [*Mediana*], [*Desvio Padrão*],
    [Sweeping Multistart], [29.56], [73.42], [51.15], [46.77],
    [Sweeping], [31.03], [78.90], [54.43], [50.08],
  ),
  kind: table,
  caption: [Resumo dos resultados. Valores em milhões.],
) <tab:resumo>

Os valores de média, mediana e desvio padrão são calculados sobre todas as 6600 execuções de cada algoritmo. Como o algoritmo é determinístico, é equivalente a calcular a média, mediana e desvio padrão sobre os 220 casos de teste.

Nota-se que o Sweeping Multistart domina o Sweeping em todas as métricas: melhor
custo, custo médio e mediana menores, além de desvio padrão mais baixo. O ganho é
relevante na prática: uma diferença de cerca de 5,5 milhões de reais no custo médio
(73,42 contra 78,90 milhões) representa economia direta na implantação do parque,
já que cada real a menos no roteamento dos cabos é um real a menos no orçamento do
projeto.

As Figuras a seguir comparam o custo médio, o tempo médio e o GAP relativo das duas
abordagens.

#figure(
  image("images/comp-custo-medio.png", width: 100%),
  caption: [Custo médio das soluções por porte de instância.],
) <fig:custo>

#figure(
  image("images/comp-tempo-medio.png", width: 100%),
  caption: [Tempo médio de execução por porte de instância.],
) <fig:tempo>

Interessante notar que uma única execução do Sweeping é extremamente rápida, e por isso nem sequer aparece no gráfico, pois fica com um tempo de execução inferior a 0.1 ms.

#figure(
  image("images/comp-gap-rel-ord.png", width: 100%),
  caption: [GAP relativo ao melhor custo encontrado (Sweeping Multistart), ordenado por instância.],
) <fig:gap>

A análise de gap mostrou uma melhora consistente da aplicação do método Multistart, melhorando pelo menos 2.5% da solução inicial encontrada pelo algoritmo Sweeping, até um máximo de 5% de melhora.

= Conclusão

Este artigo apresentou a heurística construtiva Sweeping Multistart para o problema
de roteamento de cabos em fazendas eólicas offshore (OWFCR). A heurística combina
varredura angular das turbinas, agrupamento em setores e construção de árvores
geradoras mínimas, repetindo o processo para diferentes turbinas iniciais, direções
e tamanhos de grupo.

Os experimentos sobre as 220 instâncias de Fischetti e Pisinger mostram que o
Multistart domina a versão base do Sweeping em todas as métricas, com ganhos na casa
de milhões de reais a um custo computacional baixo. Por ser determinística e rápida,
a heurística também se presta a gerar soluções iniciais de qualidade para
meta-heurísticas de refinamento.

Como trabalhos futuros, destaca-se a hibridização com a Variable Neighborhood Search,
seguindo a literatura, e a implementação de um Algoritmo Transgenético para o OWFCR, que
pode se utilizar das ideias do Sweeping Multistart tanto para uma inicialização de população
de alta qualidade, quanto de uma estratégia de intensificação aliado a uma busca local. Idealmente,
essas são ideias que podem superar os resultados reportados por Cazzaro et al.
