import java.io.BufferedReader;
import java.io.FileReader;
import java.io.IOException;

public class MatrizAdjacencia {

    static int[][] create_matriz(int ver_num) {
        return new int[ver_num][ver_num];
    }

    static void preenche_matriz(int origem, int dest, int[][] matriz) {
        matriz[origem-1][dest-1] = 1;
    }

    static void get_sucessores(int[][] matriz, int vertice) {
        for (int i = 0; i < matriz[vertice-1].length; i++) {
            if (matriz[vertice-1][i] == 1) {
                System.out.println("Sucessor: " + (i+1));
            } 
        }
    }

    static void get_pre(int[][] matriz, int vertice) {
        for (int i = 0; i < matriz.length; i++) {
            if (matriz[i][vertice-1] == 1) {
                System.out.println("Predecessor: " + (i+1));
            }
        }
    }


    public static void main(String[] args) {
        try (BufferedReader file = new BufferedReader(new FileReader("graph_simple.txt"))) {
            // Leitura do número de vértices
            String line;
            line = file.readLine();
            int ver_num = Integer.parseInt(line.trim().split("\\s+")[0]);
            int[][] matriz = create_matriz(ver_num);

           
            while ((line = file.readLine()) != null) {
                // Dividindo a linha em duas partes: origem e destino
                String[] parts = line.trim().split("\\s+"); // Usa qualquer quantidade de espaços em branco como separador
                int origem = Integer.parseInt(parts[0]);
                int dest = Integer.parseInt(parts[1]);
                
                preenche_matriz(origem, dest, matriz);
                System.out.println("origem: " + origem + " destino: " + dest);

            }

            get_pre(matriz, 1);
            get_sucessores(matriz, 1);

        } catch (IOException e) {
            e.printStackTrace();
        }
    }
}
