#ifndef GRAPH_H
#define GRAPH_H

#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>
#include "priorityQueue.h"
#include "stack.h"
#include "queue.h"

/* Estrutura para a Aresta */
/**
 * @struct Edge
 * @brief Representa uma aresta no grafo.
 */
typedef struct Edge {
    /**
     * @brief Identificador único da aresta.
     */
    int id;
    /**
     * @brief Identificador do vértice de origem.
     */
    int origin_key;
    /**
     * @brief Identificador do vértice de destino.
     */
    int dest_key;
    /**
     * @brief Ponteiro para a próxima aresta na lista de adjacência.
     */
    struct Edge *next;
    /**
     * @brief Ponteiro para o vértice de destino.
     */
    struct Vertice *dest;
} Edge;

/* Contador global de arestas */
/**
 * @brief Contador global para gerar IDs únicos para as arestas.
 */
static int EDGE_COUNTER = 0;

/* Estrutura para o Vértice */
/**
 * @struct Vertice
 * @brief Representa um vértice no grafo.
 */
typedef struct Vertice {
    /**
     * @brief Identificador único do vértice.
     */
    int key;
    /**
     * @brief Ponteiro para a lista de arestas que partem deste vértice.
     */
    Edge *edges;
    /**
     * @brief Array contendo os identificadores dos vértices sucessores.
     */
    int *sucessors;
    /**
     * @brief Array contendo os identificadores dos vértices predecessores.
     */
    int *predecessors;
    /**
     * @brief Grau de saída do vértice (número de arestas que saem deste vértice).
     */
    int exitDegree;
    /**
     * @brief Grau de entrada do vértice (número de arestas que chegam a este vértice).
     */
    int entryDegree;
} Vertice;

/* Estrutura para o Grafo Direcionado com Lista de Adjacência */
/**
 * @struct Graph
 * @brief Representa um grafo direcionado utilizando listas de adjacência.
 */
typedef struct Graph {
    /**
     * @brief Número total de vértices no grafo.
     */
    int v_nums;

    /**
     * @brief Número total de arestas no grafo.
     */
    int e_nums;

    /**
     * @brief Array de vértices presentes no grafo.
     */
    Vertice *vertices;

    /**
     * @brief Função para adicionar uma aresta ao grafo.
     *
     * @param self Ponteiro para o próprio grafo.
     * @param origin_key Identificador do vértice de origem.
     * @param dest_key Identificador do vértice de destino.
     */
    void (*add_edge)(struct Graph *self, int origin_key, int dest_key);

    /**
     * @brief Função para obter um vértice específico pelo seu identificador.
     *
     * @param self Ponteiro para o próprio grafo.
     * @param vertice_key Identificador do vértice a ser obtido.
     * @return Ponteiro para o vértice correspondente.
     */
    Vertice* (*get_vertice)(const struct Graph *self, int vertice_key);

    /**
     * @brief Função para obter o grau de saída de um vértice.
     *
     * @param self Ponteiro para o próprio grafo.
     * @param v Identificador do vértice.
     * @return Grau de saída do vértice.
     */
    int (*get_exitDegree)(const struct Graph *self, int v);

    /**
     * @brief Função para obter o grau de entrada de um vértice.
     *
     * @param self Ponteiro para o próprio grafo.
     * @param v Identificador do vértice.
     * @return Grau de entrada do vértice.
     */
    int (*get_entryDegree)(const struct Graph *self, int v);

    /**
     * @brief Função para obter os sucessores de um vértice.
     *
     * @param self Ponteiro para o próprio grafo.
     * @param v Identificador do vértice.
     * @return Array de identificadores dos sucessores do vértice, terminado por -1.
     */
    int *(*get_sucessors)(const struct Graph *self, int v);

    /**
     * @brief Função para obter os predecessores de um vértice.
     *
     * @param self Ponteiro para o próprio grafo.
     * @param v Identificador do vértice.
     * @return Array de identificadores dos predecessores do vértice, terminado por -1.
     */
    int *(*get_predecessors)(const struct Graph *self, int v);

    /**
     * @brief Função para calcular o caminho mais curto entre dois vértices.
     *
     * @param self Ponteiro para o próprio grafo.
     * @param origin Identificador do vértice de origem.
     * @param dest Identificador do vértice de destino.
     * @return Array contendo o caminho mais curto do vértice de origem ao destino, terminado por -1.
     */
    int *(*shortest_path)(struct Graph *self, int origin, int dest);

    /**
     * @brief Função para realizar uma busca em profundidade entre dois vértices.
     *
     * @param self Ponteiro para o próprio grafo.
     * @param origin Identificador do vértice de origem.
     * @param dest Identificador do vértice de destino.
     * @return Array contendo o caminho encontrado pela busca em profundidade, terminado por -1.
     */
    int *(*deep_first_search)(struct Graph *self, int origin, int dest);

    /**
     * @brief Função para realizar uma busca em largura entre dois vértices.
     *
     * @param self Ponteiro para o próprio grafo.
     * @param origin Identificador do vértice de origem.
     * @param dest Identificador do vértice de destino.
     * @return Array contendo o caminho encontrado pela busca em largura, terminado por -1.
     */
    int *(*breadth_first_search)(struct Graph *self, int origin, int dest);

    /**
     * @brief Função para escrever a representação do grafo em um arquivo.
     *
     * @param self Ponteiro para o próprio grafo.
     * @param f Ponteiro para o arquivo onde o grafo será escrito.
     */
    void (*write_graph)(const struct Graph *self, FILE *f);

    /**
     * @brief Função para explorar todas as arestas do grafo utilizando busca em profundidade.
     *
     * @param g Ponteiro para o grafo.
     * @param start Vértice inicial para a exploração.
     */
    void (*explore_all_edges_deep)(const struct Graph *g, int start);

    /**
     * @brief Função para verificar se existe uma aresta entre dois vértices específicos.
     *
     * @param g Ponteiro para o grafo.
     * @param origin Identificador do vértice de origem.
     * @param dest Identificador do vértice de destino.
     * @return true se a aresta existir, false caso contrário.
     */
    bool (*edge_exists)(const struct Graph *g, int origin, int dest);
} Graph;


/* Prototipação das funções */

/**
 * Cria uma nova aresta.
 * 
 * @param key O identificador do destino da aresta.
 * @param dest Um ponteiro para o vértice de destino.
 * @param origin_key O identificador do vértice de origem.
 * @return Um ponteiro para a nova aresta alocada.
 */
Edge *new_edge(int key, Vertice *dest, int origin_key);

/**
 * Obtém a última aresta de uma lista de arestas.
 * 
 * @param e Um ponteiro para a primeira aresta da lista.
 * @return Um ponteiro para a última aresta da lista ou NULL se a lista estiver vazia.
 */
Edge *get_last_edge(Edge *e);

/**
 * Cria um novo vértice.
 * 
 * @param key O identificador do vértice.
 * @return Um ponteiro para o novo vértice alocado.
 */
Vertice *new_vertice(int key);

/**
 * Converte o identificador do vértice em um índice de array.
 * 
 * @param v O identificador do vértice.
 * @return O índice correspondente ao identificador do vértice.
 */
int get_v_index(int v);

/**
 * Valida se um vértice está dentro do número permitido de vértices.
 * 
 * @param v O identificador do vértice a ser validado.
 * @param v_nums O número total de vértices no grafo.
 * @return void (A função encerra o programa se o vértice não for válido).
 */
void validate_v(int v, int v_nums);

/**
 * Cria um novo grafo.
 * 
 * @param v_nums O número de vértices no grafo.
 * @param e_num O número de arestas no grafo.
 * @return Um ponteiro para o novo grafo alocado.
 */
Graph *new_graph(int v_nums, int e_num);

/**
 * Adiciona uma aresta entre dois vértices no grafo.
 * 
 * @param g Um ponteiro para o grafo.
 * @param origin O identificador do vértice de origem.
 * @param dest O identificador do vértice de destino.
 * @return void
 */
void add_edge(Graph *g, int origin, int dest);

/**
 * Verifica se existe uma aresta entre dois vértices.
 * 
 * @param g Um ponteiro para o grafo.
 * @param origin O identificador do vértice de origem.
 * @param dest O identificador do vértice de destino.
 * @return true se a aresta existir, false caso contrário.
 */
bool edge_exists(const Graph *g, int origin, int dest);

/**
 * Obtém um vértice a partir de seu identificador.
 * 
 * @param self Um ponteiro para o grafo.
 * @param vertice_key O identificador do vértice.
 * @return Um ponteiro para o vértice correspondente.
 */
Vertice *get_vertice(const Graph *self, int vertice_key);

/**
 * Calcula o grau de saída (exit degree) de um vértice.
 * 
 * @param g Um ponteiro para o grafo.
 * @param v O identificador do vértice.
 * @return O grau de saída do vértice.
 */
int get_exitDegree(const Graph *g, int v);

/**
 * Calcula o grau de entrada (entry degree) de um vértice.
 * 
 * @param g Um ponteiro para o grafo.
 * @param v O identificador do vértice.
 * @return O grau de entrada do vértice.
 */
int get_entryDegree(const Graph *g, int v);

/**
 * Obtém os sucessores de um vértice.
 * 
 * @param g Um ponteiro para o grafo.
 * @param v O identificador do vértice.
 * @return Um array contendo os identificadores dos sucessores do vértice, terminado por -1.
 */
int *get_sucessors(const Graph *g, int v);

/**
 * Obtém os predecessores de um vértice.
 * 
 * @param g Um ponteiro para o grafo.
 * @param v O identificador do vértice.
 * @return Um array contendo os identificadores dos predecessores do vértice, terminado por -1.
 */
int *get_predecessors(const Graph *g, int v);

/**
 * Encontra o caminho mais curto entre dois vértices usando busca por menor caminho.
 * 
 * @param g Um ponteiro para o grafo.
 * @param origin O identificador do vértice de origem.
 * @param dest O identificador do vértice de destino.
 * @return Um array contendo o caminho mais curto do vértice de origem ao destino, terminado por -1.
 */
int *shortest_path(Graph *g, int origin, int dest);

/**
 * Realiza uma busca em profundidade entre dois vértices.
 * 
 * @param g Um ponteiro para o grafo.
 * @param origin O identificador do vértice de origem.
 * @param dest O identificador do vértice de destino.
 * @return Um array contendo o caminho encontrado pela busca em profundidade, terminado por -1.
 */
int *deep_first_search(Graph *g, int origin, int dest);

/**
 * Realiza uma busca em largura entre dois vértices.
 * 
 * @param g Um ponteiro para o grafo.
 * @param origin O identificador do vértice de origem.
 * @param dest O identificador do vértice de destino.
 * @return Um array contendo o caminho encontrado pela busca em largura, terminado por -1.
 */
int *breadth_first_search(Graph *g, int origin, int dest);

/**
 * Explora todas as arestas de um grafo utilizando busca em profundidade.
 * 
 * @param g Um ponteiro para o grafo.
 * @param start O vértice inicial para a exploração.
 * @return void
 */
void explore_all_edges_deep(const Graph *g, int start);

/**
 * Lê um grafo a partir de um arquivo.
 * 
 * @param file Um ponteiro para o arquivo contendo a representação do grafo.
 * @return Um ponteiro para o grafo lido do arquivo.
 */
Graph *graph_from_file(FILE *file);

/**
 * Gera um grafo aleatório com um determinado número de vértices e arestas.
 * 
 * @param v_nums O número de vértices no grafo.
 * @param e_nums O número de arestas no grafo.
 * @param max_edge_in_vertice O número máximo de arestas que um vértice pode ter.
 * @return Um ponteiro para o grafo gerado aleatoriamente.
 */
Graph *graph_from_random(int v_nums, int e_nums, int max_edge_in_vertice);

/**
 * Escreve a representação de um grafo em um arquivo.
 * 
 * @param self Um ponteiro para o grafo.
 * @param f Um ponteiro para o arquivo onde o grafo será escrito.
 * @return void
 */
void write_graph(const Graph *self, FILE *f);

/* Funções utilitárias internas */

/**
 * Marca um vértice como visitado.
 * 
 * @param visited Um array de booleanos que mantém o estado de visitação dos vértices.
 * @param v O identificador do vértice a ser marcado como visitado.
 * @return void
 */
void set_visited(bool *visited, int v);

/**
 * Marca um vértice como explorado.
 * 
 * @param marked_array Um array de booleanos que mantém o estado de exploração dos vértices.
 * @param v O identificador do vértice a ser marcado como explorado.
 * @return void
 */
void set_marked(bool *marked_array, int v);

/**
 * Define a distância de um vértice a partir da origem.
 * 
 * @param disMap Um array que armazena as distâncias dos vértices.
 * @param v O identificador do vértice cuja distância será definida.
 * @param distance A distância a ser atribuída ao vértice.
 * @return void
 */
void set_distance(int *disMap, int v, int distance);

/**
 * Define de onde um vértice foi alcançado.
 * 
 * @param came_from Um array que armazena de onde cada vértice foi alcançado.
 * @param v O identificador do vértice.
 * @param previous O identificador do vértice anterior no caminho.
 * @return void
 */
void set_came_from(int *came_from, int v, int previous);

/**
 * Verifica se um vértice foi visitado.
 * 
 * @param visited Um array de booleanos que mantém o estado de visitação dos vértices.
 * @param v O identificador do vértice.
 * @return true se o vértice foi visitado, false caso contrário.
 */
bool is_visited(const bool *visited, int v);

/**
 * Verifica se um vértice foi marcado.
 * 
 * @param marked_array Um array de booleanos que mantém o estado de exploração dos vértices.
 * @param v O identificador do vértice.
 * @return true se o vértice foi marcado, false caso contrário.
 */
bool is_marked(const bool *marked_array, int v);

/**
 * Obtém a distância de um vértice a partir da origem.
 * 
 * @param disMap Um array que armazena as distâncias dos vértices.
 * @param v O identificador do vértice.
 * @return A distância do vértice a partir da origem.
 */
int get_distance(const int *disMap, int v);

/**
 * Obtém de onde um vértice foi alcançado.
 * 
 * @param came_from Um array que armazena de onde cada vértice foi alcançado.
 * @param v O identificador do vértice.
 * @return O identificador do vértice anterior no caminho.
 */
int get_came_from(const int *came_from, int v);

/**
 * Constrói o caminho final do vértice de origem até o destino.
 * 
 * @param g Um ponteiro para o grafo.
 * @param origin O identificador do vértice de origem.
 * @param dest O identificador do vértice de destino.
 * @param came_from Um array que armazena de onde cada vértice foi alcançado.
 * @return Um array contendo o caminho do vértice de origem ao destino, terminado por -1.
 */
int *get_final_path(const Graph *g, int origin, int dest, const int *came_from);



/* Implementações */
bool compare(int enqueued_priority, int insert_priority) {
    return insert_priority > enqueued_priority;
}
/* Implementação de funções para arestas */
Edge *new_edge(int key, Vertice *dest, int origin_key) {
    Edge *e = (Edge *)calloc(1, sizeof(Edge));
    if (e == NULL) {
        perror("Erro ao alocar memória para a aresta\n");
        exit(EXIT_FAILURE);
    }
    e->dest_key = key;
    e->next = NULL;
    e->dest = dest;
    e->origin_key = origin_key;
    e->id = EDGE_COUNTER++;
    return e;
}

Edge *get_last_edge(Edge *e) {
    if (e == NULL) {
        return NULL;
    }
    while (e->next != NULL) {
        e = e->next;
    }
    return e;
}

/* Implementação de funções para vértices */
Vertice *new_vertice(int key) {
    Vertice *v = (Vertice *)calloc(1, sizeof(Vertice));
    if (v == NULL) {
        perror("Erro ao alocar memória para o vértice\n");
        exit(EXIT_FAILURE);
    }
    v->key = key;
    v->edges = NULL;
    return v;
}

int get_v_index(int v) {
    return v - 1;
}

void validate_v(int v, int v_nums) {
    if (get_v_index(v) >= v_nums) {
        perror("Vértice não existe\n");
        exit(1);
    }
}

/* Implementação de funções para o grafo */
Graph *new_graph(int v_nums, int e_num) {
    Graph *g = (Graph *)calloc(1, sizeof(Graph));
    if (g == NULL) {
        perror("Erro ao alocar memória\n");
        exit(1);
    }

    g->v_nums = v_nums;
    g->e_nums = e_num;
    g->vertices = (Vertice *)calloc(v_nums, sizeof(Vertice));

    if (g->vertices == NULL) {
        perror("Erro ao alocar memória\n");
        exit(1);
    }

    for (int i = 0; i < v_nums; i++) {
        g->vertices[i].key = i + 1;
        g->vertices[i].edges = NULL;
    }

    g->add_edge = add_edge;
    g->get_exitDegree = get_exitDegree;
    g->get_entryDegree = get_entryDegree;
    g->get_sucessors = get_sucessors;
    g->get_predecessors = get_predecessors;
    g->shortest_path = shortest_path;
    g->deep_first_search = deep_first_search;
    g->breadth_first_search = breadth_first_search;
    g->get_vertice = get_vertice;
    g->write_graph = write_graph;
    g->explore_all_edges_deep = explore_all_edges_deep;
    g->edge_exists = edge_exists;   

    return g;
}

void add_edge(Graph *g, int origin, int dest) {
    validate_v(origin, g->v_nums);
    validate_v(dest, g->v_nums);
    Edge *e = new_edge(dest, &g->vertices[get_v_index(dest)], origin);
    Vertice *v = &g->vertices[get_v_index(origin)];
    if (v->edges == NULL) {
        v->edges = e;
    } else {
        Edge *last = get_last_edge(v->edges);
        last->next = e;
    }
}

bool edge_exists(const Graph *g, int origin, int dest) {
    validate_v(origin, g->v_nums);
    validate_v(dest, g->v_nums);
    Edge *tempE = g->vertices[get_v_index(origin)].edges;
    while (tempE != NULL) {
        if (tempE->dest_key == dest) {
            return true;
        }
        tempE = tempE->next;
    }
    return false;
}

Vertice *get_vertice(const Graph *self, int vertice_key) {
    int index = get_v_index(vertice_key);
    if(index >= self->v_nums) {
        perror("Vértice não existe\n");
        exit(1);
    }
    return &self->vertices[index];
}

int get_exitDegree(const Graph *g, int v) {
    validate_v(v, g->v_nums);
    int degree = 0;
    Edge *tempE = g->vertices[get_v_index(v)].edges;
    while (tempE != NULL) {
        degree++;
        tempE = tempE->next;
    }
    return degree;
}

int get_entryDegree(const Graph *g, int v) {
    validate_v(v, g->v_nums);
    int entryDegree = 0;
    for (int i = 0; i < g->v_nums; i++) {
        Edge *tempE = g->vertices[i].edges;
        while (tempE != NULL) {
            if (tempE->dest_key == v) {
                entryDegree++;
            }
            tempE = tempE->next;
        }
    }
    return entryDegree;
}

int *get_sucessors(const Graph *g, int v) {
    validate_v(v, g->v_nums);
    int *sucessors = (int *)calloc(g->v_nums, sizeof(int));
    if (sucessors == NULL) {
        perror("Erro ao alocar memória para sucessores\n");
        exit(EXIT_FAILURE);
    }
    int sucessors_index = 0;

    Edge *tempE = g->vertices[get_v_index(v)].edges;
    while (tempE != NULL) {
        sucessors[sucessors_index++] = tempE->dest_key;
        tempE = tempE->next;
    }
    sucessors[sucessors_index++] = -1;
    return (int *)realloc(sucessors, sizeof(int) * sucessors_index);
}

int *get_predecessors(const Graph *g, int v) {
    validate_v(v, g->v_nums);
    int *predecessors = (int *)calloc(g->v_nums, sizeof(int));
    if (predecessors == NULL) {
        perror("Erro ao alocar memória para predecessores\n");
        exit(EXIT_FAILURE);
    }
    int predecessors_index = 0;
    for (int i = 0; i < g->v_nums; i++) {
        Edge *tempE = g->vertices[i].edges;
        while (tempE != NULL) {
            if (tempE->dest_key == v) {
                predecessors[predecessors_index++] = g->vertices[i].key;
            }
            tempE = tempE->next;
        }
    }
    predecessors[predecessors_index++] = -1;
    return (int *)realloc(predecessors, sizeof(int) * predecessors_index);
}

int *shortest_path(Graph *g, int origin, int dest) {
    bool *visited = (bool *)calloc(g->v_nums, sizeof(bool));
    int *distances = (int *)calloc(g->v_nums, sizeof(int));
    int *came_from = (int *)calloc(g->v_nums, sizeof(int));
    if (!visited || !distances || !came_from) {
        perror("Erro ao alocar memória para busca\n");
        exit(EXIT_FAILURE);
    }

    PriorityQueue *pq = new_priority_queue(compare);
    pq->push(pq, origin, 0);
    set_distance(distances, origin, 0);
    set_visited(visited, origin);
    int current = -1;
    while (!pq->is_empty(pq)) {
        current = pq->pop(pq);
        if (current == dest) {
            break;
        }
        int *sucessors = g->get_sucessors(g, current);
        for (int i = 0; sucessors[i] != -1; i++) {
            int sucessor = sucessors[i];
            if (!is_visited(visited, sucessor)) {
                set_visited(visited, sucessor);
                int sucessor_distance = get_distance(distances, current) + 1;
                set_distance(distances, sucessor, sucessor_distance);
                set_came_from(came_from, sucessor, current);
                pq->push(pq, sucessor, sucessor_distance);
            }
        }
        free(sucessors);
    }
    free(visited);
    free(distances);

    return get_final_path(g, origin, dest, came_from);
}

int *deep_first_search(Graph *g, int origin, int dest) {
    bool *visited = (bool *)calloc(g->v_nums, sizeof(bool));
    int *came_from = (int *)calloc(g->v_nums, sizeof(int));
    if (!visited || !came_from) {
        perror("Erro ao alocar memória para busca em profundidade\n");
        exit(EXIT_FAILURE);
    }

    Stack *s = new_stack();
    s->push(s, origin);
    set_visited(visited, origin);
    int current;
    while (!s->is_empty(s)) {
        current = s->pop(s);
        if (current == dest) {
            break;
        }
        int *sucessors = g->get_sucessors(g, current);

        for (int i = 0; sucessors[i] != -1; i++) {
            int sucessor = sucessors[i];
            if (!is_visited(visited, sucessor)) {
                set_visited(visited, sucessor);
                set_came_from(came_from, sucessor, current);
                s->push(s, sucessor);
            }
        }
        free(sucessors);
    }
    free(visited);

    return get_final_path(g, origin, dest, came_from);
}

int *breadth_first_search(Graph *g, int origin, int dest) {
    bool *visited = (bool *)calloc(g->v_nums, sizeof(bool));
    int *came_from = (int *)calloc(g->v_nums, sizeof(int));
    if (!visited || !came_from) {
        perror("Erro ao alocar memória para busca em largura\n");
        exit(EXIT_FAILURE);
    }

    Queue *q = new_queue();
    set_visited(visited, origin);
    int current;
    q->enqueue(q, origin);
    while (!q->is_empty(q)) {
        current = q->dequeue(q);
        if (current == dest) {
            break;
        }
        int *sucessors = g->get_sucessors(g, current);
        for (int i = 0; sucessors[i] != -1; i++) {
            int sucessor = sucessors[i];
            if (!is_visited(visited, sucessor)) {
                set_visited(visited, sucessor);
                set_came_from(came_from, sucessor, current);
                q->enqueue(q, sucessor);
            }
        }
        free(sucessors);
    }
    free(visited);

    return get_final_path(g, origin, dest, came_from);
}

/* Funções utilitárias */
void set_visited(bool *visited, int v) {
    visited[get_v_index(v)] = true;
}

void set_marked(bool *marked_array, int v) {
    marked_array[get_v_index(v)] = true;
}

void set_distance(int *disMap, int v, int distance) {
    disMap[get_v_index(v)] = distance;
}

void set_came_from(int *came_from, int v, int previous) {
    came_from[get_v_index(v)] = previous;
}

bool is_visited(const bool *visited, int v) {
    return visited[get_v_index(v)];
}

bool is_marked(const bool *marked_array, int v) {
    return marked_array[get_v_index(v)];
}

int get_distance(const int *disMap, int v) {
    return disMap[get_v_index(v)];
}

int get_came_from(const int *came_from, int v) {
    return came_from[get_v_index(v)];
}

int *get_final_path(const Graph *g, int origin, int dest, const int *came_from) {
    int current = dest;
    int *path = (int *)calloc(g->v_nums, sizeof(int));
    if (path == NULL) {
        perror("Erro ao alocar memória para o caminho\n");
        exit(EXIT_FAILURE);
    }
    int path_index = 0;

    while (current != origin) {
        if (get_v_index(current) < 0 || get_v_index(current) >= g->v_nums) {
            fprintf(stderr, "Índice inválido detectado no caminho.\n");
            free(path);
            return NULL;
        }

        path[path_index++] = current;
        current = get_came_from(came_from, current);
    }

    path[path_index++] = origin;

    int *final_path = (int *)calloc(path_index + 1, sizeof(int));
    if (final_path == NULL) {
        perror("Erro ao alocar memória para o caminho final\n");
        free(path);
        exit(EXIT_FAILURE);
    }

    for (int i = 0; i < path_index; i++) {
        final_path[i] = path[path_index - i - 1];
    }

    final_path[path_index] = -1;

    free(path);

    return final_path;
}

/* Implementações de busca em profundidade e largura */
void Pri_explore_all_edges_deep(Vertice *v, int *clock, int *TD, int *TT, int *FATHER, Graph *g) {
    TD[v->key - 1] = ++(*clock);
    printf("visited: %d time:%d\n", v->key, *clock);
    int *w_vetor = g->get_sucessors(g, v->key);

    for (int i = 0; w_vetor[i] != -1; i++) {
        int w = w_vetor[i];
        if (TD[w - 1] == 0) {
            FATHER[w - 1] = v->key;
            printf("Aresta arvore: %d -> %d\n", v->key, w);
            Pri_explore_all_edges_deep(g->get_vertice(g, w), clock, TD, TT, FATHER, g);
        } else {
            if (TT[w - 1] == 0) {
                printf("Aresta Retorno: %d -> %d\n", v->key, w);
            } else if (TD[v->key - 1] < TD[w - 1]) {
                printf("Aresta Avanco: %d -> %d\n", v->key, w);
            } else {
                printf("Aresta Cruzamento: %d -> %d\n", v->key, w);
            }
        }
    }
    TT[v->key - 1] = ++(*clock);
    printf("explored: %d time:%d\n", v->key, *clock);
}

void explore_all_edges_deep(const Graph *g, int start) {
    int CLOCK = 0;  
    int *TD = (int *)calloc(g->v_nums, sizeof(int));
    int *TT = (int *)calloc(g->v_nums, sizeof(int));
    int *FATHER = (int *)calloc(g->v_nums, sizeof(int));
    
    do {
        printf("Root: %d\n", start);
        Pri_explore_all_edges_deep(g->get_vertice(g, start), &CLOCK, TD, TT, FATHER, g);
    } while ((start = check_explored_all(TT, g)));   
}

int check_explored_all(int *TT, Graph *g) {
    for (int i = 0; i < g->v_nums; i++) {
        if (TT[i] == 0) {
           return i + 1;
        }
    }
    return 0;
}

void pri_explore_breadth(const Graph *g, int *CLOCK, int *NASCIMENTO, int *NIVEL, int *FATHER, Queue *q) {
    while (!q->is_empty(q)) {
        int _v = q->dequeue(q);
        int v_index = get_v_index(_v);
        int *sucessors = g->get_sucessors(g, _v);
        for (int i = 0; sucessors[i] != -1; i++) {
            int w = sucessors[i], w_index = get_v_index(w);
            if (!NASCIMENTO[w_index]) {
                NASCIMENTO[w_index] = ++(*CLOCK);
                FATHER[w_index] = _v;
                NIVEL[w_index] = NIVEL[v_index] + 1;
                q->enqueue(q, w);
                printf("Aresta Arvore: %d -> %d\n", _v, w);
            } else {
                printf("Aresta Cruzamento: %d -> %d\n", _v, w);
            }
        }
    }
}

void explore_all_edges_breadth(const Graph *g, int v_root) {
    int CLOCK = 0;  
    int *NASCIMENTO = (int *)calloc(g->v_nums, sizeof(int));
    int *NIVEL = (int *)calloc(g->v_nums, sizeof(int));
    int *FATHER = (int *)calloc(g->v_nums, sizeof(int));
    Queue *q = new_queue();
    do {
        printf("Root: %d\n", v_root);
        CLOCK++; NASCIMENTO[v_root - 1] = CLOCK;
        q->enqueue(q, v_root);
        pri_explore_breadth(g, &CLOCK, NASCIMENTO, NIVEL, FATHER, q);
    } while ((v_root = check_explored_all(NASCIMENTO, g)));
}

/* Funções de leitura e escrita de grafos */
Graph *graph_from_file(FILE *file) {
    char buffer[100];
    if (fgets(buffer, sizeof(buffer), file) == NULL) {
        perror("Erro ao ler a primeira linha do arquivo\n");
        exit(EXIT_FAILURE);
    }

    int v_number, e_number;
    sscanf(buffer, "%d %d", &v_number, &e_number);

    Graph *g = new_graph(v_number, e_number);
    if (g == NULL) {
        perror("Erro ao alocar memória\n");
        exit(1);
    }

    while (fgets(buffer, 100, file) != NULL) {
        int origin, dest;
        sscanf(buffer, "%d %d", &origin, &dest);
        g->add_edge(g, origin, dest);
    }

     for(int i = 0; i < g->v_nums; i++) {
        g->vertices[i].exitDegree = g->get_exitDegree(g, i + 1);
        g->vertices[i].entryDegree = g->get_entryDegree(g, i + 1);
        g->vertices[i].sucessors = g->get_sucessors(g, i + 1);
        g->vertices[i].predecessors = g->get_predecessors(g, i + 1);
    }

    return g;
}

Graph *graph_from_random(int v_nums, int e_nums, int max_edge_in_vertice) {
    Graph *g = new_graph(v_nums, e_nums);
    if (g == NULL) {
        perror("Erro ao alocar memória\n");
        exit(1);
    }

    // Primeiro, conecte todos os vértices de forma que o grafo seja conexo
    for (int i = 1; i < v_nums; i++) {
        int origin = i;
        int dest = i + 1;
        g->add_edge(g, origin, dest);
    }

    // Adicione arestas adicionais para aumentar a conectividade
    for (int i = v_nums - 1; i < e_nums; i++) {
        int origin = rand() % v_nums + 1;
        int dest = rand() % v_nums + 1;
        if (origin == dest || g->edge_exists(g, origin, dest)) {
            i--;
            continue;
        }
        g->add_edge(g, origin, dest);
    }

    return g;
}

void write_graph(const Graph *self, FILE *f) {
    fprintf(f, "%d %d\n", self->v_nums, self->e_nums);
    for (int i = 0; i < self->v_nums; i++) {
        Edge *temp = self->vertices[i].edges;
        while (temp != NULL) {
            fprintf(f, "%d %d\n", temp->origin_key, temp->dest_key);
            temp = temp->next;
        }
    }
}

#endif // GRAPH_H
