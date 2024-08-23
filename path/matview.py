import matplotlib.pyplot as plt
import networkx as nx
from matplotlib.animation import FuncAnimation, FFMpegWriter
import pydot

def read_graph_edges(file_path):
    edges = []
    with open(file_path, 'r') as file:
        file.readline()
        lines = file.readlines()
        for line in lines:
            line = line.strip()
            nodes = line.split()
            if len(nodes) == 2:
                edges.append((int(nodes[0]), int(nodes[1])))
    return edges

def read_generalized_search(file_path):
    tree_edges = []
    back_edges = []
    forward_edges = []
    cross_edges = []
    root_vertices = []
    explored_nodes = set()
    events = []
    with open(file_path, 'r') as file:
        lines = file.readlines()
        for line in lines:
            line = line.strip()
            parts = line.split()
            if parts[0] == 'Root:':
                root_vertices.append(int(parts[1]))
                events.append(('root', int(parts[1])))
            elif parts[0] == 'Aresta':
                aresta_tipo = parts[1]
                src = int(parts[2])
                dest = int(parts[4])
                if aresta_tipo == 'Arvore:':
                    tree_edges.append((src, dest))
                    events.append(('tree', src, dest))
                elif aresta_tipo == 'Retorno:':
                    back_edges.append((src, dest))
                    events.append(('back', src, dest))
                elif aresta_tipo == 'Avanco:':
                    forward_edges.append((src, dest))
                    events.append(('forward', src, dest))
                elif aresta_tipo == 'Cruzamento:':
                    cross_edges.append((src, dest))
                    events.append(('cross', src, dest))
            elif parts[0] == 'explored:':
                explored_nodes.add(int(parts[1]))
                events.append(('explored', int(parts[1])))
    return tree_edges, back_edges, forward_edges, cross_edges, root_vertices, explored_nodes, events

def draw_search_tree(G, tree_edges, back_edges, forward_edges, cross_edges, root_vertices, title, show_back_edges, show_forward_edges, show_cross_edges, output_file=None):
    fig, ax = plt.subplots(figsize=(14, 10))

    # Criação de um subgrafo contendo apenas a árvore de busca
    tree_subgraph = nx.DiGraph()
    tree_subgraph.add_edges_from(tree_edges)

    # Calcular a posição dos nós apenas com base nas arestas de árvore
    pos = nx.nx_pydot.graphviz_layout(tree_subgraph, prog='dot')

    # Marcar nós raiz
    visited_nodes = {}
    for node in G.nodes():
        if node in root_vertices:
            visited_nodes[node] = 'orange'
        else:
            visited_nodes[node] = 'lightgray'
    
    colors = [visited_nodes.get(node, 'lightgray') for node in tree_subgraph.nodes()]
    
    # Desenhar o subgrafo da árvore de busca
    nx.draw(tree_subgraph, pos, ax=ax, with_labels=False, node_color=colors, node_size=700)
    
    # Desenhar rótulos com a cor correta
    labels = {node: str(node) for node in tree_subgraph.nodes()}
    font_colors = {node: 'black' for node in tree_subgraph.nodes()}
    
    for node, (x, y) in pos.items():
        ax.text(x, y, labels[node], fontsize=10, ha='center', va='center', color=font_colors[node])

    # Desenhar arestas de árvore
    nx.draw_networkx_edges(tree_subgraph, pos, edgelist=tree_edges, ax=ax, edge_color='blue', width=2, style='solid', arrows=True)

    # Desenhar arestas de retorno, avanço e cruzamento como "enfeites"
    if show_back_edges:
        nx.draw_networkx_edges(G, pos, edgelist=back_edges, ax=ax, edge_color='red', width=2, style='dotted', arrows=True)
    if show_forward_edges:
        nx.draw_networkx_edges(G, pos, edgelist=forward_edges, ax=ax, edge_color='green', width=2, style='dotted', arrows=True)
    if show_cross_edges:
        nx.draw_networkx_edges(G, pos, edgelist=cross_edges, ax=ax, edge_color='purple', width=2, style='dotted', arrows=True)

    # Legenda
    handles = [
        plt.Line2D([0], [0], marker='o', color='w', markerfacecolor='lightgray', markersize=10, label='Não visitado'),
        plt.Line2D([0], [0], marker='o', color='w', markerfacecolor='orange', markersize=10, label='Raiz'),
        plt.Line2D([0], [0], color='blue', lw=2, label='Aresta de árvore', linestyle='solid')
    ]
    if show_back_edges:
        handles.append(plt.Line2D([0], [0], color='red', lw=2, label='Aresta de retorno', linestyle='dotted'))
    if show_forward_edges:
        handles.append(plt.Line2D([0], [0], color='green', lw=2, label='Aresta de avanço', linestyle='dotted'))
    if show_cross_edges:
        handles.append(plt.Line2D([0], [0], color='purple', lw=2, label='Aresta de cruzamento', linestyle='dotted'))

    ax.legend(handles=handles, loc='upper left', bbox_to_anchor=(-0.1, 1), borderaxespad=0.)

    ax.set_title(title)
    
    if output_file:
        plt.savefig(output_file)
    plt.show()

def draw_final_graph(G, pos, tree_edges, back_edges, forward_edges, cross_edges, root_vertices, explored_nodes, title, output_file=None):
    fig, ax = plt.subplots(figsize=(14, 10))
    visited_nodes = {}
    for node in G.nodes():
        if node in explored_nodes:
            visited_nodes[node] = 'black'
        elif node in root_vertices:
            visited_nodes[node] = 'orange'
        else:
            visited_nodes[node] = 'lightgray'
    
    colors = [visited_nodes.get(node, 'lightgray') for node in G.nodes()]
    
    # Desenhar o grafo
    nx.draw(G, pos, ax=ax, with_labels=False, node_color=colors, node_size=700)

    # Desenhar rótulos com a cor correta
    labels = {node: str(node) for node in G.nodes()}
    font_colors = {node: 'white' if visited_nodes[node] == 'black' else 'black' for node in G.nodes()}
    
    for node, (x, y) in pos.items():
        ax.text(x, y, labels[node], fontsize=10, ha='center', va='center', color=font_colors[node])

    # Desenhar arestas de árvore
    nx.draw_networkx_edges(G, pos, edgelist=tree_edges, ax=ax, edge_color='blue', width=2, style='solid', arrows=True)

    # Desenhar arestas de retorno
    nx.draw_networkx_edges(G, pos, edgelist=back_edges, ax=ax, edge_color='red', width=2, style='dotted', arrows=True)

    # Desenhar arestas de avanço
    nx.draw_networkx_edges(G, pos, edgelist=forward_edges, ax=ax, edge_color='green', width=2, style='dotted', arrows=True)

    # Desenhar arestas de cruzamento
    nx.draw_networkx_edges(G, pos, edgelist=cross_edges, ax=ax, edge_color='purple', width=2, style='dotted', arrows=True)

    # Legenda
    handles = [
        plt.Line2D([0], [0], marker='o', color='w', markerfacecolor='lightgray', markersize=10, label='Não visitado'),
        plt.Line2D([0], [0], marker='o', color='w', markerfacecolor='orange', markersize=10, label='Raiz'),
        plt.Line2D([0], [0], marker='o', color='w', markerfacecolor='black', markersize=10, label='Explorado'),
        plt.Line2D([0], [0], color='blue', lw=2, label='Aresta de árvore', linestyle='solid'),
        plt.Line2D([0], [0], color='red', lw=2, label='Aresta de retorno', linestyle='dotted'),
        plt.Line2D([0], [0], color='green', lw=2, label='Aresta de avanço', linestyle='dotted'),
        plt.Line2D([0], [0], color='purple', lw=2, label='Aresta de cruzamento', linestyle='dotted')
    ]
    ax.legend(handles=handles, loc='upper left', bbox_to_anchor=(-0.1, 1), borderaxespad=0.)

    ax.set_title(title)
    
    if output_file:
        plt.savefig(output_file)
    plt.show()

def animate_path(G, pos, tree_edges, back_edges, forward_edges, cross_edges, root_vertices, events, title, output_file=None, save_video=False, interval=500):
    fig, ax = plt.subplots(figsize=(14, 10))
    visited_edges = []
    visited_nodes = {}
    visited_edges_colors = []
    visited_edges_styles = []
    def init():
        nx.draw(G, pos, ax=ax, with_labels=False, node_color='lightgray', node_size=700)
        labels = {node: str(node) for node in G.nodes()}
        for node, (x, y) in pos.items():
            ax.text(x, y, labels[node], fontsize=10, ha='center', va='center', color='black')
        ax.set_title(title)
        handles = [
            plt.Line2D([0], [0], marker='o', color='w', markerfacecolor='lightgray', markersize=10, label='Não visitado'),
            plt.Line2D([0], [0], marker='o', color='w', markerfacecolor='gray', markersize=10, label='Visitado'),
            plt.Line2D([0], [0], marker='o', color='w', markerfacecolor='black', markersize=10, label='Explorado'),
            plt.Line2D([0], [0], color='gray', lw=2, label='Aresta não visitada'),
            plt.Line2D([0], [0], color='blue', lw=2, label='Aresta de árvore', linestyle='-'),
            plt.Line2D([0], [0], color='red', lw=2, label='Aresta de retorno', linestyle='dotted'),
            plt.Line2D([0], [0], color='green', lw=2, label='Aresta de avanço', linestyle='dotted'),
            plt.Line2D([0], [0], color='purple', lw=2, label='Aresta de cruzamento', linestyle='dotted')
        ]
        ax.legend(handles=handles, loc='upper left', bbox_to_anchor=(-0.1, 1), borderaxespad=0.)
        return ax,
    def update(num):
        ax.clear()
        if num < len(events):
            event = events[num]
            if event[0] == 'root':
                visited_nodes[event[1]] = 'orange'
            elif event[0] == 'tree':
                src, dest = event[1], event[2]
                if src not in visited_nodes:
                    visited_nodes[src] = 'gray'
                if dest not in visited_nodes:
                    visited_nodes[dest] = 'gray'
                visited_edges.append((src, dest))
                visited_edges_colors.append('blue')
                visited_edges_styles.append('solid')
            elif event[0] == 'back':
                src, dest = event[1], event[2]
                visited_nodes[src] = 'black'
                visited_nodes[dest] = 'black'
                visited_edges.append((src, dest))
                visited_edges_colors.append('red')
                visited_edges_styles.append('dotted')
            elif event[0] == 'forward':
                src, dest = event[1], event[2]
                visited_edges.append((src, dest))
                visited_edges_colors.append('green')
                visited_edges_styles.append('dotted')
            elif event[0] == 'cross':
                src, dest = event[1], event[2]
                visited_edges.append((src, dest))
                visited_edges_colors.append('purple')
                visited_edges_styles.append('dotted')
            elif event[0] == 'explored':
                visited_nodes[event[1]] = 'black'
        colors = [visited_nodes.get(node, 'lightgray') for node in G.nodes()]
        nx.draw(G, pos, ax=ax, with_labels=False, node_color=colors, node_size=700)
        labels = {node: str(node) for node in G.nodes()}
        font_colors = {node: 'white' if visited_nodes.get(node) == 'black' else 'black' for node in G.nodes()}
        for node, (x, y) in pos.items():
            ax.text(x, y, labels[node], fontsize=10, ha='center', va='center', color=font_colors[node])
        for i, (edge, color, style) in enumerate(zip(visited_edges, visited_edges_colors, visited_edges_styles)):
            nx.draw_networkx_edges(G, pos, edgelist=[edge], ax=ax, edge_color=color, width=2, style=style, arrows=True)
        ax.set_title(title)
    ani = FuncAnimation(fig, update, frames=len(events), init_func=init, interval=interval, repeat=False)
    if save_video and output_file:
        writer = FFMpegWriter(fps=10, metadata=dict(artist='Me'), bitrate=5000)
        ani.save(output_file, writer=writer)
    else:
        plt.show()

def create_graph(graph_file, search_file, output_file=None):
    try:
        edges = read_graph_edges(graph_file)
        tree_edges, back_edges, forward_edges, cross_edges, root_vertices, explored_nodes, events = read_generalized_search(search_file)
        G = nx.DiGraph()
        G.add_edges_from(edges)
        print("Escolha uma opção:")
        print("1. Mostrar animação")
        print("2. Mostrar resultado final pronto")
        print("3. Mostrar árvore de busca")
        choice = input("Sua escolha: ").strip()
        if choice == "1":
            print("Escolha a velocidade da animação:")
            print("1. Baixo")
            print("2. Médio")
            print("3. Alto")
            speed_choice = input("Sua escolha: ").strip()
            if speed_choice == "1":
                interval = 1000
            elif speed_choice == "2":
                interval = 500
            elif speed_choice == "3":
                interval = 200
            else:
                interval = 500
            pos = nx.spring_layout(G)
            animate_path(G, pos, tree_edges, back_edges, forward_edges, cross_edges, root_vertices, events, "Busca em Profundidade", output_file=output_file, interval=interval)
        elif choice == "2":
            pos = nx.spring_layout(G)
            draw_final_graph(G, pos, tree_edges, back_edges, forward_edges, cross_edges, root_vertices, explored_nodes, "Busca em Profundidade - Estado Final", output_file=output_file)
        elif choice == "3":
            print("Escolha quais arestas mostrar na árvore de busca:")
            print("1. Apenas arestas de árvore")
            print("2. Arestas de árvore e de retorno")
            print("3. Arestas de árvore, de retorno e de avanço")
            print("4. Mostrar todas as arestas")
            edge_choice = input("Sua escolha: ").strip()
            show_back_edges = show_forward_edges = show_cross_edges = False
            if edge_choice == "2":
                show_back_edges = True
            elif edge_choice == "3":
                show_back_edges = True
                show_forward_edges = True
            elif edge_choice == "4":
                show_back_edges = True
                show_forward_edges = True
                show_cross_edges = True
            draw_search_tree(G, tree_edges, back_edges, forward_edges, cross_edges, root_vertices, "Árvore de Busca", show_back_edges, show_forward_edges, show_cross_edges, output_file=output_file)
        else:
            print("Opção inválida. Saindo...")
    except Exception as e:
        print(f"An error occurred: {e}")

create_graph('output/graph.txt', 'output/busca_geral.txt', output_file='final_graph.png')
