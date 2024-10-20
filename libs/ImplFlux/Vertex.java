import java.util.*;

public class Vertex {
    private final int key;
    // Mapeia para cada par (v, w) uma lista de arestas (permitindo arestas paralelas)
    private final Map<Pair<Integer, Integer>, List<Edge>> edges;

    public Vertex(int key) {
        this.key = key;
        this.edges = new HashMap<>();
    }

    public int getKey() {
        return key;
    }

    public Map<Pair<Integer, Integer>, List<Edge>> getEdgesMap() {
        return edges;
    }

    public List<Edge> getEdgesTo(int destinyKey) {
        Pair<Integer, Integer> pairKey = new Pair<>(this.key, destinyKey);
        return edges.getOrDefault(pairKey, Collections.emptyList());
    }

    public void addEdge(Edge edge) {
        Pair<Integer, Integer> pairKey = new Pair<>(edge.getOriginKey(), edge.getDestinyKey());
        edges.computeIfAbsent(pairKey, k -> new ArrayList<>()).add(edge);
    }

    public boolean hasEdgeTo(int destinyKey) {
        Pair<Integer, Integer> pairKey = new Pair<>(this.key, destinyKey);
        return edges.containsKey(pairKey);
    }

    public List<Edge> getAllEdges() {
        List<Edge> allEdges = new ArrayList<>();
        for (List<Edge> edgeList : edges.values()) {
            allEdges.addAll(edgeList);
        }
        return allEdges;
    }

    public void removeEdge(Edge edge) {
        Pair<Integer, Integer> pairKey = new Pair<>(edge.getOriginKey(), edge.getDestinyKey());
        List<Edge> edgeList = edges.get(pairKey);
        if (edgeList != null) {
            edgeList.removeIf(e -> e.equals(edge));
            if (edgeList.isEmpty()) {
                edges.remove(pairKey);
            }
        }
    }

    @Override
    public String toString() {
        return "Vertex{" +
                "key=" + key +
                ", edges=" + edges +
                '}';
    }
}
