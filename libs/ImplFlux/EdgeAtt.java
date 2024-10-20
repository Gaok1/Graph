public class EdgeAtt {
    private int flux;
    private final int capacity;

    // Construtor a partir de uma aresta
    public EdgeAtt(Edge e) {
        if (e.getWeight() < 0) {
            throw new IllegalArgumentException("Peso da aresta é negativo");
        }
        this.flux = 0;
        this.capacity = e.getWeight();
    }

    // Define o fluxo da aresta
    public void setFlux(int flux) {
        if (flux > capacity) {
            throw new IllegalArgumentException("Fluxo excede a capacidade");
        }
        this.flux = flux;
    }

    // Obtém o fluxo da aresta
    public int getFlux() {
        return flux;
    }

    // Obtém a capacidade da aresta
    public int getCapacity() {
        return capacity;
    }

    // Retorna uma tupla (fluxo, capacidade)
    public Pair<Integer, Integer> tuple() {
        return new Pair<>(flux, capacity);
    }

    @Override
    public String toString() {
        return "Fluxo: " + flux + ", Capacidade: " + capacity;
    }
}
