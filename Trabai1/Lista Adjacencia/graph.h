#ifndef GRAPH_H
#define GRAPH_H
#include <stdio.h>
#include <stdlib.h>
#include "priorityQueue.h"
#include "stack.h"
#include "queue.h"
/*
    Lista de Adjacência - Estruturas e Funções
    Autor: Luis Phillip Lemos Martins
*/

/* Estrutura para a Aresta
    @param key: chave da aresta
    @param next: ponteiro para a próxima aresta
    @param dest: ponteiro para o vértice de destino
*/
typedef struct Edge
{
    int key;
    struct Edge *next;
    struct Vertice *dest;
} Edge;

/* Estrutura para o Vértice
    @param key: chave do vértice
    @param edges: ponteiro para a primeira aresta
*/
typedef struct Vertice
{
    int key;
    Edge *edges;
} Vertice;

/* Estrutura para o Grafo Direcionado com Lista de Adjacência]
    @param v_nums: número de vértices
    @param vertices: vetor de vértices
*/
typedef struct Graph
{
    int v_nums;
    Vertice *vertices;

    /* Função para adicionar uma aresta ao grafo entre 2 vértices
        @param self: ponteiro para o grafo
        @param origin: chave do vértice de origem
        @param dest: chave do vértice de destino
    */
    void (*add_edge)(struct Graph *self, int origin, int dest);

    /* Função para obter o grau de saída de um vértice
        @param self: ponteiro para o grafo
        @param v: chave do vértice
        @return grau de saída do vértice
    */
    int (*get_exitDegree)(struct Graph *self, int v);

    /* Função para obter o grau de entrada de um vértice
        @param self: ponteiro para o grafo
        @param v: chave do vértice
        @return grau de entrada do vértice
    */
    int (*get_entryDegree)(struct Graph *self, int v);

    /* Função para obter os sucessores de um vértice
        @param self: ponteiro para o grafo
        @param v: chave do vértice
        @return vetor com os sucessores do vértice
    */
    int *(*get_sucessors)(struct Graph *self, int v);
    /* Função para obter os predecessores de um vértice
        @param self: ponteiro para o grafo
        @param v: chave do vértice
        @return vetor com os predecessores do vértice
    */
    int *(*get_predecessors)(struct Graph *self, int v);
    /* Função para obter o menor caminho entre dois vértices
        @param self: ponteiro para o grafo
        @param origin: vértice de origem
        @param dest: vértice de destino
        @return vetor com o menor caminho
    */
    int *(*shortest_path)(struct Graph *self, int origin, int dest);

    /* Função para obter o caminho entre dois vértices utilizando busca em profundidade
        @param self: ponteiro para o grafo
        @param origin: vértice de origem
        @param dest: vértice de destino
        @return vetor com caminho
    */
    int *(*deep_first_search)(struct Graph *self, int origin, int dest);

    /* Função para obter o caminho entre dois vértices utilizando busca em largura
        @param self: ponteiro para o grafo
        @param origin: vértice de origem
        @param dest: vértice de destino
        @return vetor com caminho
    */
    int *(*breadth_first_search)(struct Graph *self, int origin, int dest);
} Graph;

// Funções para manipulação de Arestas
Edge *new_edge(int key);
Edge *get_last_edge(Edge *e);

// Funções para manipulação de Vértices
Vertice *new_vertice(int key);
int get_v_index(int v);
void validate_v(int v, int v_nums);

// Funções para manipulação de Grafos
Graph *new_graph(int v_nums);
void add_edge(Graph *g, int origin, int dest);
int get_exitDegree(Graph *g, int v);
int get_entryDegree(Graph *g, int v);
int *get_sucessors(Graph *g, int v);
int *get_predecessors(Graph *g, int v);
int *shortest_path(Graph *g, int origin, int dest);
int *deep_first_search(Graph *g, int origin, int dest);
int *breadth_first_search(Graph *g, int origin, int dest);
Graph *graph_from_file(FILE *file);
FILE *get_file();

// Implementação das funções

/* Cria uma nova aresta */
Edge *new_edge(int key)
{
    Edge *e = (Edge *)calloc(1, sizeof(Edge));
    e->key = key;
    e->next = NULL;
    e->dest = NULL;
    return e;
}

/* Obtém a última aresta de uma lista encadeada de arestas
@param e: ponteiro para a primeira aresta
 */
Edge *get_last_edge(Edge *e)
{
    if (e == NULL)
    {
        return NULL;
    }
    while (e->next != NULL)
    {
        e = e->next;
    }
    return e;
}

/* Cria um novo vértice
    @param key: chave do vértice
 */
Vertice *new_vertice(int key)
{
    Vertice *v = (Vertice *)calloc(1, sizeof(Vertice));
    v->key = key;
    v->edges = NULL;
    return v;
}

/* Obtém o índice do vértice a partir da chave */
int get_v_index(int v)
{
    return v - 1;
}

/* Valida se o vértice existe no grafo
    @param v: chave do vértice
    @param v_nums: número de vértices do grafo
    @return void
 */
void validate_v(int v, int v_nums)
{
    if (v > v_nums)
    {
        perror("Vértice não existe\n");
        exit(1);
    }
}

/*
    Cria um novo grafo Direcionado utilizando implementação de Lista de Adjacência
    @param v_nums: número de vértices do grafo
*/
Graph *new_graph(int v_nums)
{
    Graph *g = (Graph *)calloc(1, sizeof(Graph));
    if (g == NULL)
    {
        perror("Erro ao alocar memória\n");
        exit(1);
    }

    g->v_nums = v_nums;
    g->vertices = (Vertice *)calloc(v_nums, sizeof(Vertice));

    if (g->vertices == NULL)
    {
        perror("Erro ao alocar memória\n");
        exit(1);
    }

    for (int i = 0; i < v_nums; i++)
    {
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
    return g;
}

/* Adiciona uma aresta ao grafo
    @param g: ponteiro para o grafo
    @param origin: chave do vértice de origem
    @param dest: chave do vértice de destino
*/
void add_edge(Graph *g, int origin, int dest)
{
    int origin_index = get_v_index(origin);
    int dest_index = get_v_index(dest);
    if (origin_index >= g->v_nums || dest_index >= g->v_nums)
    {
        perror("Vértice não existe\n");
        exit(1);
    }
    Edge *e = new_edge(dest);
    Vertice *v = &g->vertices[origin_index];
    if (v->edges == NULL)
    {
        v->edges = e;
    }
    else
    {
        Edge *last = get_last_edge(v->edges);
        last->next = e;
    }
    e->dest = &g->vertices[dest_index];
}

/* Obtém o grau de saída de um vértice
    @param g: ponteiro para o grafo
    @param v: chave do vértice
    @return grau de saída do vértice
    */
int get_exitDegree(Graph *g, int v)
{
    int v_index = get_v_index(v);
    validate_v(v, g->v_nums);
    int degree = 0;
    Edge *tempE = g->vertices[v_index].edges;
    while (tempE != NULL)
    {
        degree++;
        tempE = tempE->next;
    }
    return degree;
}

/* Obtém o grau de entrada de um vértice
    @param g: ponteiro para o grafo
    @param v: chave do vértice
    @return grau de entrada do vértice
 */
int get_entryDegree(Graph *g, int v)
{
    validate_v(v, g->v_nums);
    int entryDegree = 0;
    for (int i = 0; i < g->v_nums; i++)
    {
        Edge *tempE = g->vertices[i].edges;
        while (tempE != NULL)
        {
            if (tempE->key == v)
            {
                entryDegree++;
            }
            tempE = tempE->next;
        }
    }
    return entryDegree;
}

/* Obtém os sucessores de um vértice terminando com -1
    @param g: ponteiro para o grafo
    @param v: chave do vértice
    @return vetor com os sucessores do vértice
 */
int *get_sucessors(Graph *g, int v)
{
    validate_v(v, g->v_nums);
    int *sucessors = (int *)calloc(g->v_nums, sizeof(int));
    int sucessors_index = 0;

    Edge *tempE = g->vertices[get_v_index(v)].edges;
    while (tempE != NULL)
    {
        sucessors[sucessors_index++] = tempE->key;
        tempE = tempE->next;
    }
    sucessors[sucessors_index++] = -1;
    return (int *)realloc(sucessors, sizeof(int) * sucessors_index);
}

/* Obtém os predecessores de um vértice terminando com -1
    @param g: ponteiro para o grafo
    @param v: chave do vértice
    @return vetor com os predecessores do vértice
*/
int *get_predecessors(Graph *g, int v)
{
    validate_v(v, g->v_nums);
    int *predecessors = (int *)calloc(g->v_nums, sizeof(int));
    int predecessors_index = 0;
    for (int i = 0; i < g->v_nums; i++)
    {
        Edge *tempE = g->vertices[i].edges;
        while (tempE != NULL)
        {
            if (tempE->key == v)
            {
                predecessors[predecessors_index++] = i + 1;
            }
            tempE = tempE->next;
        }
    }
    predecessors[predecessors_index++] = -1;
    return (int *)realloc(predecessors, sizeof(int) * predecessors_index);
}



bool compare(int enqueued_priority, int insert_priority)
{
    return insert_priority > enqueued_priority;
}

void set_visited(bool *visited, int v, int v_index)
{
    visited[v_index] = true;
}

void set_distance(int *disMap, int v, int v_index, int distance)
{
    disMap[v_index] = distance;
}

void set_came_from(int *came_from, int v, int v_index, int previous)
{
    came_from[v_index] = previous;
}
bool is_visited(bool *visited, int v, int v_index)
{
    return (bool)visited[v_index];
}
int get_distance(int *disMap, int v, int v_index)
{
    return disMap[v_index];
}
int get_came_from(int *came_from, int v, int v_index)
{
    return came_from[v_index];
}

int *get_final_path(Graph *g, int origin, int dest, int *came_from)
{
    int current = dest;
    int *path = (int *)calloc(g->v_nums, sizeof(int)); // vetor para armazenar o caminho
    int path_index = 0;
    // Construa o caminho de trás para frente
    while (current != origin)
    {
        if (current < 0 || current >= g->v_nums)
        {
            fprintf(stderr, "Índice inválido detectado no caminho.\n");
            free(path);
            return NULL;
        }

        path[path_index++] = current;
        // Atualize `current` com o vértice anterior no caminho
        current = get_came_from(came_from, current, get_v_index(current));
    }

    // Adicione a origem ao caminho
    path[path_index++] = origin;

    // Alocar o vetor para o caminho final
    int *final_path = (int *)calloc(path_index + 1, sizeof(int));
    if (final_path == NULL)
    {
        fprintf(stderr, "Erro ao alocar memória para o caminho final.\n");
        free(path);
        exit(EXIT_FAILURE);
    }

    // Inverta o caminho para a ordem correta
    for (int i = 0; i < path_index; i++)
    {
        final_path[i] = path[path_index - i - 1];
    }
    // Marque o final do caminho
    final_path[path_index] = -1;

    // Liberar a memória do vetor temporário
    free(path);

    return final_path;
}

int *shortest_path(Graph *g, int origin, int dest)
{
    bool *visited = (bool*)calloc(sizeof(bool), g->v_nums);         // mapa de visitados
    int *distances = (int*)calloc(sizeof(int), g->v_nums);         // mapa de distâncias
    int *came_from = (int *)calloc(g->v_nums, sizeof(int));  // mapa de vértices anteriores
    int *sucessors;                                          // vetor de sucessores
    PriorityQueue *pq = new_priority_queue(compare);         // fila de prioridade
    pq->push(pq, origin, 0);                                 // inserindo a origem na fila de prioridade
    set_distance(distances, origin, get_v_index(origin), 0); // setando a distância da origem como 0
    set_visited(visited, origin, get_v_index(origin));       // marcando a origem como visitada
    int current;
    while (!pq->is_empty(pq))
    {

        current = pq->pop(pq); // removendo o vértice com a menor distância da fila de prioridade
        if (current == dest)
        {
            break;
        }
        sucessors = g->get_sucessors(g, current); // obtendo os sucessores do vértice atual
        for (int i = 0; sucessors[i] != -1; i++)
        {
            
            int sucessor = sucessors[i];
            if (!is_visited(visited, sucessor, get_v_index(sucessor)))
            {
                set_visited(visited, sucessor, get_v_index(sucessor));                              // marcando o sucessor como visitado
                int sucessor_distance = get_distance(distances, current, get_v_index(current)) + 1; // calculando a distância do sucessor
                set_distance(distances,
                             sucessor,
                             get_v_index(sucessor),
                             sucessor_distance // setando a distância do sucessor
                );
                set_came_from(came_from, sucessor, get_v_index(sucessor), current); // setando o vértice anterior do sucessor
                pq->push(pq,
                         sucessor,
                         sucessor_distance); // inserindo o sucessor na fila de prioridade
            }
        }
    }
    return get_final_path(g, origin, dest, came_from);
}

int *deep_first_search(Graph *g, int origin, int dest)
{
    bool *visited = (bool *)calloc(g->v_nums, sizeof(bool)); // mapa de visitados
    int *came_from = (int *)calloc(g->v_nums, sizeof(int));  // mapa de vértices anteriores
    Stack *s = new_stack();                                  // pilha
    s->push(s, origin);                                      // inserindo a origem na pilha
    set_visited(visited, origin, get_v_index(origin));       // marcando a origem como visitada
    int current;
    int *sucessors; // vetor de sucessores
    while (!s->is_empty(s))
    {
        fflush(stdout);
        current = s->pop(s); // removendo o vértice do topo da pilha
        fflush(stdout);

        if (current == dest)
        {
            break;
        }

        sucessors = g->get_sucessors(g, current); // obtendo os sucessores do vértice atual

        for (int i = 0; sucessors[i] != -1; i++)
        {
            int sucessor = sucessors[i];
    
            fflush(stdout);

            // Verifica se o sucessor **ainda não foi visitado**
            if (!is_visited(visited, sucessor, get_v_index(sucessor)))
            {
                set_visited(visited, sucessor, get_v_index(sucessor));              // marcando o sucessor como visitado
                set_came_from(came_from, sucessor, get_v_index(sucessor), current); // setando o vértice anterior do sucessor
                s->push(s, sucessor);                                               // inserindo o sucessor na pilha
            }
        }
    }
    return get_final_path(g, origin, dest, came_from);
}


int* breadth_first_search(Graph * g, int origin, int dest){
    bool *visited = (bool *)calloc(g->v_nums, sizeof(bool)); // mapa de visitados
    int *came_from = (int *)calloc(g->v_nums, sizeof(int));  // mapa de vértices anteriores
    Queue *q = new_queue();                                  // fila
    set_visited(visited, origin, get_v_index(origin));       // marcando a origem como visitada
    int current;
    int *sucessors; // vetor de sucessores
    q->enqueue(q, origin); // inserindo a origem na fila
    while (!q->is_empty(q))
    {
        current = q->dequeue(q); // removendo o vértice da frente da fila
        if (current == dest)
        {
            break;
        }
        sucessors = g->get_sucessors(g, current); // obtendo os sucessores do vértice atual
        for (int i = 0; sucessors[i] != -1; i++)
        {
            int sucessor = sucessors[i];
            // Verifica se o sucessor **ainda não foi visitado**
            if (!is_visited(visited, sucessor, get_v_index(sucessor)))
            {
                set_visited(visited, sucessor, get_v_index(sucessor));              // marcando o sucessor como visitado
                set_came_from(came_from, sucessor, get_v_index(sucessor), current); // setando o vértice anterior do sucessor
                q->enqueue(q, sucessor);                                            // inserindo o sucessor na fila
            }
        }
    }
    return get_final_path(g, origin, dest, came_from);
}


Graph *graph_from_file(FILE *file)
{
    char buffer[100]; // Buffer para armazenar a linha

    fgets(buffer, sizeof(buffer), file); // Pegando a primeira linha

    int v_number;
    sscanf(buffer, "%d", &v_number); // Pegando o número de vértices

    Graph *g = new_graph(v_number); // Criando o grafo

    if (g == NULL)
    {
        perror("Erro ao alocar memória\n");
        exit(1);
    }
    long size = v_number;
    printf("Grafo de tamanho %d alocado com sucesso! :D\nAo todo temos %li bytes\n", v_number, (long int)(size * sizeof(Vertice)));
    fflush(stdout);
    while (fgets(buffer, 100, file) != NULL)
    {
        int origin, dest;
        sscanf(buffer, "%d %d", &origin, &dest);
        g->add_edge(g, origin, dest);
    }

    return g;
}

/* Abre um arquivo para leitura */
FILE *get_file()
{
    char file_name[200];
    printf("Digite o nome do arquivo: ");
    scanf("%200s", file_name);

    FILE *file = fopen(file_name, "r");
    if (file == NULL)
    {
        perror("Erro ao abrir o arquivo\n");
        exit(1);
    }
    printf("Arquivo aberto com sucesso\n");
    fflush(stdout);
    return file;
}

#endif // GRAPH_H
