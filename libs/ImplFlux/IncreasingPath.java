import java.util.*;

public class IncreasingPath {
    private final int bottleneck;
    private final List<Edge> edges;

    public IncreasingPath(int bottleneck, List<Edge> edges) {
        this.bottleneck = bottleneck;
        this.edges = edges;
    }

    /**
     * Cria um caminho aumentante a partir do grafo residual.
     *
     * @param residualGraph O grafo residual.
     * @param s             A fonte do fluxo.
     * @param t             O terminal do fluxo.
     * @return Uma instância de IncreasingPath ou null se não houver caminho.
     */
    public static IncreasingPath fromResidualGraph(ResidualGraph residualGraph, int s, int t) {
        List<Edge> path = residualGraph.getGraph().pathBetween(s, t);
        if (path == null || path.isEmpty()) {
            return null;
        }
        int bottleneck = Integer.MAX_VALUE;
        for (Edge e : path) {
            bottleneck = Math.min(bottleneck, e.getWeight());
        }
        return new IncreasingPath(bottleneck, path);
    }

    public int getBottleneck() {
        return bottleneck;
    }

    public List<Edge> getEdges() {
        return edges;
    }
}
