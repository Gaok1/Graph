import java.util.*;

public class GeradorDeGrafos {

    public static Grafo gerarGrafoAleatorio(int numeroDeVertices, Integer numeroDeArestas) {
        Grafo grafo = new Grafo();

        for (int i = 0; i < numeroDeVertices; i++) {
            grafo.adicionarVertice(i);
        }

        int maxArestasPossiveis = (numeroDeVertices * (numeroDeVertices - 1)) / 2;

        if (numeroDeArestas == null) {
            Random rand = new Random();
            int minAresta = numeroDeVertices - 1;
            numeroDeArestas = rand.nextInt(minAresta*10 - minAresta) + minAresta;
        }

        numeroDeArestas = Math.min(numeroDeArestas, maxArestasPossiveis);
        System.out.println("Gerando grafo com " + numeroDeVertices + " vértices e " + numeroDeArestas + " arestas");
        //densidade de vertice para aresta 
        System.out.println("Densidade: " + (double) numeroDeArestas / (numeroDeVertices * (numeroDeVertices - 1)));

        // Usar um Set para garantir que não há duplicatas
        Set<String> arestasAdicionadas = new HashSet<>();
        Random rand = new Random();

        int count = 0;
        while (count < numeroDeArestas) {
            int vertice1 = rand.nextInt(numeroDeVertices);
            int vertice2 = rand.nextInt(numeroDeVertices);
            // Evitar laços e duplicatas
            if (vertice1 != vertice2) {
                int menor = Math.min(vertice1, vertice2);
                int maior = Math.max(vertice1, vertice2);
                String chaveAresta = menor + " " + maior;

                // Apenas adicionar a aresta se ainda não foi adicionada
                if (arestasAdicionadas.add(chaveAresta)) {
                    grafo.adicionarAresta(menor, maior);
                    count++;
                }
            }
        }

        return grafo;
    }

    public static void main(String[] args) {
        long start = System.currentTimeMillis();
        Grafo grafo = gerarGrafoAleatorio(100000, null);
        long tempoFinal = System.currentTimeMillis() - start;
        System.out.println("grafo completo !! ebaaaa\nPronto em  " + tempoFinal / 1000 + "s");
    }
}
