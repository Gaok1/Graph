import networkx as nx
import matplotlib.pyplot as plt

# Definir os nós
nodes = ['A', 'B', 'C', 'D', 'E', 'F', 'G']

# Criar um grafo direcionado
graph = nx.DiGraph()
graph.add_nodes_from(nodes)

# Adicionar arestas entre todos os pares de nós
for element in nodes:
    for dest in nodes:
        if element == dest:
            continue
        graph.add_edge(element, dest)

# Definir o tamanho da figura
plt.figure(figsize=(10, 7))

# Personalizações:
node_colors = ['red', 'green', 'blue', 'yellow', 'orange', 'purple', 'pink']  # Cor de cada nó
edge_colors = 'gray'  # Cor das arestas
node_shapes = 'o'  # Formato dos nós ('o' para círculo, 's' para quadrado, '^' para triângulo, etc.)
node_size = 1000  # Tamanho dos nós
font_size = 12  # Tamanho da fonte dos rótulos
font_color = 'Black'  # Cor da fonte dos rótulos
edge_width = 2  # Largura das arestas

# Desenhar o grafo com as personalizações
nx.draw_networkx(
    graph,
    with_labels=True,
    node_color=node_colors,
    edge_color=edge_colors,
    node_shape=node_shapes,
    node_size=node_size,
    font_size=font_size,
    font_color=font_color,
    width=edge_width,
)

# Exibir o grafo
plt.show()
