= Descrição do Problema

O Wind Farm Cabling Problem (WFCP) surge da necessidade de interligar as turbinas de um parque gerador a uma subestação de coleta, minimizando os custos totais de instalação.
Como a infraestrutura e os materiais envolvidos na implementação de parques eólicos são extremamente caros, a otimização da topologia da rede e a alocação correta dos recursos
representam uma economia financeira substancial no orçamento final do projeto.

No WFCP base, os cabos disponíveis para a conexão das turbinas são produzidos e comercializados em um número limitado de especificações, possuindo capacidades máximas de transmissão de energia fixas. O custo de aquisição e instalação de cada cabo cresce de acordo com a sua capacidade de transmissão. Dessa forma, o dimensionamento inadequado — como a escolha de cabos mais robustos e caros do que o necessário para um trecho específico — gera desperdício financeiro direto. O objetivo final é decidir, de forma ótima, quais cabos conectarão quais turbinas até a subestação.

Em linhas gerais, considera-se que todas as turbinas operam com potência idêntica, sendo a geração de cada unidade normalizada para o valor 1. Sob essa premissa, a capacidade
de transmissão de um determinado cabo passa a ser definida por um limite N, indicando que aquele condutor suporta a carga combinada de, no máximo, N turbinas. Além das limitações
dos cabos, o problema também considera uma restrição estrutural na subestação, que possui um limite superior (upper bound) de K portas disponíveis para o recebimento de conexões diretas.

Por fim, a construção real impõe restrições físicas e topológicas. A principal delas é a restrição de planaridade, ou seja, a proibição do cruzamento de cabos no terreno.
Em ambientes reais, transpassar um cabo por cima de outro exige isolamentos e estruturas complexas, elevando drasticamente o custo da operação em comparação com o roteamento
simples de um ponto a outro.

== Variações do Problema

A formulação base do WFCP atende às premissas iniciais de viabilidade, no entanto, a literatura apresenta diversas variações que buscam aproximar a modelagem das complexidades encontradas no mundo real. As principais extensões incluem:

1. *Múltiplas Subestações:* A formulação base pode ser estendida para considerar o roteamento direcionado a mais de uma subestação coletora. Essa variação adiciona ao modelo a complexidade de decidir previamente ou simultaneamente qual turbina deve ser alocada a qual subestação, com o objetivo de balancear a carga da rede e minimizar o custo total de cabeamento.

2. *Perdas de Energia:* Além do custo de capital da instalação, algumas modelagens incorporam a perda de potência dissipada na forma de calor ao longo dos cabos durante toda a vida útil do parque. Conforme demonstrado em @fischetti2018, essa variação pode justificar a escolha de um cabo mais caro e de maior capacidade de passagem de energia no presente para garantir eficiência energética e maior retorno financeiro a longo prazo.

3. *Desvio de Obstáculos:* Considera a presença de zonas restritas no terreno, como reservas naturais, outros parques eólicos ou cabos já existentes e irregularidades severas no terreno. Modelagens mais completas, como a proposta em @fischetti2018, exigem que o roteamento desvie dessas áreas, frequentemente utilizando a inserção de nós de ramificação artificiais (Steiner points) para encontrar caminhos viáveis.
