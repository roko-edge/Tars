# Tars — LinkedIn launch

## Post em inglês

We’re building a neural-network library from scratch in Rust.

Meet Tars.

Our goal is to understand every step of learning: how inputs move through a network, how a loss becomes a gradient, and how that gradient changes the weights.

So far, we’ve implemented dense layers, ReLU and sigmoid activations, mean squared error, and gradient-based training. The repository includes an XOR experiment and, separately, a SystemVerilog dot-product prototype for exploring the hardware side.

Building these pieces ourselves makes the math concrete. Every layer, derivative and weight update becomes something we can inspect, question and improve.

Tars is still experimental. We’re sharing it as a learning and research project, with plenty left to build.

If you work with Rust, machine learning or digital hardware, we’d love your technical feedback.

What concept made neural networks finally click for you?

Explore the project: https://github.com/roko-edge/Tars

#Rust #MachineLearning #BuildInPublic

## Peça visual

- [Vídeo revisado em MP4](https://d2ol7oe51mr4n9.cloudfront.net/user_3JFgQ1PJ4OR1FHNig8XKVIVCbow/7a6fcf64-6020-43d9-a49d-c72a1b048c6b.mp4)
- [Capa revisada em PNG](https://d2ol7oe51mr4n9.cloudfront.net/user_3JFgQ1PJ4OR1FHNig8XKVIVCbow/12173dac-8a6a-4e87-84d9-cb0d2f2a9f4a.png)
- [Storyboard extraído do MP4 revisado](https://d2ol7oe51mr4n9.cloudfront.net/user_3JFgQ1PJ4OR1FHNig8XKVIVCbow/433d9003-3efe-41b3-a00c-4f775bed60d7.png)
- [Caminhada em câmera lenta, a um terço da velocidade](https://d2ol7oe51mr4n9.cloudfront.net/user_3JFgQ1PJ4OR1FHNig8XKVIVCbow/d1269af9-bb09-488c-bd0b-c92f3d0590f9.mp4)

Animação de 9 segundos, 1080 × 1350 (4:5), 30 fps, H.264 em MP4. Funciona sem áudio. Fundo branco, texto em carvão, logo original e bastante espaço para leitura.

| Tempo | Conteúdo |
|---|---|
| 0–3,2 s | “Neural networks. Built from scratch.” aparece desde o primeiro quadro. O robô entra pela esquerda com coxas, joelhos e pés articulados, alternando apoio e avanço. Os braços acompanham os passos. |
| 3,2–3,85 s | O robô termina numa pose estável no centro. A palavra original “Tars” surge abaixo. |
| 4,1–4,75 s | Aparecem “Dense layers · Activations · Gradient descent” e o endereço do GitHub. |
| 4,75–9 s | A composição final permanece para leitura. |

O arquivo original do logo foi preservado. Para a animação, braços e pernas foram reconstruídos com formas vetoriais; a cabeça, o tronco e a palavra “Tars” usam o PNG original. A composição foi montada com as skills Higgsfield [video-editing](/home/arthur/.codex/plugins/cache/openai-curated-remote/app-6a3293e129088191abf0875820e839da/2.1.0/skills/video-editing/SKILL.md) e [motion-craft](/home/arthur/.codex/plugins/cache/openai-curated-remote/app-6a3293e129088191abf0875820e839da/2.1.0/skills/motion-craft/SKILL.md).

Os links acima correspondem à revisão com a caminhada articulada. O PNG local `docs/assets/linkedin/tars-cover.png` pertence ao primeiro rascunho; use a capa revisada disponibilizada no link acima.

### Verificação visual da caminhada

Foram inspecionados todos os 99 quadros entre 0 e 3,267 segundos, extraídos do MP4, com recortes das pernas em escala de 100%. Outros 18 quadros ampliados mostram o corpo inteiro durante a caminhada e a parada. O storyboard contém 12 momentos do vídeo completo, incluindo entrada, aparecimento da marca e composição final.

- Quadros consecutivos das pernas: [0–32](https://d2ol7oe51mr4n9.cloudfront.net/user_3JFgQ1PJ4OR1FHNig8XKVIVCbow/5114a19d-304d-4a45-9542-134fc4f87420.png), [33–65](https://d2ol7oe51mr4n9.cloudfront.net/user_3JFgQ1PJ4OR1FHNig8XKVIVCbow/c2922aee-be14-4ecc-a09a-f2ef61b18b53.png) e [66–98](https://d2ol7oe51mr4n9.cloudfront.net/user_3JFgQ1PJ4OR1FHNig8XKVIVCbow/7c8c95c7-d60a-4105-a6e2-967b71922df3.png).
- [Corpo inteiro durante os passos](https://d2ol7oe51mr4n9.cloudfront.net/user_3JFgQ1PJ4OR1FHNig8XKVIVCbow/d5b104b4-3a0a-4b53-a4fd-2b9eeb09774c.png) e [parada ampliada](https://d2ol7oe51mr4n9.cloudfront.net/user_3JFgQ1PJ4OR1FHNig8XKVIVCbow/4a49f9a1-84c1-4e0f-ad88-7509063cc35a.png).
- Os recortes que produziam fragmentos nas pernas foram substituídos. A sobreposição de duas poses na parada foi removida.
- Durante o ciclo regular, o pé de apoio permanece na mesma posição horizontal enquanto o corpo avança. A verificação numérica encontrou deslocamento inferior a 0,001 pixel nas amostras de apoio; os comprimentos das duas partes de cada perna também permanecem constantes.
- Nos quadros decodificados de 1 a 2,7 segundos, a borda inferior do pé apoiado permaneceu na linha 778. A linha de chão geométrica é 779,01; a diferença corresponde ao preenchimento e à rasterização da borda. Não foi encontrada oscilação da altura de apoio nesse intervalo.
- A caminhada termina em 3,2 segundos, sem a troca de imagens sobrepostas da versão anterior. A leitura da composição final foi conferida até 8,966 segundos.

As linhas de chão e os horários aparecem apenas nas imagens de revisão.

## Publicação

Baixe o MP4 e envie-o diretamente na publicação do LinkedIn. Use o PNG como capa, onde a interface oferecer essa opção. Cole o texto acima, mantendo as quebras de parágrafo. As menções aos colaboradores ficam a seu critério, conforme combinado.

O vídeo apresenta a marca; o texto explica o trabalho realizado. Para uma publicação posterior, vale mostrar a execução real do experimento XOR, com a configuração e os resultados medidos. A peça atual não apresenta métricas, resultados de treinamento simulados ou promessas de desempenho.

Comentário opcional para abrir uma conversa técnica:

> For anyone reviewing the code: we’re especially interested in feedback on the gradient calculations and the model API.

Texto alternativo da capa, se o LinkedIn permitir:

> Tars launch artwork on a white background. The headline reads “Neural networks. Built from scratch.” A geometric walking robot sits above the Tars wordmark. Below are “Dense layers · Activations · Gradient descent” and github.com/roko-edge/Tars.

## Referências e verificação

- Recursos conferidos no código local: camadas densas, ReLU, sigmoid, MSE, gradientes, otimização e experimento XOR. O protótipo SystemVerilog é separado.
- [Repositório público](https://github.com/roko-edge/Tars).
- [Requisitos oficiais de vídeo do LinkedIn](https://www.linkedin.com/help/linkedin/answer/a548372): MP4, resolução, duração, frame rate e áreas livres nas bordas.
- [Orientações do LinkedIn para vídeos](https://www.linkedin.com/business/marketing/blog/content-marketing/13-top-tips-for-compelling-b2b-video-content-on-linkedin): contexto desde o início, leitura no celular e texto complementar.

O alcance depende da audiência e da resposta ao conteúdo; não há garantia de viralização. O material foi preparado para apresentar trabalho verificável e incentivar uma conversa técnica.
