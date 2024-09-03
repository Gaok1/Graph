#ifndef DATASTRUCTS_H
#define DATASTRUCTS_H

#include <stdlib.h>
#include <stdbool.h>
#include <stdio.h>

/*----------------- Fila (Queue) -----------------*/

/* Estrutura do Nó da Fila */
/**
 * @struct QueueNode
 * @brief Representa um nó na fila.
 *
 * @var QueueNode::data
 * O valor do elemento armazenado no nó.
 * 
 * @var QueueNode::next
 * Ponteiro para o próximo nó na fila.
 */
typedef struct QueueNode {
    int data;
    struct QueueNode* next;
} QueueNode;

/* Estrutura da Fila */
/**
 * @struct Queue
 * @brief Representa uma fila com operações associadas.
 *
 * @var Queue::front
 * Ponteiro para o primeiro nó da fila.
 * 
 * @var Queue::rear
 * Ponteiro para o último nó da fila.
 *
 * @var Queue::size
 * O número de elementos na fila.
 *
 * @var Queue::enqueue
 * Ponteiro para a função que enfileira um elemento na fila.
 *
 * @var Queue::dequeue
 * Ponteiro para a função que desenfileira um elemento da fila.
 *
 * @var Queue::is_empty
 * Ponteiro para a função que verifica se a fila está vazia.
 *
 * @var Queue::peek
 * Ponteiro para a função que obtém o elemento na frente da fila sem removê-lo.
 */
typedef struct Queue {
    QueueNode* front;
    QueueNode* rear;
    int size;

    void (*enqueue)(struct Queue* self, int data);
    int (*dequeue)(struct Queue* self);
    bool (*is_empty)(struct Queue* self);
    int (*peek)(struct Queue* self);
} Queue;

/* Declaração das funções associadas à estrutura Queue */
void enqueue(Queue* self, int data);
int dequeue(Queue* self);
bool is_empty(Queue* self);
int peek(Queue* self);
QueueNode* new_Queuenode(int data);
Queue* new_queue();

/*----------------- Pilha (Stack) -----------------*/

/* Estrutura do Nó da Pilha */
/**
 * @struct StackNode
 * @brief Representa um nó na pilha.
 *
 * @var StackNode::data
 * O valor do elemento armazenado no nó.
 * 
 * @var StackNode::next
 * Ponteiro para o próximo nó na pilha.
 */
typedef struct StackNode {
    int data;
    struct StackNode* next;
} StackNode;

/* Estrutura da Pilha */
/**
 * @struct Stack
 * @brief Representa uma pilha com operações associadas.
 *
 * @var Stack::top
 * Ponteiro para o nó do topo da pilha.
 *
 * @var Stack::size
 * O número de elementos na pilha.
 *
 * @var Stack::push
 * Ponteiro para a função que empilha um elemento na pilha.
 *
 * @var Stack::pop
 * Ponteiro para a função que desempilha um elemento da pilha.
 *
 * @var Stack::is_empty
 * Ponteiro para a função que verifica se a pilha está vazia.
 *
 * @var Stack::peek
 * Ponteiro para a função que obtém o elemento no topo da pilha sem removê-lo.
 */
typedef struct Stack {
    StackNode* top;
    int size;

    void (*push)(struct Stack* self, int data);
    int (*pop)(struct Stack* self);
    bool (*is_empty)(struct Stack* self);
    int (*peek)(struct Stack* self);
} Stack;

/* Declaração das funções associadas à estrutura Stack */
void Stack_push(Stack* self, int data);
int Stack_pop(Stack* self);
bool Stack_is_empty(Stack* self);
int Stack_peek(Stack* self);
StackNode* new_StackNode(int data);
Stack* new_stack();

/*----------------- Fila de Prioridade (PriorityQueue) -----------------*/

/* Estrutura do Nó da Fila de Prioridade */
/**
 * @struct PriorityQueueNode
 * @brief Representa um nó na fila de prioridade.
 *
 * @var PriorityQueueNode::key
 * A chave do elemento armazenado no nó.
 * 
 * @var PriorityQueueNode::priority
 * A prioridade do elemento armazenado no nó.
 * 
 * @var PriorityQueueNode::next
 * Ponteiro para o próximo nó na fila de prioridade.
 */
typedef struct PriorityQueueNode {
    int key;
    int priority;
    struct PriorityQueueNode* next;
} PriorityQueueNode;

/* Estrutura da Fila de Prioridade */
/**
 * @struct PriorityQueue
 * @brief Representa uma fila de prioridade com operações associadas.
 *
 * @var PriorityQueue::head
 * Ponteiro para o primeiro nó da fila de prioridade.
 * 
 * @var PriorityQueue::tail
 * Ponteiro para o último nó da fila de prioridade.
 *
 * @var PriorityQueue::size
 * O número de elementos na fila de prioridade.
 *
 * @var PriorityQueue::compare
 * Ponteiro para a função de comparação que determina a prioridade dos elementos.
 *
 * @var PriorityQueue::push
 * Ponteiro para a função que insere um novo elemento na fila de prioridade.
 *
 * @var PriorityQueue::pop
 * Ponteiro para a função que remove o elemento com a maior prioridade.
 *
 * @var PriorityQueue::is_empty
 * Ponteiro para a função que verifica se a fila de prioridade está vazia.
 */
typedef struct PriorityQueue {
    PriorityQueueNode* head;
    PriorityQueueNode* tail;
    int size;

    bool (*compare)(int enqueued_priority, int insert_priority);
    void (*push)(struct PriorityQueue* self, int key, int priority);
    int (*pop)(struct PriorityQueue* self);
    bool (*is_empty)(struct PriorityQueue* self);
} PriorityQueue;

/* Declaração das funções associadas à estrutura PriorityQueue */
void PriorityQueue_push(PriorityQueue* self, int key, int priority);
int PriorityQueue_pop(PriorityQueue* self);
bool PriorityQueue_is_empty(PriorityQueue* self);
PriorityQueueNode* new_PriorityQueuenode(int key, int priority);
PriorityQueue* new_priority_queue(bool (*compare_func)(int, int));

/*----------------- Implementações -----------------*/

/* Implementações das funções associadas à estrutura Queue */

/* Cria um novo nó da fila */
QueueNode* new_Queuenode(int data) {
    QueueNode* queuenode = (QueueNode*)malloc(sizeof(QueueNode));
    if (!queuenode) {
        perror("Falha ao alocar memória para QueueNode");
        exit(EXIT_FAILURE);
    }
    queuenode->data = data;
    queuenode->next = NULL;
    return queuenode;
}

/* Cria uma nova fila vazia */
Queue* new_queue() {
    Queue* queue = (Queue*)malloc(sizeof(Queue));
    if (!queue) {
        perror("Falha ao alocar memória para Queue");
        exit(EXIT_FAILURE);
    }
    queue->front = NULL;
    queue->rear = NULL;
    queue->size = 0;
    queue->enqueue = enqueue;
    queue->dequeue = dequeue;
    queue->is_empty = is_empty;
    queue->peek = peek;
    return queue;
}

/* Enfileira um elemento na fila */
void enqueue(Queue* self, int data) {
    QueueNode* queuenode = new_Queuenode(data);
    if (self->rear == NULL) {
        self->front = queuenode;
        self->rear = queuenode;
    } else {
        self->rear->next = queuenode;
        self->rear = queuenode;
    }
    self->size++;
}

/* Desenfileira o elemento na frente da fila */
int dequeue(Queue* self) {
    if (self->is_empty(self)) {
        return -1; // ou outro valor indicativo de erro
    }

    QueueNode* temp = self->front;
    int data = temp->data;
    self->front = self->front->next;

    if (self->front == NULL) {
        self->rear = NULL;
    }

    free(temp);
    self->size--;
    return data;
}

/* Verifica se a fila está vazia */
bool is_empty(Queue* self) {
    return self->front == NULL;
}

/* Obtém o elemento na frente da fila sem removê-lo */
int peek(Queue* self) {
    if (self->is_empty(self)) {
        return -1; // ou outro valor indicativo de erro
    }
    return self->front->data;
}

/* Implementações das funções associadas à estrutura Stack */

/* Cria um novo nó da pilha */
StackNode* new_StackNode(int data) {
    StackNode* node = (StackNode*)malloc(sizeof(StackNode));
    if (node == NULL) {
        perror("Erro ao alocar memória para o nó da pilha");
        exit(EXIT_FAILURE);
    }
    node->data = data;
    node->next = NULL;
    return node;
}

/* Cria uma nova pilha vazia */
Stack* new_stack() {
    Stack* stack = (Stack*)calloc(1, sizeof(Stack));
    if (stack == NULL) {
        perror("Erro ao alocar memória para a pilha");
        exit(EXIT_FAILURE);
    }
    stack->top = NULL;
    stack->size = 0;
    stack->push = Stack_push;
    stack->pop = Stack_pop;
    stack->is_empty = Stack_is_empty;
    stack->peek = Stack_peek;
    return stack;
}

/* Empilha um elemento na pilha */
void Stack_push(Stack* self, int data) {
    StackNode* node = new_StackNode(data);
    node->next = self->top;
    self->top = node;
    self->size++;
}

/* Desempilha o elemento do topo da pilha */
int Stack_pop(Stack* self) {
    if (self->is_empty(self)) {
        return -1; // ou outro valor indicativo de erro
    }
    StackNode* temp = self->top;
    int popped = temp->data;
    self->top = self->top->next;
    free(temp);
    self->size--;
    return popped;
}

/* Verifica se a pilha está vazia */
bool Stack_is_empty(Stack* self) {
    return self->size == 0;
}

/* Obtém o elemento do topo da pilha sem removê-lo */
int Stack_peek(Stack* self) {
    if (self->is_empty(self)) {
        return -1; // ou outro valor indicativo de erro
    }
    return self->top->data;
}

/* Implementações das funções associadas à estrutura PriorityQueue */

/* Cria um novo nó da fila de prioridade */
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

/* Verifica se a fila de prioridade está vazia */
bool PriorityQueue_is_empty(PriorityQueue* self) {
    return self->size == 0;
}

#endif // DATASTRUCTS_H
