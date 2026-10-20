= Estado da Arte

O estado da arte para a resolução do WFCP é apresentado em @cazzaro2020, que apresenta um estudo comparativo abrangente entre diversas abordagens
de otimização. O trabalho introduz a heurística denominada Sweeping Multistart e avalia seu desempenho frente a seis meta-heurísticas, explorando
tanto os resultados isolados de cada algoritmo quanto os benefícios de estratégias híbridas.

De acordo com o estudo, a implementação de maior sucesso consiste em uma hibridização: uma solução inicial de alta qualidade é gerada rapidamente pela heurística
Sweeping Multistart e, em seguida, essa solução é fornecida como entrada para ser refinada pela meta-heurística Variable Neighborhood Search (VNS).

Essa estratégia conjunta superou consistentemente os demais algoritmos testados. Em termos de qualidade de solução, a hibridização alcançou resultados notáveis, gerando
topologias que atingem o ótimo global ou apresentam desvios marginais nas instâncias em que o modelo matemático exato conseguiu finalizar dentro do tempo limite estipulado,
validando assim a sua alta eficácia.

Como base de comparação estrutural (baseline) para essas heurísticas, utiliza-se o modelo matemático exato descrito por @fischetti2018. A resolução dessa formulação
através de solvers de Programação Linear Inteira Mista (MILP) garante a obtenção da solução ótima global. Contudo, devido à complexidade NP-difícil intrínseca ao WFCP,
a aplicação bem-sucedida de métodos puramente exatos restringe-se, na prática, a instâncias de pequeno porte.

