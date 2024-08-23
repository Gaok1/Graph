#ifndef STACK_H
#define STACK_H

#include <stdlib.h>
#include <stdbool.h>

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
 */
typedef struct Stack {
    StackNode* top;
    int size;

    void (*push)(struct Stack* self, int data);
    int (*pop)(struct Stack* self);
    bool (*is_empty)(struct Stack* self);
    int (*peek)(struct Stack* self);
} Stack;

void Stack_push(Stack* self, int data);
int Stack_pop(Stack* self);
bool Stack_is_empty(Stack* self);
int Stack_peek(Stack* self);


/* Cria um novo nó */
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

#endif // STACK_H
