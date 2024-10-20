import java.io.BufferedReader;
import java.io.FileReader;
import java.io.IOException;
import java.util.*;

public class DiGraph {
    private int verticesLen;
    private int edgesLen;
    private final Map<Integer, Vertex> vertices;

    // Constantes para geração aleatória de grafos
    private static final int MAX_EDGES_MULTIPLIER = 20;
    private static int MAX_EDGE_WEIGHT = 40;

    // Construtor para criar um grafo vazio
    public DiGraph() {
        this.verticesLen = 0;
        this.edgesLen = 0;
        this.vertices = new HashMap<>();
    }

    // Construtor para criar um grafo com capacidade inicial
    public DiGraph(int vertexNum) {
        this.verticesLen = 0;
        this.edgesLen = 0;
        this.vertices = new HashMap<>(vertexNum);
    }

    // Cria um grafo a partir de uma lista de arestas
    public static DiGraph fromEdges(List<Edge> edgeList) {
        DiGraph graph = new DiGraph();

        for (Edge edge : edgeList) {
            int origin = edge.getOriginKey();
            int destiny = edge.getDestinyKey();
            graph.addEdge(origin, destiny, edge.getWeight());
        }

        return graph;
    }

    /**
     * Cria um grafo direcionado a partir de um arquivo.
     *
     * O arquivo deve estar no formato:
     * ```
     * <número_de_vértices> <número_de_arestas>
     * <origem1> <destino1>
     * <origem2> <destino2>
     * ...
     * ```
     *
     * @param filePath Caminho para o arquivo de entrada.
     * @return Uma instância de DiGraph representando o grafo, ou null se ocorrer um
     *         erro.
     */
    public static DiGraph fromFile(String filePath) {
        DiGraph graph = new DiGraph();

        try (BufferedReader br = new BufferedReader(new FileReader(filePath))) {
            String line = br.readLine();
            if (line == null) {
                System.err.println("Arquivo vazio ou formato inválido.");
                return null;
            }

            String[] firstLineParts = line.trim().split("\\s+");
            if (firstLineParts.length < 2) {
                System.err.println("Primeira linha do arquivo deve conter o número de vértices e arestas.");
                return null;
            }

            int vertNum;
            int edgeNum;
            try {
                vertNum = Integer.parseInt(firstLineParts[0]);
                edgeNum = Integer.parseInt(firstLineParts[1]);
            } catch (NumberFormatException e) {
                System.err.println("Erro de leitura: Os valores de número de vértices e arestas devem ser inteiros.");
                return null;
            }

            // Opcional: Validar se o número de vértices está correto
            // Neste exemplo, apenas adicionamos arestas e vértices conforme necessário

            int currentEdge = 0;
            while ((line = br.readLine()) != null && currentEdge < edgeNum) {
                line = line.trim();
                if (line.isEmpty())
                    continue; // Ignora linhas vazias

                String[] parts = line.split("\\s+");
                if (parts.length < 2) {
                    System.err.println("Formato inválido na linha " + (currentEdge + 2) + ": \"" + line + "\"");
                    return null;
                }

                int origin;
                int destiny;
                try {
                    origin = Integer.parseInt(parts[0]);
                    destiny = Integer.parseInt(parts[1]);
                } catch (NumberFormatException e) {
                    System.err.println(
                            "Erro de leitura: Origem e destino devem ser inteiros na linha " + (currentEdge + 2));
                    return null;
                }

                
                graph.addEdge(origin, destiny, 1);
                currentEdge++;
            }

            // Verifica se o número de arestas lidas corresponde ao declarado
            if (currentEdge < edgeNum) {
                System.err.println("Atenção: Número de arestas lidas (" + currentEdge + ") é menor que o declarado ("
                        + edgeNum + ").");
            } else if (currentEdge > edgeNum) {
                System.err.println("Atenção: Número de arestas lidas (" + currentEdge + ") é maior que o declarado ("
                        + edgeNum + ").");
            }

        } catch (IOException e) {
            switch (e.getClass().getSimpleName()) {
                case "FileNotFoundException":
                    System.err.println("Arquivo não encontrado!");
                    break;
                case "AccessDeniedException":
                    System.err.println("Acesso ao arquivo foi negado!");
                    break;
                default:
                    System.err.println("Um erro inesperado aconteceu: " + e.getMessage());
            }
            return null;
        }

        return graph;
    }

    // Retorna o número de vértices
    public int getVerticesLength() {
        return verticesLen;
    }

    // Retorna o número de arestas
    public int getEdgesLength() {
        return edgesLen;
    }

    // Retorna uma lista das chaves dos vértices
    public List<Integer> getVertexKeyList() {
        return new ArrayList<>(vertices.keySet());
    }

    // Retorna o vértice com a chave especificada
    public Vertex getVertex(int key) {
        return vertices.get(key);
    }

    // Retorna todas as arestas do grafo
    public List<Edge> getAllEdges() {
        List<Edge> allEdges = new ArrayList<>(edgesLen);
        for (Vertex vertex : vertices.values()) {
            allEdges.addAll(vertex.getAllEdges());
        }
        return allEdges;
    }

    // Remove uma aresta do grafo
    public void removeEdge(Edge edge) {
        Vertex originVertex = vertices.get(edge.getOriginKey());
        if (originVertex != null) {
            originVertex.removeEdge(edge);
            edgesLen--;
        }
    }

    // Verifica se um vértice existe no grafo
    public boolean vertexExists(int key) {
        return vertices.containsKey(key);
    }

    // Adiciona um vértice ao grafo
    public void addVertex(int key) {
        if (!vertices.containsKey(key)) {
            vertices.put(key, new Vertex(key));
            verticesLen++;
        }
    }

    // Adiciona uma aresta ao grafo (ponderada)
    public void addEdge(int origin, int destiny, int weight) {
        addVertex(origin);
        addVertex(destiny);

        Edge edge = new Edge(origin, destiny, weight);
        Vertex originVertex = vertices.get(origin);
        if (originVertex != null) {
            originVertex.addEdge(edge);
            edgesLen++;
        }
    }

    // Retorna as arestas de um vértice específico
    public List<Edge> getEdgesOf(int key) {
        Vertex vertex = vertices.get(key);
        if (vertex == null)
            return Collections.emptyList();
        return vertex.getAllEdges();
    }

    /**
     * Encontra o caminho entre dois vértices usando Busca em Profundidade (DFS).
     *
     * @param startKey O vértice de início.
     * @param endKey   O vértice de destino.
     * @return Uma lista de arestas representando o caminho, ou null se não houver
     *         caminho.
     */
    public List<Edge> pathBetween(int startKey, int endKey) {
        Map<Integer, Edge> parentMap = new HashMap<>();
        Set<Integer> visited = new HashSet<>();
        Deque<Integer> stack = new ArrayDeque<>();
        stack.push(startKey);
        visited.add(startKey);

        while (!stack.isEmpty()) {
            int current = stack.pop();
            if (current == endKey) {
                break;
            }

            Vertex vertex = vertices.get(current);
            if (vertex == null)
                continue;

            for (Edge edge : vertex.getAllEdges()) {
                int neighbor = edge.getDestinyKey();
                if (!visited.contains(neighbor)) {
                    visited.add(neighbor);
                    parentMap.put(neighbor, edge);
                    stack.push(neighbor);
                }
            }
        }

        if (!parentMap.containsKey(endKey)) {
            return null; // Não há caminho
        }

        // Reconstrução do caminho
        List<Edge> path = new ArrayList<>();
        int current = endKey;
        while (current != startKey) {
            Edge edge = parentMap.get(current);
            if (edge == null) {
                return null; // Caminho inválido
            }
            path.add(edge);
            current = edge.getOriginKey();
        }

        Collections.reverse(path);
        return path;
    }

    // Gera um grafo aleatório
    public static DiGraph fromRandom(int verticesLen, Integer edgesLenOpt, boolean weighted, boolean negativeWeight) {
        Random rng = new Random();
        int minEdges = Math.max(verticesLen - 1, 0);
        int randomEdgesLen = rng.nextInt(verticesLen * MAX_EDGES_MULTIPLIER + 1);
        int edgesLen = edgesLenOpt != null ? Math.max(edgesLenOpt, minEdges) : Math.max(randomEdgesLen, minEdges);
        long maxEdges = (long) verticesLen * ((long) verticesLen - 1);
        edgesLen = (int) Math.min(edgesLen, maxEdges);

        DiGraph graph = new DiGraph();
        for (int i = 0; i < verticesLen; i++) {
            graph.addVertex(i);
        }

        Set<String> edgesAdded = new HashSet<>();

        // Adiciona arestas para garantir conectividade mínima
        for (int i = 1; i < verticesLen; i++) {
            int origin = i;
            int destiny = rng.nextInt(i);
            String edgeKey = origin + "->" + destiny;
            if (!edgesAdded.contains(edgeKey)) {
                int weight = 1;
                if (weighted) {
                    weight = rng.nextInt(MAX_EDGE_WEIGHT * 2) - MAX_EDGE_WEIGHT;
                    if (!negativeWeight && weight < 0) {
                        weight = -weight;
                    }
                }
                graph.addEdge(origin, destiny, weighted ? weight : 1);
                edgesAdded.add(edgeKey);
            }
        }

        // Adiciona arestas restantes
        while (graph.getEdgesLength() < edgesLen) {
            int origin = rng.nextInt(verticesLen);
            int destiny = rng.nextInt(verticesLen);
            if (origin == destiny)
                continue;

            String edgeKey = origin + "->" + destiny;
            if (edgesAdded.contains(edgeKey))
                continue;

            int weight = 1;
            if (weighted) {
                weight = rng.nextInt(MAX_EDGE_WEIGHT * 2) - MAX_EDGE_WEIGHT;
                if (!negativeWeight && weight < 0) {
                    weight = -weight;
                }
            }

            graph.addEdge(origin, destiny, weighted ? weight : 1);
            edgesAdded.add(edgeKey);
        }

        System.out.println("Grafo gerado com " + verticesLen + " vértices e " + graph.getEdgesLength() + " arestas.");
        return graph;
    }

    /**
     * Cria um grafo em malha (grid graph) com as dimensões especificadas.
     *
     * Cada vértice é conectado ao seu vizinho da direita e ao vizinho de baixo, se
     * existirem.
     *
     * @param height A altura da grade (número de linhas).
     * @param width  A largura da grade (número de colunas).
     * @return Uma instância de DiGraph representando o grafo em grade.
     */
    public static DiGraph newGrid(int height, int width) {
        int totalVertices = height * width;
        DiGraph digraph = new DiGraph(totalVertices);

        for (int row = 0; row < height; row++) {
            for (int col = 0; col < width; col++) {
                int v = row * width + col;

                // Conectar com o vizinho da direita
                if (col + 1 < width) {
                    int rightNeighbor = v + 1;
                    digraph.addEdge(v, rightNeighbor, 1); // Peso 1
                }

                // Conectar com o vizinho de baixo
                if (row + 1 < height) {
                    int bottomNeighbor = v + width;
                    digraph.addEdge(v, bottomNeighbor, 1); // Peso 1
                }
            }
        }

        return digraph;
    }

}
