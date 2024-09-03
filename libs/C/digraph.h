#ifndef DIGRAPH_H
#define DIGRAPH_H

#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>
#include "dataStructs.h"

/* Estrutura para a Aresta */
/**
 * @struct Edge
 * @brief Representa uma aresta no grafo.
 */
typedef struct Edge {
    /**
     * @brief Identificador único da aresta.
     * 
     * incrementado a cada nova aresta criada a partir de `EDGE_COUNTER` inicializado com 0.
     * 
     * otimo parametro para indexação de arestas
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
     * @brief Índice do vértice na lista de vértices do grafo.
    */
    int index;

    /**
     * @brief Ponteiro para a lista de arestas que partem deste vértice.
     */
    Edge *edges;

    /**
     * @brief Função para verificar se existe uma aresta entre este vértice e outro vértice específico.
     * @param self Ponteiro para o próprio vértice.
     * @param dest_v_Key Identificador do vértice de origem.
     * @return `Ponteiro` para a aresta correspondente 
     * 
     * ou
     * 
     * `NULL` caso a aresta não exista.
     */
    Edge* (*get_edge)(struct Vertice * self ,int dest_v_Key);
} Vertice;

/**  
@brief Contador global para gerar IDs únicos para os vértices.
*/
static int VERTICE_COUNTER = 0;
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
     * @brief Função para escrever a representação do grafo em um arquivo.
     *
     * @param self Ponteiro para o próprio grafo.
     * @param f Ponteiro para o arquivo onde o grafo será escrito.
     */
    void (*write_graph)(const struct Graph *self, FILE *f);

    /**
     * @brief Função para explorar todas as arestas do grafo utilizando busca em profundidade.
     * 
     * `sem recursão`
     * 
     * @param g Ponteiro para o grafo.
     * @param start Vértice inicial para a exploração.
     * @return void
     */
    void (*explore_all_edges_deep_noRecursion)(const struct Graph *g, int start);

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
Edge *new_edge(int key, Vertice *dest, int origin_key);
Edge *get_last_edge(Edge *e);
Vertice *new_vertice(int key);
Edge *get_edge(Vertice *self, int dest_v_Key);
void validate_v(int v, int v_nums);
Graph *new_graph(int v_nums, int e_num);
void add_edge(Graph *g, int origin, int dest);
bool edge_exists(const Graph *g, int origin, int dest);
Vertice *get_vertice(const Graph *self, int vertice_key);
int get_exitDegree(const Graph *g, int v);
int get_entryDegree(const Graph *g, int v);
int *get_sucessors(const Graph *g, int v);
int *get_predecessors(const Graph *g, int v);
void explore_all_edges_deep_noRecursion(const struct Graph *g, int start);
void explore_all_edges_deep(const Graph *g, int start);
Graph *graph_from_file(FILE *file);
Graph *graph_from_random(int v_nums, int e_nums, int max_edge_in_vertice);
void write_graph(const Graph *self, FILE *f);

// utilitárias sem declaração prévia 
bool compare(int enqueued_priority, int insert_priority) {
    return insert_priority > enqueued_priority;
}

int check_explored_all(int *array, const Graph *g) {
    for (int i = 0; i < g->v_nums; i++) {
        if (array[i] == 0) {
            return i + 1;
        }
    }
    return 0;
}


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

Vertice *new_vertice(int key) {
    Vertice *v = (Vertice *)calloc(1, sizeof(Vertice));
    if (v == NULL) {
        perror("Erro ao alocar memória para o vértice\n");
        exit(EXIT_FAILURE);
    }
    v->key = key;
    v->edges = NULL;
    v->get_edge = get_edge;
    v->index = VERTICE_COUNTER++;
    return v;
}

void validate_v(int v, int v_nums) {
    if (v - 1 >= v_nums) {
        perror("Vértice não existe\n");
        exit(1);
    }
}

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
        g->vertices[i] = *new_vertice(i + 1);
    }

    g->add_edge = add_edge;
    g->get_exitDegree = get_exitDegree;
    g->get_entryDegree = get_entryDegree;
    g->get_sucessors = get_sucessors;
    g->get_predecessors = get_predecessors;
    g->get_vertice = get_vertice;
    g->write_graph = write_graph;
    g->explore_all_edges_deep = explore_all_edges_deep;
    g->explore_all_edges_deep_noRecursion = explore_all_edges_deep_noRecursion;
    g->edge_exists = edge_exists;   

    return g;
}

void add_edge(Graph *g, int origin, int dest) {
    validate_v(origin, g->v_nums);
    validate_v(dest, g->v_nums);
    Edge *e = new_edge(dest, &g->vertices[origin - 1], origin);
    Vertice *v = &g->vertices[origin - 1];
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
    Edge *tempE = g->vertices[origin - 1].edges;
    while (tempE != NULL) {
        if (tempE->dest_key == dest) {
            return true;
        }
        tempE = tempE->next;
    }
    return false;
}

Vertice *get_vertice(const Graph *self, int vertice_key) {
    if (vertice_key - 1 >= self->v_nums) {
        perror("Vértice não existe\n");
        exit(1);
    }
    return &self->vertices[vertice_key - 1];
}

Edge *get_edge(Vertice *self, int dest_v_Key) {
    Edge *tempE = self->edges;
    while (tempE != NULL) {
        if (tempE->dest_key == dest_v_Key) {
            return tempE;
        }
        tempE = tempE->next;
    }
    return NULL;
}

int get_exitDegree(const Graph *g, int v) {
    validate_v(v, g->v_nums);
    int degree = 0;
    Edge *tempE = g->vertices[v - 1].edges;
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

    Edge *tempE = g->vertices[v - 1].edges;
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

/* Implementações de busca em profundidade e largura */
void Pri_explore_all_edges_deep(Vertice *v, int *clock, int *TD, int *TT, int *FATHER, const Graph *g) {
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

void explore_all_edges_deep_noRecursion(const struct Graph * g, int start) {
    int *NASCIMENTO = (int *)calloc(g->v_nums, sizeof(int));
    int *MORTE = (int *)calloc(g->v_nums, sizeof(int));
    int *FATHER = (int *)calloc(g->v_nums, sizeof(int));
    bool *EDGES_EXPLORED = (bool *)calloc(g->e_nums, sizeof(bool));
    int CLOCK = 0;
    Stack *stack = new_stack();
    stack->push(stack, start);

    while (!stack->is_empty(stack)) {
        int v = stack->peek(stack);
        int v_index = v - 1;
        Vertice *vertice = &g->vertices[v_index];

        if (NASCIMENTO[v_index] == 0) {
            NASCIMENTO[v_index] = ++CLOCK;
        }

        int discovered = 0;
        int *sucessors = g->get_sucessors(g, v);

        for (int i = 0; sucessors[i] != -1; i++) {
            int sucessor = sucessors[i];
            int sucessor_index = sucessor - 1;

            Edge *edge = vertice->get_edge(vertice, sucessor);

            if (edge == NULL) {
                printf("Edge NULL no vértice %d para o sucessor %d\n", v, sucessor);
                continue;
            }

            if (EDGES_EXPLORED[edge->id]) {
                continue;
            }

            EDGES_EXPLORED[edge->id] = true;

            if (NASCIMENTO[sucessor_index] == 0) {
                printf("Aresta Arvore: %d -> %d\n", v, sucessor);
                FATHER[sucessor_index] = v;
                stack->push(stack, sucessor);
                discovered++;
            } else {
                if (MORTE[sucessor_index] == 0)
                    printf("Aresta Retorno: %d -> %d\n", v, sucessor);
                else if (NASCIMENTO[sucessor_index] > NASCIMENTO[v_index])
                    printf("Aresta Avanco: %d -> %d\n", v, sucessor);
                else
                    printf("Aresta Cruzamento: %d -> %d\n", v, sucessor);
            }
        }
        if (discovered == 0) {
            MORTE[v_index] = ++CLOCK;
            stack->pop(stack);
        }
    }
}

void pri_explore_breadth(const Graph *g, int *CLOCK, int *NASCIMENTO, int *NIVEL, int *FATHER, Queue *q) {
    while (!q->is_empty(q)) {
        int _v = q->dequeue(q);
        int v_index = _v - 1;
        int *sucessors = g->get_sucessors(g, _v);
        for (int i = 0; sucessors[i] != -1; i++) {
            int w = sucessors[i], w_index = w - 1;
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

    return g;
}

Graph *graph_from_random(int v_nums, int e_nums, int max_edge_in_vertice) {
    Graph *g = new_graph(v_nums, e_nums);
    if (g == NULL) {
        perror("Erro ao alocar memória\n");
        exit(1);
    }

    int *edge_count = (int*)calloc(v_nums + 1, sizeof(int));
    if (edge_count == NULL) {
        perror("Erro ao alocar memória para edge_count\n");
        exit(1);
    }

    for (int i = 0; i < e_nums; i++) {
        int origin = rand() % v_nums + 1;
        int dest = rand() % v_nums + 1;

        if (origin == dest || g->edge_exists(g, origin, dest) ||
            edge_count[origin] >= max_edge_in_vertice ||
            edge_count[dest] >= max_edge_in_vertice) {
            i--;
            continue;
        }

        g->add_edge(g, origin, dest);
        edge_count[origin]++;
        edge_count[dest]++;
    }

    free(edge_count);
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

#endif // DIGRAPH_H
