import java.util.*;

public class Main {
    public static void main(String[] args) {
        Scanner scanner = new Scanner(System.in);

        System.out.print("Digite o caminho do arquivo do grafo: ");
        String filePath = scanner.nextLine();

        DiGraph graph = DiGraph.fromFile(filePath);

        if (graph == null) {
            System.err.println("Falha ao criar o grafo a partir do arquivo.");
            return;
        }

        System.out.println("\nGrafo criado com sucesso!");
        System.out.println("Número de vértices: " + graph.getVerticesLength());
        System.out.println("Número de arestas: " + graph.getEdgesLength());

        System.out.print("Digite o vértice de fonte (s): ");
        int source = readint(scanner);
        System.out.print("Digite o vértice de terminal (t): ");
        int sink = readint(scanner);

        if (!graph.vertexExists(source)) {
            System.err.println("Vértice de fonte (" + source + ") não existe no grafo.");
            return;
        }
        if (!graph.vertexExists(sink)) {
            System.err.println("Vértice de terminal (" + sink + ") não existe no grafo.");
            return;
        }

        Pair<FluxMap, ResidualGraph> result = MaxFlow.maxFlux(graph, source, sink);

        System.out.println("\nFluxo Máximo: " + result.first.getMaxFlux());

        System.out.println("\nArestas Utilizadas com seus Fluxos (Fluxo > 0):");
        List<Pair<Edge, EdgeAtt>> usedEdges = result.first.getUsedEdges();
        for (Pair<Edge, EdgeAtt> entry : usedEdges) {
            Edge e = entry.first;
            EdgeAtt att = entry.second;
            System.out.println(e.getOriginKey() + " -> " + e.getDestinyKey() +
                    " | Fluxo: " + att.getFlux() + " / Capacidade: " + att.getCapacity());
        }

        DiGraph flowGraph = createFlowGraph(usedEdges);

        System.out.println("\nCaminhos de " + source + " até " + sink + " no grafo de fluxo:");
        findAndPrintPaths(flowGraph, source, sink);
    }

    private static int readint(Scanner scanner) {
        while (true) {
            String input = scanner.nextLine();
            try {
                return Integer.parseInt(input.trim());
            } catch (NumberFormatException e) {
                System.out.print("Entrada inválida. Por favor, digite um número inteiro: ");
            }
        }
    }

    /**
     * Cria um novo grafo com as arestas utilizadas no fluxo (fluxo > 0).
     *
     * @param usedEdges A lista de pares (Edge, EdgeAtt) onde fluxo > 0.
     * @return Uma instância de DiGraph representando o grafo de fluxo.
     */
    private static DiGraph createFlowGraph(List<Pair<Edge, EdgeAtt>> usedEdges) {
        DiGraph flowGraph = new DiGraph();
        for (Pair<Edge, EdgeAtt> pair : usedEdges) {
            Edge e = pair.first;
            int flow = pair.second.getFlux();
            flowGraph.addEdge(e.getOriginKey(), e.getDestinyKey(), flow);
        }
        return flowGraph;
    }

    /**
     * Encontra e imprime todos os caminhos de s até t no grafo fornecido,
     * removendo as arestas utilizadas após cada caminho encontrado.
     *
     * @param graph O grafo onde buscar os caminhos.
     * @param s     O vértice de fonte.
     * @param t     O vértice de terminal.
     */
    private static void findAndPrintPaths(DiGraph graph, int s, int t) {
        int pathCount = 0;
        while (true) {
            List<Edge> path = graph.pathBetween(s, t);
            if (path == null || path.isEmpty()) {
                break; // Não há mais caminhos
            }

            pathCount++;
            System.out.print("Caminho " + pathCount + ": ");
            List<String> pathVertices = new ArrayList<>();
            for (Edge e : path) {
                pathVertices.add(String.valueOf(e.getOriginKey()));
            }
            // Adiciona o destino do último edge
            pathVertices.add(String.valueOf(path.get(path.size() - 1).getDestinyKey()));
            System.out.println(String.join(" -> ", pathVertices));

            // Remove as arestas do caminho do grafo
            for (Edge e : path) {
                graph.removeEdge(e);
            }
        }

        if (pathCount == 0) {
            System.out.println("Nenhum caminho encontrado de " + s + " até " + t + ".");
        } else {
            System.out.println("\nTotal de caminhos encontrados: " + pathCount);
        }
    }
}
