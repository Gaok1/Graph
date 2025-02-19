#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>
#include <locale.h>
#include <time.h>
#include <limits.h>

/*
dia 8:
 Por mais que eu ache legal implementar lista ligada. vou preferir representaro grafo por
    matriz de  adjacência pois a pesquisa em aresta (Vi,Vj) é mais eficiente. apesar de gastar ""uns""" MB a mais... :D
        edição dia 12 : meu pc crashou pq rodei em loop sem free...2gb de ram a cada loop
    A matriz de adjacência é uma matriz quadrada de tamanho n x n onde n é o número de vértices do grafo.
    decidir entre a Matriz de adjacência e a Matriz de incidência pois a M de incidência usa bastante espaço para cada aresta
    preferi algo menos variável

    Autoria: Luis Phillip lemos Martins
*/

/*
    (I) grau de saída;
    (II) grau de entrada;
    (III) conjunto de sucessores;
    (IV) conjunto de predecessores.
*/

int get_v_index(int vertice)
{
    return vertice - 1;
}
void vertice_nao_existe()
{
    // printf("Vértice não existe amigo :( \n");
    fflush(stdout);
}

void *func();

typedef struct
    Graph
{
    int v_number;
    bool **matriz; // matriz de adjacência usando byte para menor custo de memória

    // functions
    void (*add_edge)(struct Graph *self, int origin, int destiny);
    bool (*exists_v)(struct Graph *self, int v);
    int (*get_exitDegree)(struct Graph *self, int v);
    int (*get_entryDegree)(struct Graph *self, int v);
    int *(*get_successors)(struct Graph *self, int v);
    int *(*get_predecessors)(struct Graph *self, int v);
    int (*get_max_entry_degree)(struct Graph *self);
    int (*get_min_entry_degree)(struct Graph *self);
    int (*get_max_exit_degree)(struct Graph *self);
    int (*get_min_exit_degree)(struct Graph *self);
} Graph;

void Padd_edge(Graph *self, int origin, int destiny)
{

    int v1_index = get_v_index(origin);
    int v2_index = get_v_index(destiny);

    if (v1_index >= self->v_number || v2_index >= self->v_number)
    {
        perror("Erro ao adicionar aresta, vértice não existe\n");
        exit(1);
    }
    // grafo direcioando apenas
    self->matriz[v1_index][v2_index] = true;
}

bool Pexists_v(Graph *self, int origin)
{
    int v1_index = get_v_index(origin);
    if (v1_index >= self->v_number)
    {
        return false;
    }
    return true;
}

int Pget_exitDegree(Graph *self, int v)
{
    if (!self->exists_v(self, v))
    {
        vertice_nao_existe();
        exit(1);
    }
    int v_index = get_v_index(v);
    int degree = 0;
    for (int i = 0; i < self->v_number; i++)
    {
        if (self->matriz[v_index][i])
        {
            degree++;
        }
    }

    return degree;
}

int Pget_entryDegree(Graph *self, int v)
{
    if (!self->exists_v(self, v))
    {
        vertice_nao_existe();
        exit(1);
    }
    int v_index = get_v_index(v);
    int degree = 0;
    for (int i = 0; i < self->v_number; i++)
    {
        if (self->matriz[i][v_index])
        {
            degree++;
        }
    }
    return degree;
}

int * Pget_successors(Graph *self, int v)
{
    if (!self->exists_v(self, v))
    {
        vertice_nao_existe();
        exit(1);
    }
    int v_index = get_v_index(v);
    int *sucessors = calloc(sizeof(int), self->v_number);
    int sucessors_index = 0;
    for (int i = 0; i < self->v_number; i++)
    {
        if (self->matriz[v_index][i])
        {
            sucessors[sucessors_index++] = i + 1;
        }
    }
    sucessors[sucessors_index++] = -1;
    return realloc(sucessors, sizeof(int) * sucessors_index);
}

int *Pget_predecessors(Graph *self, int v)
{

    if (!self->exists_v(self, v))
    {
        vertice_nao_existe();
        exit(1);
    }

    int v_index = get_v_index(v);
    int *predecessors = calloc(sizeof(int), self->v_number);
    int predecessors_index = 0;

    for (int i = 0; i < self->v_number; i++)
    {
        if (self->matriz[i][v_index])
        {
            predecessors[predecessors_index++] = i + 1;
        }
    }
    predecessors[predecessors_index++] = -1;

    return realloc(predecessors, sizeof(int) * predecessors_index);
}

int Pget_max_entry_degree(Graph *self)
{
    int max = INT_MIN;
    int vertice = INT_MIN;
    for (int i = 0; i < self->v_number; i++)
    {
        int degree = Pget_entryDegree(self, i + 1);
        if (degree > max)
        {
            max = degree;
            vertice = i + 1;
        }
    }
    // printf("Vértice com maior grau de entrada: %d com %d arestas entrando\n", vertice, max);
    return max;
}

int Pget_min_entry_degree(Graph *self)
{
    int min = INT_MAX;
    int vertice = INT_MAX;
    for (int i = 0; i < self->v_number; i++)
    {
        int degree = Pget_entryDegree(self, i + 1);
        if (degree < min)
        {
            min = degree;
            vertice = i + 1;
        }
    }
    // printf("Vértice com menor grau de entrada: %d com %d arestas entrando\n", vertice, min);
    return min;
}

int Pget_max_exit_degree(Graph *self)
{
    int max = INT_MIN;
    int vertice = INT_MIN;
    for (int i = 0; i < self->v_number; i++)
    {
        int degree = Pget_exitDegree(self, i + 1);
        if (degree > max)
        {
            max = degree;
            vertice = i + 1;
        }
    }
    // printf("Vértice com maior grau de saída: %d com %d arestas saindo\n", vertice, max);
    return max;
}

int Pget_min_exit_degree(Graph *self)
{
    int min = INT_MAX;
    int vertice = INT_MAX;
    for (int i = 0; i < self->v_number; i++)
    {
        int degree = Pget_exitDegree(self, i + 1);
        if (degree < min)
        {
            min = degree;
            vertice = i + 1;
        }
    }
    // printf("Vértice com menor grau de saída: %d com %d arestas saindo\n", vertice,min);
    return min;
}

Graph *new_graph(int v_number)
{
    Graph *g = (Graph *)calloc(sizeof(Graph), 1);
    g->v_number = v_number;
    g->matriz = calloc(sizeof(bool *), v_number); // alocando as linhas

    for (int i = 0; i < v_number; i++)
    {
        g->matriz[i] = calloc(sizeof(bool), v_number); // alocando as colunas
    }

    g->add_edge = Padd_edge;
    g->exists_v = Pexists_v;
    g->get_exitDegree = Pget_exitDegree;
    g->get_entryDegree = Pget_entryDegree;
    g->get_successors = Pget_successors;
    g->get_predecessors = Pget_predecessors;
    g->get_max_entry_degree = Pget_max_entry_degree;
    g->get_min_entry_degree = Pget_min_entry_degree;
    g->get_max_exit_degree = Pget_max_exit_degree;
    g->get_min_exit_degree = Pget_min_exit_degree;

    return g;
}

Graph *graph_from_file(FILE *file)
{
    char buffer[100]; // buffer para armazenar a linha

    fgets(buffer, 100, file); // pegando a primeira linha

    int v_number;
    sscanf(buffer, "%d", &v_number); // pegando o número de vértices

    Graph *g = new_graph(v_number); // criando o grafo

    if (g == NULL)
    {
        perror("Erro ao alocar memória\n");
        exit(1);
    }
    long size = v_number;
    // printf("grafo de tamanho %d alocado com Sucesso! :D \nAo todo temos %li bytes\n", v_number, size*size);
    fflush(stdout);

    while (fgets(buffer, sizeof(buffer), file) != NULL)
    {
        int origin = 0, destiny = 0;
        if (sscanf(buffer, "%d %d", &origin, &destiny) == 2)
        {
            g->add_edge(g, origin, destiny);
        }
        else
        {
            // printf(stderr, "Erro ao ler a linha: %s\n", buffer);
        }
    }
    fclose(file);
    return g;
}

FILE *get_file()
{
    char file_name[200] = "graphBig.txt";
    // //printf("Digite o nome do arquivo: ");
    // scanf("%s", file_name);

    FILE *file = fopen(file_name, "r");

    if (file == NULL)
    {
        perror("Erro ao abrir o arquivo\n");
        exit(1);
    }

    // printf("Arquivo aberto com sucesso\n");
    fflush(stdout);
    return file;
}

void show_data_v(int v, Graph *g)
{

    if (!g->exists_v(g, v))
    {
        vertice_nao_existe();
        return;
    }
    printf("\n\n====== Vértice %d ======\n\n", v);

    printf("I) Grau de saída: %d\n", g->get_exitDegree(g, v));

    printf("II) Grau de entrada: %d\n", g->get_entryDegree(g, v));

    printf("III) Conjunto de sucessores:[ ");
    int *sucessors = g->get_successors(g, v);
    for (int i = 0; sucessors[i] != -1; i++)
    {
        printf("%d, ", sucessors[i]);
    }
    printf("]\n");

    printf("IV) Conjunto de predecessores: [ ");
    int *predecessors = g->get_predecessors(g, v);
    for (int i = 0; predecessors[i] != -1; i++)
    {
        printf("%d, ", predecessors[i]);
    }
    printf("]\n");
    free(sucessors);
    free(predecessors);
}

void print_Bonus(Graph *g)
{
    printf("====== Bônus ======\n");
    g->get_max_entry_degree(g);
    g->get_min_entry_degree(g);
    g->get_max_exit_degree(g);
    g->get_min_exit_degree(g);
    // printf("\n\n");
}

int get_theChoosenOne()
{
    int theChoosenOne = 45;
    printf("Digite o vértice para analisarmos!\n");
    scanf("%d", &theChoosenOne);
    return theChoosenOne;
}

int print_menu()
{
    int option;
    printf("====== Menu ======\n");
    printf("1) Digitar o nome do arquivo\n");
    printf("2) Digitar o vértice para análise\n");
    printf("3) Bônus\n");
    printf("4) Sair\n");
    printf("Digite a opção desejada: ");
    scanf("%d", &option);
    return option;
}

int main()
{
    if (setlocale(LC_ALL, "pt_BR.UTF-8") == NULL)
    {
        perror("Erro ao configurar a localidade");
        return 1;
    }
    
    Graph *g = NULL;
    while (true)
    {
        switch (print_menu())
        {
        case 1:
            if(g != NULL){
                free(g);
                g = NULL;
            }
            g = graph_from_file(get_file());
            break;
        case 2:
            int theChoosenOne = get_theChoosenOne();
            show_data_v(theChoosenOne, g);
            break;
        case 3:
            print_Bonus(g);
            break;
        default:
            break;
        }
    }

    return 0;
}