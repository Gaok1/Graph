#include <stdio.h>
#include <stdlib.h>
#include <locale.h>
#include "digraph.h"
#include "dataStructs.h"


int *sortSucessors(int *arr)
{
    int length = 0;
    // Calcula o comprimento do array (terminado por -1)
    for (int i = 0; arr[i] != -1; i++)
    {
        length++;
    }
    // Algoritmo de ordenação Selection Sort
    for (int i = 0; i < length - 1; i++)
    {
        int minIndex = i;
        for (int j = i + 1; j < length; j++)
        {
            if (arr[j] < arr[minIndex])
            {
                minIndex = j;
            }
        }
        // Troca o menor elemento encontrado com o primeiro elemento não ordenado
        if (minIndex != i)
        {
            int temp = arr[i];
            arr[i] = arr[minIndex];
            arr[minIndex] = temp;
        }
    }

    return arr;
}

// Enumeração que define as possíveis classificações de uma aresta
typedef enum
{
    ARVORE,    // Aresta árvore
    RETORNO,   // Aresta de retorno
    AVANCO,    // Aresta de avanço
    CRUZAMENTO // Aresta de cruzamento
} DFS_EDGE_Classification;

/* Estrutura para armazenar classificação de arestas em uma pesquisa em profundiadade
 */
typedef struct DFS_ClassificationEdge
{
    /*
    @brief Identificador da aresta
     */
    int edgeId;
    /*
    @brief tipo Enumerado que define a classificação da aresta
     */
    DFS_EDGE_Classification class;
    /*
    @brief ponteiro para a aresta cuja classificação está sendo armazenada
     */
    Edge *edge;
} DFS_ClassificationEdge;

/** Estrutura para armazenar os dados de uma busca em profundidade

 */
typedef struct DFS_DFS_SearchStruct
{
    /**
    @brief Array para armazenar o tempo de descoberta de cada vértice
    */
    int *TD;
    /**
   @brief Array para armazenar o tempo de término de cada vértice
   */
    int *TT;
    /**
   @brief Array para armazenar pai de cad vértice, armaenando a chave do vértice pai na posição de indexação do vertice filho

   indexamos vertices a partir de `v.key -1`
   */
    int *FATHERS;
    /**
        @brief Array para armazenar a classificação de cada aresta, `indexado` pelo ID da aresta
    */
    DFS_ClassificationEdge *classification;
} DFS_SearchStruct;

/** Função para criar uma nova estrutura de busca em profundidade
    @param g ponteiro para o grafo
    @return ponteiro para a estrutura de busca em profundidade
*/
DFS_SearchStruct *new_DFS_SearchStruct(Graph *g)
{
    DFS_SearchStruct *ss = (DFS_SearchStruct *)malloc(sizeof(DFS_SearchStruct)); // Aloca memória para a estrutura
    ss->TD = calloc(g->v_nums, sizeof(int));                                     // Aloca memória para o tempo de descoberta
    ss->TT = calloc(g->v_nums, sizeof(int));                                     // Aloca memória para o tempo de término
    ss->FATHERS = calloc(g->v_nums, sizeof(int));                                // Aloca memória para os pais dos vértices
    ss->classification = calloc(g->e_nums, sizeof(DFS_ClassificationEdge));      // Aloca memória para a classificação das arestas
    return ss;
}

/**
 * Função para classificar uma aresta
 * @param edge ponteiro para a aresta
 * @param class classificação da aresta
 * @return estrutura de classificação de aresta
 */
DFS_ClassificationEdge classificateEdge(Edge *edge, DFS_EDGE_Classification class)
{
    DFS_ClassificationEdge CE;
    CE.edgeId = edge->id; // Atribui o ID da aresta
    CE.edge = edge;       // Atribui o ponteiro para a aresta
    CE.class = class;     // Atribui a classificação da aresta
    return CE;
}

/* Função para realizar uma busca em profundidade em um grafo
    @param start vértice de origem da busca
    @param g ponteiro para o grafo
    @return ponteiro para a estrutura de dados de uma busca em profundidade
*/
DFS_SearchStruct *BuscaProfundidade(int start, Graph *g)
{
    DFS_SearchStruct *searchData = new_DFS_SearchStruct(g); // Inicializa a estrutura de dados da busca
    int CLOCK = 0;                                          // Contador de tempo
    Stack *stack = new_stack();                             // Cria uma nova pilha
    stack->push(stack, start);                              // Coloca o vértice inicial na pilha
    while (!stack->is_empty(stack))
    {                                                   // Enquanto a pilha não estiver vazia
        bool descobriu_alguem = false;                  // Flag para verificar se um novo vértice foi descoberto
        int v = stack->peek(stack);                     // Obtém o vértice no topo da pilha
        int vertice_index = v - 1;                      // Índice do vértice no array (ajustado para zero-indexado)
        Vertice *vertice = &g->vertices[vertice_index]; // Obtém o vértice correspondente no grafo

        if (searchData->TD[vertice_index] == 0)
        {                                            // Se o vértice ainda não foi descoberto
            searchData->TD[vertice_index] = ++CLOCK; // Marca o tempo de descoberta
        }
        // Obtém e ordena os sucessores do vértice atual
        int *sucessors = sortSucessors(g->get_sucessors(g, vertice->key));
        for (int i = 0; sucessors[i] != -1; i++)
        {
            int sucessor = sucessors[i];
            int sucessor_Index = sucessor - 1;
            Edge *aresta = vertice->get_edge(vertice, sucessor);
            DFS_ClassificationEdge *CE_aresta = &searchData->classification[aresta->id];

            if (aresta == NULL)
            {
                perror("ERRO AO LOCALIZAR ARESTA");
                exit(1);
            }

            if (CE_aresta->edge == NULL)
            {
                if (searchData->TD[sucessor_Index] == 0)
                {
                    searchData->TD[sucessor_Index] = ++CLOCK;
                    *CE_aresta = classificateEdge(aresta, ARVORE);
                    stack->push(stack, sucessor);
                    descobriu_alguem = true;
                    break;
                }
                else if (searchData->TT[sucessor_Index] == 0)
                {
                    *CE_aresta = classificateEdge(aresta, RETORNO);
                }
                else if (searchData->TD[sucessor_Index] < searchData->TD[vertice_index])
                {
                    *CE_aresta = classificateEdge(aresta, CRUZAMENTO);
                }
                else
                {
                    *CE_aresta = classificateEdge(aresta, AVANCO);
                }
            }
        }
        if (!descobriu_alguem)
        {                                            // Se nenhum novo vértice foi descoberto
            searchData->TT[vertice_index] = ++CLOCK; // Marca o tempo de término do vértice
            stack->pop(stack);                       // Remove o vértice da pilha, pois foi totalmente explorado
        }
    }
    
    
    return searchData; // Retorna os dados da busca
}


int main() {
    Graph *g = graph_from_random(100000, 200000,100000 ); // Cria um grafo a partir de um arquivo
    g->explore_all_edges_deep(g, 1);
    while(1);
}

