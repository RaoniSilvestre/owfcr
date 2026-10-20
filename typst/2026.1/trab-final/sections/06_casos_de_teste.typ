= Casos de testes encontrados na literatura

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
