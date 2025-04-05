package Grafo;

import java.util.*;

public class GeradorDeGrafos {

    public static Grafo gerarGrafoAleatorioConexo(int numeroDeVertices, Integer numeroDeArestas) {
        Grafo grafo = new Grafo();

        // Adiciona todos os vértices ao grafo
        for (int i = 0; i < numeroDeVertices; i++) {
            grafo.adicionarVertice(i);
        }

        // Número máximo de arestas possíveis
        int maxArestasPossiveis = (numeroDeVertices * (numeroDeVertices - 1)) / 2;


        // Pega 10x o número minimo de arestas para ligar o grafo
        if (numeroDeArestas == null) {
            Random rand = new Random();
            int minAresta = numeroDeVertices - 1;
            numeroDeArestas = rand.nextInt(minAresta * 10 - minAresta) + minAresta;
        }

        // Número mínimo de arestas possíveis
        numeroDeArestas = Math.min(numeroDeArestas, maxArestasPossiveis);
        System.out.println("Gerando grafo com " + numeroDeVertices + " vértices e " + numeroDeArestas + " arestas");
        System.out.println("Densidade: " + ((double) numeroDeArestas / (numeroDeVertices * (numeroDeVertices - 1))));

        Set<String> arestasAdicionadas = new HashSet<>();
        Random rand = new Random();

        List<Integer> vertices = new ArrayList<>();
        for (int v = 0; v < numeroDeVertices; v++) {
            vertices.add(v);
        }

        Collections.shuffle(vertices, rand);

        for (int v = 1; v < vertices.size(); v++) {
            adicionarArestaConexa(grafo, arestasAdicionadas, vertices.get(v), vertices.get(rand.nextInt(v)));
        }

        while (arestasAdicionadas.size() < numeroDeArestas) {
            int vertice1 = rand.nextInt(numeroDeVertices);
            int vertice2 = rand.nextInt(numeroDeVertices);

            if (vertice1 != vertice2) {
                adicionarArestaConexa(grafo, arestasAdicionadas, vertice1, vertice2);
            }
        }

        return grafo;
    }

    private static void adicionarArestaConexa(Grafo grafo, Set<String> arestasAdicionadas, int vertice1, int vertice2) {
        int menor = Math.min(vertice1, vertice2);
        int maior = Math.max(vertice1, vertice2);
        String chaveAresta = menor + " " + maior;
        
        if (arestasAdicionadas.add(chaveAresta)) {
            grafo.adicionarAresta(menor, maior);
        }
    }
}
