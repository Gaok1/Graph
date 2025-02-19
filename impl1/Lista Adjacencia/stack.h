#ifndef STACK_H
#define STACK_H

#include <stdlib.h>
#include <stdbool.h>

/* Estrutura do Nó da Pilha */
typedef struct StackNodeNode {
    int data;
    struct StackNodeNode* next;
} StackNodeNode;

/* Estrutura da Pilha */
typedef struct Stack {
    StackNodeNode* top;
    int size;

    /* Empilha um elemento na pilha
       @param self: a pilha
       @param data: o valor do elemento
    */
    void (*push)(struct Stack* self, int data);

    /* Desempilha o elemento do topo da pilha
       @param self: a pilha
       @return: o valor do elemento desempilhado
    */
    int (*pop)(struct Stack* self);

    /* Verifica se a pilha está vazia
       @param self: a pilha
       @return: true se a pilha estiver vazia, false caso contrário
    */
    bool (*Stack_is_empty)(struct Stack* self);

    /* Obtém o elemento do topo da pilha sem removê-lo
       @param self: a pilha
       @return: o valor do elemento no topo da pilha
    */
    int (*peek)(struct Stack* self);

    /* Verifica se a pilha está vazia
       @param self: a pilha
       @return: true se a pilha estiver vazia, false caso contrário
    */
    bool (*is_empty)(struct Stack* self);

} Stack;



/* Cria um novo nó */
StackNodeNode* new_StackNodenode(int data) {
    StackNodeNode* StackNodenode = (StackNodeNode*)malloc(sizeof(StackNodeNode));
    StackNodenode->data = data;
    StackNodenode->next = NULL;
    return StackNodenode;
}

void Stack_push(Stack* self, int data);
int Stack_pop(Stack* self);
bool Stack_is_empty(Stack* self);
int Stack_peek(Stack* self);


/* Cria uma nova pilha vazia */
Stack* new_stack() {
    Stack* stack = (Stack*)calloc(sizeof(Stack),1);
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
    StackNodeNode* StackNodenode = new_StackNodenode(data);
    StackNodenode->next = self->top;
    self->top = StackNodenode;
    self->size++;
}

/* Desempilha o elemento do topo da pilha */
int Stack_pop(Stack* self) {
    if (self->is_empty(self)) {
        return -1; // ou outro valor indicativo de erro
    }
    fflush(stdout);
    StackNodeNode* temp = self->top;
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
    if (self->Stack_is_empty(self)) {
        return -1; // ou outro valor indicativo de erro
    }
    return self->top->data;
}

#endif // STAC