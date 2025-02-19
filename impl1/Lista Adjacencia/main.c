#include "graph.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main() {
    // Abre o arquivo de grafo
    FILE *graph_file = fopen("graph.txt", "r");
    if (graph_file == NULL) {
        printf("Error opening graph file!\n");
        return 1;
    }

    // Cria o grafo a partir do arquivo
    Graph *graph = graph_from_file(graph_file);
    fclose(graph_file);  // Fecha o arquivo do grafo após o uso

    // Abre o arquivo para escrever os caminhos
    FILE *file = fopen("paths.txt", "w");
    if (file == NULL) {
        printf("Error opening output file!\n");
        return 1;
    }

    char *line = (char *)calloc(200, sizeof(char));  // Aloca memória para a linha
    if (line == NULL) {
        printf("Error allocating memory!\n");
        fclose(file);
        return 1;
    }

    int *path;
    int origin = 1;
    int destination = 27;

    // Busca em largura (Breadth First Search)
    path = graph->breadth_first_search(graph, origin, destination);
    if (path != NULL) {
        snprintf(line, 200, "Breadth First Search: ");
        for (int i = 0; path[i] != -1; i++) {
            snprintf(line + strlen(line), 200 - strlen(line), "%d ", path[i]);
        }
        snprintf(line + strlen(line), 200 - strlen(line), "\n");
        fprintf(file, "%s", line);
        free(path);  // Libera a memória do caminho
    } else {
        fprintf(file, "Breadth First Search: No path found\n");
    }

    // Busca em profundidade (Depth First Search)
    path = graph->deep_first_search(graph, origin, destination);
    if (path != NULL) {
        snprintf(line, 200, "Depth First Search: ");
        for (int i = 0; path[i] != -1; i++) {
            snprintf(line + strlen(line), 200 - strlen(line), "%d ", path[i]);
        }
        snprintf(line + strlen(line), 200 - strlen(line), "\n");
        fprintf(file, "%s", line);
        free(path);  // Libera a memória do caminho
    } else {
        fprintf(file, "Depth First Search: No path found\n");
    }

    // Algoritmo de Dijkstra
    path = graph->shortest_path(graph, origin, destination);
    if (path != NULL) {
        snprintf(line, 200, "Dijkstra: ");
        for (int i = 0; path[i] != -1; i++) {
            snprintf(line + strlen(line), 200 - strlen(line), "%d ", path[i]);
        }
        snprintf(line + strlen(line), 200 - strlen(line), "\n");
        fprintf(file, "%s", line);
        free(path);  // Libera a memória do caminho
    } else {
        fprintf(file, "Dijkstra: No path found\n");
    }

    // Libera recursos e fecha arquivos
    free(line);  // Libera a memória alocada para a linha
    fclose(file);  // Fecha o arquivo de saída

    return 0;  // Indica que o programa foi executado com sucesso
}
