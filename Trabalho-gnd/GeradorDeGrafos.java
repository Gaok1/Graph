import java.util.*;

public class GeradorDeGrafos {
    public static Grafo gerarGrafoAleatorio(int numeroDeVertices, Integer numeroDeArestas) {
        Grafo grafo = new Grafo();

        // Adiciona os vértices ao grafo
        for (int i = 0; i < numeroDeVertices; i++) {
            grafo.adicionarVertice(i);
        }

        // Calcula o número máximo de arestas possíveis
        int maxArestasPossiveis = (numeroDeVertices * (numeroDeVertices - 1)) / 2;

        // Se o número de arestas não for especificado, gera um aleatório
        if (numeroDeArestas == null) {
            Random rand = new Random();
            numeroDeArestas = rand.nextInt(maxArestasPossiveis + 1);
        }

        // Garante que o número de arestas não exceda o máximo possível
        numeroDeArestas = Math.min(numeroDeArestas, maxArestasPossiveis);

        Random rand = new Random();
        Set<String> arestasAdicionadas = new HashSet<>();

        while (arestasAdicionadas.size() < numeroDeArestas) { 
            int vertice1 = rand.nextInt(numeroDeVertices);
            int vertice2 = rand.nextInt(numeroDeVertices);

            // Evita laços (aresta de um vértice para ele mesmo)
            if (vertice1 == vertice2) {
                continue;
            }

            // Cria uma representação única da aresta
            int menor = Math.min(vertice1, vertice2);
            int maior = Math.max(vertice1, vertice2);
            String chaveAresta = menor + " " + maior;

            // Verifica se a aresta já foi adicionada
            if (!arestasAdicionadas.contains(chaveAresta)) {
                arestasAdicionadas.add(chaveAresta);
                grafo.adicionarAresta(menor, maior);
            }
        }

        return grafo;
    }
}
