package Grafo;

import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

// Classe Vertice
public class Vertice {
    private int chave;
    private Map<String, Aresta> arestas;

    public Vertice(int chave) {
        this.chave = chave;
        this.arestas = new HashMap<>();
    }

    public int getChave() {
        return chave;
    }

    public Map<String, Aresta> getArestas() {
        return arestas;
    }

    public List<Aresta> getArestasList() {
        return new ArrayList<>(arestas.values());
    }

    public void adicionarAresta(Aresta aresta) {
        int key1 = this.chave;
        int key2 = aresta.get_vertice_incidente(this).getChave(); // Assumindo que Aresta tem um método getDestino()
        String hashKey = key1 + " " + key2;
        arestas.put(hashKey, aresta);
    }

    public boolean contemArestaCom(Vertice vertice) {
        int key1 = this.chave;
        int key2 = vertice.getChave();
        String hashKey = key1 + " " + key2;
        return arestas.containsKey(hashKey);
    }

    public void removerAresta(Vertice w) {
        int key1 = this.chave;
        int key2 = w.getChave();
        String hashKey = key1 + " " + key2;
        arestas.remove(hashKey);
    }

    public int getGrau() {
        return arestas.size();
    }

}
