
import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

// Classe Grafo
public class Grafo {
    private Map<Integer, Vertice> vertices_map;

    public Grafo() {
        vertices_map = new HashMap<>();
    }

    public void adicionarVertice(int chave) {
        if (!vertices_map.containsKey(chave)) {
            vertices_map.put(chave, new Vertice(chave));
        }
    }

    public void adicionarAresta(int chave1, int chave2) {
        Vertice vertice1 = vertices_map.get(chave1);
        Vertice vertice2 = vertices_map.get(chave2);
        if (vertice1 == null ) {
            adicionarVertice(chave1);
            vertice1 = vertices_map.get(chave1);
        }
        if (vertice2 == null ) {
            adicionarVertice(chave2);
            vertice2 = vertices_map.get(chave2);
        }
        

        Aresta aresta = new Aresta(vertice1, vertice2);

        // Adiciona a aresta em ambos os vértices
        vertice1.adicionarAresta(aresta);
        vertice2.adicionarAresta(aresta);
    }

    public Vertice getVertice(int chave) {
        return vertices_map.get(chave);
    }

    public List<Vertice> getVertices() {
        ArrayList<Vertice> vertices = new ArrayList<>();
        for (Vertice vertice : vertices_map.values()) {
            vertices.add(vertice);
        }
        return vertices;

    }

    public Aresta get_aresta_v_w(int chave1, int chave2) {
        Vertice vertice1 = vertices_map.get(chave1);
        Vertice vertice2 = vertices_map.get(chave2);
        if (vertice1 == null || vertice2 == null) {
            return null;
        }
        if (vertice1.contemArestaCom(vertice2)) { //ordem não importa
            return vertice1.getArestas().get(chave1 + " " + chave2);
        } else {
            return null;
        }
    }
}
