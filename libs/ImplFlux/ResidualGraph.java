import java.util.*;

public class ResidualGraph {
    private final DiGraph graph; // Grafo residual
    private final Map<Pair<Integer, Integer>, Boolean> edgeInverted; // (v,w) -> true se invertida

    // Construtor a partir do grafo original e do FluxMap
    public ResidualGraph(DiGraph originalGraph, FluxMap fluxMap) {
        this.edgeInverted = new HashMap<>();
        List<Edge> residualEdges = new ArrayList<>();

        for (Edge e : originalGraph.getAllEdges()) {
            Pair<Integer, Integer> key = new Pair<>(e.getOriginKey(), e.getDestinyKey());
            EdgeAtt att = fluxMap.get(key);
            if (att == null) {
                throw new IllegalArgumentException("Aresta (" + key.first + ", " + key.second + ") não encontrada no FluxMap");
            }
            int flux = att.getFlux();
            int capacity = att.getCapacity();

            if (flux > 0) {
                // Aresta invertida
                Edge invertedEdge = new Edge(e.getDestinyKey(), e.getOriginKey(), flux);
                residualEdges.add(invertedEdge);
                edgeInverted.put(new Pair<>(invertedEdge.getOriginKey(), invertedEdge.getDestinyKey()), true);
            }

            if (capacity > flux) {
                // Aresta residual
                Edge residualEdge = new Edge(e.getOriginKey(), e.getDestinyKey(), capacity - flux);
                residualEdges.add(residualEdge);
                edgeInverted.put(new Pair<>(residualEdge.getOriginKey(), residualEdge.getDestinyKey()), false);
            }
        }

        this.graph = DiGraph.fromEdges(residualEdges);
    }

    // Obtém o grafo residual
    public DiGraph getGraph() {
        return graph;
    }

    // Verifica se uma aresta está invertida
    public boolean isEdgeInverted(Edge e) {
        Pair<Integer, Integer> key = new Pair<>(e.getOriginKey(), e.getDestinyKey());
        return edgeInverted.getOrDefault(key, false);
    }

    // Representação textual do grafo residual
    public String toGraphPainterRepresentation() {
        StringBuilder sb = new StringBuilder();
        for (Edge e : graph.getAllEdges()) {
            Pair<Integer, Integer> key = new Pair<>(e.getOriginKey(), e.getDestinyKey());
            boolean isInverted = edgeInverted.getOrDefault(key, false);
            sb.append(e.getOriginKey())
              .append(" -> ")
              .append(e.getDestinyKey())
              .append(" | ");
            if (isInverted) {
                sb.append("Invertida | ");
            } else {
                sb.append("Residual | ");
            }
            sb.append("Capacidade: ")
              .append(e.getWeight())
              .append("\n");
        }
        return sb.toString();
    }
}
