import java.util.*;

public class FluxMap {
    private final Map<Pair<Integer, Integer>, EdgeAtt> map; // Mapa (v, w) -> EdgeAtt
    private final Pair<Integer, Integer> s_t; // Fonte e terminal (s, t)
    private int maxFlux;

    // Construtor a partir de uma lista de arestas e da fonte e terminal
    public FluxMap(List<Edge> edges, Pair<Integer, Integer> s_t) {
        this.map = new HashMap<>();
        this.s_t = s_t;
        this.maxFlux = 0;
        for (Edge e : edges) {
            Pair<Integer, Integer> key = new Pair<>(e.getOriginKey(), e.getDestinyKey());
            map.put(key, new EdgeAtt(e));
        }
    }

    // Obtém os atributos de uma aresta
    public EdgeAtt get(Pair<Integer, Integer> key) {
        return map.get(key);
    }

    // Define o fluxo máximo
    public void setMaxFlux(int flux) {
        this.maxFlux = flux;
    }

    // Obtém o fluxo máximo
    public int getMaxFlux() {
        return maxFlux;
    }

    // Retorna as arestas utilizadas com seus respectivos fluxos
    public List<Pair<Edge, EdgeAtt>> getUsedEdges() {
        List<Pair<Edge, EdgeAtt>> usedEdges = new ArrayList<>();
        for (Map.Entry<Pair<Integer, Integer>, EdgeAtt> entry : map.entrySet()) {
            EdgeAtt att = entry.getValue();
            if (att.getFlux() > 0) {
                int v = entry.getKey().first;
                int w = entry.getKey().second;
                Edge e = new Edge(v, w, att.getCapacity());
                usedEdges.add(new Pair<>(e, att));
            }
        }
        return usedEdges;
    }
}
