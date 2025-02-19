#ifndef PRIORITYQUEUE_H
#define PRIORITYQUEUE_H

#include <stdlib.h>
#include <stdbool.h>

typedef struct PriorityQueueNode {
    int key;
    int priority;
    struct PriorityQueueNode* next;
} PriorityQueueNode;


/* Estrutura da Fila de Prioridade 
    @param head: ponteiro para o primeiro nó
    @param tail: ponteiro para o último nó
    @param size: tamanho da fila

    @param compare: função de comparação para determinar a prioridade
    @param push: função para inserir um novo elemento na fila
*/
typedef struct PriorityQueue {
    PriorityQueueNode* head;
    PriorityQueueNode* tail;
    int size;
    
    /* Função de comparação para determinar a prioridade, irá inserir quando der false
       Retorna true se o primeiro elemento tiver prioridade maior que o segundo.
       @param a: primeiro elemento -> elemento atual
       @param b: segundo elemento -> elemento a ser comparado/inserido na fila
       @return: true se a prioridade de enqueued_priority for maior que a de insert_priority, caso contrário false
    */
    bool (*compare)(int enqueued_priority, int insert_priority);

    /* Pushes a new element to the priority queue using the given key and priority and function compare
       @param self: the priority queue
       @param key: the key of the element
       @param priority: the priority of the element
    */
    void (*push)(struct PriorityQueue* self, int key, int priority);

    /* Pops the element with the highest priority
       @param self: the priority queue
       @return key: the key of the element
    */
    int (*pop)(struct PriorityQueue* self);
    /* Verifica se a fila de prioridade está vazia
       @param self: a fila de prioridade
       @return: true se a fila de prioridade estiver vazia, caso contrário false
    */
    bool (*is_empty)(struct PriorityQueue* self);
} PriorityQueue;

void PriorityQueue_push(PriorityQueue* self, int key, int priority);
int PriorityQueue_pop(PriorityQueue* self);
bool PriorityQueue_is_empty(PriorityQueue* self);

/* Cria um novo nó */
PriorityQueueNode* new_PriorityQueuenode(int key, int priority) {
    PriorityQueueNode* new_PriorityQueuenode = (PriorityQueueNode*)malloc(sizeof(PriorityQueueNode));
    new_PriorityQueuenode->key = key;
    new_PriorityQueuenode->priority = priority;
    new_PriorityQueuenode->next = NULL;
    return new_PriorityQueuenode;
}

/* Cria uma nova fila de prioridade vazia */
PriorityQueue* new_priority_queue(bool (*compare_func)(int, int)) {
    PriorityQueue* pq = (PriorityQueue*)malloc(sizeof(PriorityQueue));
    pq->head = NULL;
    pq->tail = NULL;
    pq->size = 0;
    pq->compare = compare_func;
    pq->push = PriorityQueue_push;
    pq->pop = PriorityQueue_pop;
    pq->is_empty = PriorityQueue_is_empty;
    return pq;
}

/* Insere um novo elemento na fila de prioridade */
void PriorityQueue_push(PriorityQueue* self, int key, int priority) {
    PriorityQueueNode* newPriorityQueueNode = new_PriorityQueuenode(key, priority);

    if (self->size == 0) {
        self->head = newPriorityQueueNode;
        self->tail = newPriorityQueueNode;
    } else {
        PriorityQueueNode* current = self->head;
        PriorityQueueNode* prev = NULL;
        while (current != NULL && self->compare(current->priority, priority)) {
            prev = current;
            current = current->next;
        }
        if (prev == NULL) {
            newPriorityQueueNode->next = self->head;
            self->head = newPriorityQueueNode;
        } else if (current == NULL) {
            self->tail->next = newPriorityQueueNode;
            self->tail = newPriorityQueueNode;
        } else {
            prev->next = newPriorityQueueNode;
            newPriorityQueueNode->next = current;
        }
    }

    self->size++;
}

/* Remove o elemento com a maior prioridade */
int PriorityQueue_pop(PriorityQueue* self) {
    if (self->size == 0) {
        return -1; // ou outro valor indicativo de erro
    }

    PriorityQueueNode* temp = self->head;
    int key = temp->key;
    
    self->head = self->head->next;
    if (self->head == NULL) {
        self->tail = NULL;
    }
    free(temp);
    self->size--;
    return key;
}

bool PriorityQueue_is_empty(PriorityQueue* self) {
    return self->size == 0;
}

#endif // PRIORITYQUEUE_H
