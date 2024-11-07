import networkx as nx
import matplotlib.pyplot as plt
import pandas as pd
import random

def alternate_marking_dfs(G, start_node):
    marked_nodes = set()
    unmarked_nodes = set()
    stack = [start_node]  # Inicia com o nó inicial
    visited = set()
    count = 0  # Contador para determinar par ou ímpar

    while stack:
        current_node = stack.pop()
        
        # Se o nó já foi visitado, pula para o próximo
        if current_node in visited:
            continue

        # Marca ou desmarca o nó baseado no valor do contador (par ou ímpar)
        if count % 2 == 0:
            marked_nodes.add(current_node)
        else:
            unmarked_nodes.add(current_node)
        
        # Marca o nó como visitado
        visited.add(current_node)
        count += 1  # Incrementa o contador

        # Adiciona os vizinhos na pilha
        for neighbor in G.neighbors(current_node):
            if neighbor not in visited:
                stack.append(neighbor)

    return marked_nodes, unmarked_nodes


def test_alternate_matching_on_random_graphs_with_optimal_debug(n_graphs=10, n_nodes=6, edge_prob=0.5):
    """
    Testa a implementação do emparelhamento em `n_graphs` grafos aleatórios.
    Retorna um DataFrame com o tamanho do emparelhamento máximo encontrado pelo
    algoritmo do usuário e pela solução ótima para cada grafo.
    Inclui desenhos de cada estágio, incluindo o emparelhamento ótimo para debug.
    """
    results = []

    for i in range(n_graphs):
        # Gera um grafo aleatório conexo
        while True:
            G_random = nx.gnp_random_graph(n_nodes, edge_prob)
            if nx.is_connected(G_random):
                break
        
        # Escolhe um nó inicial aleatório
        start_node = random.choice(list(G_random.nodes))
        
        # Etapa 1: Desenha o grafo original
        plt.figure(figsize=(6, 4))
        nx.draw(G_random, with_labels=True, node_color="skyblue", edge_color="gray", node_size=500)
        plt.title(f"Grafo {i+1}: Grafo Original")
        plt.show()
        
        # Executa a marcação alternada de nós
        marked_nodes, unmarked_nodes = alternate_marking_dfs(G_random, start_node)
        
        # Etapa 2: Desenha o grafo com nós marcados e não marcados
        plt.figure(figsize=(6, 4))
        color_map = ["green" if node in marked_nodes else "red" for node in G_random.nodes()]
        nx.draw(G_random, with_labels=True, node_color=color_map, edge_color="gray", node_size=500)
        plt.title(f"Grafo {i+1}: Nós Marcados (verde) e Não Marcados (vermelho)")
        plt.show()

        # Constrói o subgrafo bipartido
        bipartite_edges = [(u, v) for u in marked_nodes for v in G_random.neighbors(u) if v in unmarked_nodes]
        B = nx.Graph()
        B.add_edges_from(bipartite_edges)

        # Etapa 3: Desenha o subgrafo bipartido
        plt.figure(figsize=(6, 4))
        nx.draw(B, with_labels=True, node_color="orange", edge_color="blue", node_size=500)
        plt.title(f"Grafo {i+1}: Subgrafo Bipartido")
        plt.show()

        # Calcula o emparelhamento máximo do subgrafo bipartido (solução do usuário)
        max_matching_user = nx.max_weight_matching(B, maxcardinality=True)

        # Etapa 4: Desenha o emparelhamento máximo encontrado pelo usuário
        plt.figure(figsize=(6, 4))
        matching_edges = list(max_matching_user)
        pos = nx.spring_layout(B)
        nx.draw(B, pos, with_labels=True, node_color="orange", edge_color="blue", node_size=500)
        nx.draw_networkx_edges(B, pos, edgelist=matching_edges, edge_color="purple", width=2)
        plt.title(f"Grafo {i+1}: Emparelhamento Máximo (Usuário)")
        plt.show()

        # Calcula o emparelhamento máximo do grafo completo (solução ótima)
        max_matching_optimal = nx.max_weight_matching(G_random, maxcardinality=True)

        # Etapa 5: Desenha o emparelhamento máximo ótimo no grafo original
        plt.figure(figsize=(6, 4))
        optimal_edges = list(max_matching_optimal)
        pos = nx.spring_layout(G_random)
        nx.draw(G_random, pos, with_labels=True, node_color="skyblue", edge_color="gray", node_size=500)
        nx.draw_networkx_edges(G_random, pos, edgelist=optimal_edges, edge_color="red", width=2)
        plt.title(f"Grafo {i+1}: Emparelhamento Máximo Ótimo")
        plt.show()

        # Registra o resultado
        results.append({
            "Grafo": f"Grafo_{i+1}",
            "Emparelhamento Usuário": len(max_matching_user),
            "Emparelhamento Ótimo": len(max_matching_optimal)
        })

    # Converte os resultados em um DataFrame
    df_results = pd.DataFrame(results)
    return df_results

# Testa a função para 10 grafos para visualização
df_results_optimal_debug = test_alternate_matching_on_random_graphs_with_optimal_debug(n_graphs=30, n_nodes=6, edge_prob=0.5)

# Exibe os resultados
print("Resultado de Emparelhamentos com Debug Completo (Ótimo e Usuário):")
print(df_results_optimal_debug)
