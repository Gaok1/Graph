import java.util.concurrent.atomic.AtomicInteger;

public class Edge implements Comparable<Edge> {
    private static final AtomicInteger EDGE_COUNTER = new AtomicInteger(0);

    private final int originKey;
    private final int destinyKey;
    private int weight;
    private final int id;

    // Construtor para arestas não ponderadas
    public Edge(int originKey, int destinyKey) {
        this.originKey = originKey;
        this.destinyKey = destinyKey;
        this.weight = 1;
        this.id = EDGE_COUNTER.getAndIncrement();
    }

    // Construtor para arestas ponderadas
    public Edge(int originKey, int destinyKey, int weight) {
        this.originKey = originKey;
        this.destinyKey = destinyKey;
        this.weight = weight;
        this.id = EDGE_COUNTER.getAndIncrement();
    }

    public int getOriginKey() {
        return originKey;
    }

    public int getDestinyKey() {
        return destinyKey;
    }

    public int getWeight() {
        return weight;
    }

    public void setWeight(int weight) {
        this.weight = weight;
    }

    public int getId() {
        return id;
    }

    @Override
    public int compareTo(Edge other) {
        return Integer.compare(this.weight, other.weight);
    }

    @Override
    public String toString() {
        return originKey + " -> " + destinyKey + " w " + weight + " id " + id;
    }

    @Override
    public boolean equals(Object obj) {
        if (this == obj) return true;
        if (!(obj instanceof Edge)) return false;
        Edge other = (Edge) obj;
        return this.originKey == other.originKey &&
               this.destinyKey == other.destinyKey &&
               this.weight == other.weight &&
               this.id == other.id;
    }

    @Override
    public int hashCode() {
        int result = Integer.hashCode(originKey);
        result = 31 * result + Integer.hashCode(destinyKey);
        result = 31 * result + Integer.hashCode(weight);
        result = 31 * result + Integer.hashCode(id);
        return result;
    }
}