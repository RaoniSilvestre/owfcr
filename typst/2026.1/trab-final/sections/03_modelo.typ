= Modelagem em grafos

A modelagem em grafos do WFCP geralmente segue por duas linhas gerais, a primeira, definida por @fischetti2018, se baseia em ver o problema
como uma anti-arborescência enraizada na subestação, enquanto a abordagem proposta em @jenne2020 vê um problema como uma rede de fluxo a custo mínimo.
Formalmente, o problema é definido sobre um grafo direcionado $G = (V, A)$, onde a composição do conjunto de vértices $V$ varia conforme a necessidade
de pontos intermediários de conexão.

== Anti-arborescência e MILP

Nesta perspectiva, o layout final do parque é modelado como uma estrutura de árvore direcionada onde a energia flui das turbinas em direção à raiz (subestação).
O conjunto de vértices é definido como $V = V_T union V_R union V_S$, onde $V_T$ representa as turbinas, $V_R$ as subestações e $V_S$ os pontos de Steiner
(Steiner points). Estes últimos são nós de ramificação artificiais inseridos para permitir caminhos viáveis ao redor de obstáculos geográficos e otimizar as
junções de cabos fora das coordenadas das turbinas.

O modelo utiliza variáveis binárias para representar a seleção de arestas e o tipo de cabo, respeitando a
restrição de fluxo indivisível (unsplittable flow): toda a energia que sai de uma turbina deve ser transportada por um único cabo. Essa abordagem apresentada
por @fischetti2018 é amplamente utilizada como base de comparação com outros modelos heurísticos e meta-heurísticos, visto que permite o uso de solvers exatos
para encontrar soluções ótimas em instâncias de pequeno porte, servindo de parâmetro para avaliar a qualidade de outros algoritmos.

== Fluxo a custo mínimo

A abordagem de @jenne2020 modela o WFCP sob a ótica da teoria de redes de fluxo, formulando-o como um Problema de Fluxo a Custo Mínimo (Minimum-Cost Flow - MCF). Nesta
formulação, cada turbina atua como um nó fonte (source) injetando uma unidade de corrente na rede ($b(i) = 1$). Para lidar com o fato de que as subestações reais não demandam
uma quantidade fixa de energia, mas sim possuem um limite máximo de recebimento, o modelo introduz o conceito de uma 'super-subestação' global (sink). Todas as subestações
físicas são conectadas a esse sumidouro artificial através de arestas de custo zero, cujas capacidades são equivalentes às restrições térmicas das subestações físicas reais.

A grande distinção desta modelagem para o MCF clássico reside na função de custo. Enquanto no MCF tradicional o custo cresce linearmente em relação à quantidade de carga, no
WFCP a função de custo assume um formato em degraus não-linear (step function). Isso significa que enviar uma unidade de fluxo extra por um arco só incorre em custo adicional
caso a capacidade do cabo atual seja excedida, exigindo um upgrade para uma bitola maior. Caso contrário, a inserção de fluxo adicional é 'gratuita'. O foco matemático
desta abstração recai sobre as restrições de fluxo em rede: garantir que as equações de conservação de massa sejam respeitadas em cada nó e que a rede residual consiga computar
dinamicamente os ganhos e penalidades financeiras à medida que novas conexões são estabelecidas.

É importante notar que, ao adotar a abstração de rede de fluxo, o modelo de Jenne restringe-se às restrições
de natureza fluídica:

- Conservação de fluxo em cada nó (balanceamento);
- Limite de recebimento de energia na subestação;
- Ausência de fluxo de saída da subestação.

Restrições geométricas e combinatórias do WFCP — como a proibição de cruzamento de cabos, a seleção discreta de bitolas
e a necessidade de desviar de obstáculos — são deixadas de fora da formulação de Jenne.

