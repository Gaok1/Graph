#include "graph.h"
#include "stack.h"
#include <stdio.h>
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>

void deep(Graph * g, int start){
    int * NASCIMENTO = (int *)calloc(g->v_nums, sizeof(int));
    int * MORTE = (int *)calloc(g->v_nums, sizeof(int));
    int * FATHER = (int *)calloc(g->v_nums, sizeof(int));
    int CLOCK = 0;
    Stack * stack = new_stack();
    stack->push(stack, start);

    while(!stack->is_empty(stack)){
         
        int v = stack->peek(stack);
        int v_index = v-1;
        Vertice * vertice = &g->vertices[v_index];
         
        if(NASCIMENTO[v_index] == 0){
            NASCIMENTO[v_index] = ++CLOCK;
            printf("Nascimento %d as %d\n", v, NASCIMENTO[v_index]);
        }
        int discovered = 0;
      
        for(int i = 0; vertice->sucessors[i] != -1  ; i++){
            int sucessor = vertice->sucessors[i];
            int sucessor_index = sucessor-1;
            if(NASCIMENTO[sucessor_index] == 0){
                printf("Nascimento %d as %d\n", v, NASCIMENTO[v_index]);
                NASCIMENTO[sucessor_index] = ++CLOCK;
               // printf("Aresta arvore\n");
                FATHER[sucessor_index] = v;
                stack->push(stack, sucessor);
                discovered ++;
                break;
            }else if(MORTE[sucessor_index] == 0 && FATHER[sucessor_index] != v){
               // printf("Aresta não arvore\n");
            }
        }
        if(discovered == 0){ //acabou de explorar
            MORTE[v_index] = ++CLOCK;
            printf("Morte %d as %d\n", v, MORTE[v_index]);
            stack->pop(stack);
        }
    }
}


int main()
{
    Graph *g = graph_from_file(fopen("graph.txt", "r"));
   
    deep(g,1);
    
    return 0;
}
