import java.util.*;

public class MaxFlow {

    /**
     * Calcula o fluxo máximo usando o algoritmo de Ford-Fulkerson.
     *
     * @param graph O grafo direcionado.
     * @param s     A fonte do fluxo.
     * @param t     O terminal do fluxo.
     * @return Um par contendo o FluxMap com os fluxos das arestas e o ResidualGraph final.
     */
    public static Pair<FluxMap, ResidualGraph> maxFlux(DiGraph graph, int s, int t) {
        // Inicializa o mapa de fluxo
        FluxMap fluxMap = new FluxMap(graph.getAllEdges(), new Pair<>(s, t));

        // Cria o grafo residual inicial
        ResidualGraph residualGraph = new ResidualGraph(graph, fluxMap);

        // Encontra caminhos aumentantes e atualiza o fluxo
        IncreasingPath increasingPath = IncreasingPath.fromResidualGraph(residualGraph, s, t);

        while (increasingPath != null) {
            int bottleneck = increasingPath.getBottleneck();
            List<Edge> pathEdges = increasingPath.getEdges();

            // Atualiza os fluxos no FluxMap
            for (Edge e : pathEdges) {
                int v = e.getOriginKey();
                int w = e.getDestinyKey();
                boolean isInverted = residualGraph.isEdgeInverted(e);

                Pair<Integer, Integer> key = isInverted ? new Pair<>(w, v) : new Pair<>(v, w);
                EdgeAtt att = fluxMap.get(key);

                if (att == null) {
                    throw new IllegalStateException("Aresta não encontrada no FluxMap: " + key);
                }

                if (isInverted) {
                    // Reduz o fluxo na aresta original
                    int newFlux = att.getFlux() - bottleneck;
                    if (newFlux < 0) {
                        throw new IllegalArgumentException("Fluxo reduzido abaixo de zero para a aresta " + key);
                    }
                    att.setFlux(newFlux);
                } else {
                    // Aumenta o fluxo na aresta original
                    int newFlux = att.getFlux() + bottleneck;
                    if (newFlux > att.getCapacity()) {
                        throw new IllegalArgumentException("Fluxo excede a capacidade para a aresta " + key);
                    }
                    att.setFlux(newFlux);
                }
            }

            // Atualiza o grafo residual com os novos fluxos
            residualGraph = new ResidualGraph(graph, fluxMap);

            // Procura por um novo caminho aumentante
            increasingPath = IncreasingPath.fromResidualGraph(residualGraph, s, t);
        }

        // Calcula o fluxo máximo a partir das arestas saindo da fonte
        int maxFluxValue = 0;
        for (Edge e : graph.getEdgesOf(s)) {
            Pair<Integer, Integer> key = new Pair<>(e.getOriginKey(), e.getDestinyKey());
            EdgeAtt att = fluxMap.get(key);
            if (att != null) {
                maxFluxValue += att.getFlux();
            }
        }
        fluxMap.setMaxFlux(maxFluxValue);

        return new Pair<>(fluxMap, residualGraph);
    }
}
