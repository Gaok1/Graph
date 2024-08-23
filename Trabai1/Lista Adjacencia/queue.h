#ifndef QUEUE_H
#define QUEUE_H

#include <stdlib.h>
#include <stdbool.h>

/* Estrutura do Nó da Fila */
typedef struct QueueNode {
    int data;
    struct QueueNode* next;
} QueueNode;

/* Estrutura da Fila */
typedef struct Queue {
    QueueNode* front;
    QueueNode* rear;
    int size;

    /* Enfileira um elemento na fila
       @param self: a fila
       @param data: o valor do elemento
    */
    void (*enqueue)(struct Queue* self, int data);

    /* Desenfileira o elemento na frente da fila
       @param self: a fila
       @return: o valor do elemento desenfileirado
    */
    int (*dequeue)(struct Queue* self);

    /* Verifica se a fila está vazia
       @param self: a fila
       @return: true se a fila estiver vazia, false caso contrário
    */
    bool (*is_empty)(struct Queue* self);

    /* Obtém o elemento na frente da fila sem removê-lo
       @param self: a fila
       @return: o valor do elemento na frente da fila
    */
    int (*peek)(struct Queue* self);

} Queue;

/* Cria um novo nó */
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

void enqueue(Queue* self, int data);
int dequeue(Queue* self);
bool is_empty(Queue* self);
int peek(Queue* self);

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

bool is_empty(Queue* self) {
    return self->front == NULL;
}

int peek(Queue* self) {
    if (self->is_empty(self)) {
        return -1; // ou outro valor indicativo de erro
    }
    return self->front->data;
}

#endif // QUEUE_H
