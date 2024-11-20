import java.io.*;
import java.util.*;

public class DualMatrixTransportation {

    public static void main(String[] args) {
        Scanner scanner = new Scanner(System.in);
        System.out.print("Digite o nome do arquivo contendo os dados: ");
        String fileName = scanner.nextLine();

        try {
            // Ler o arquivo
            BufferedReader reader = new BufferedReader(new FileReader(fileName));
            String[] firstLine = reader.readLine().trim().split("\\s+");
            int originalM = Integer.parseInt(firstLine[0]); // Número de ofertas originais
            int originalN = Integer.parseInt(firstLine[1]); // Número de demandas originais

            // Ler as ofertas
            int[] oferta = new int[originalM];
            for (int i = 0; i < originalM; i++) {
                oferta[i] = Integer.parseInt(reader.readLine().trim());
            }

            // Ler as demandas
            int[] demanda = new int[originalN];
            for (int i = 0; i < originalN; i++) {
                demanda[i] = Integer.parseInt(reader.readLine().trim());
            }

            // Ler os custos
            int[][] custo = new int[originalM][originalN];
            for (int i = 0; i < originalM; i++) {
                String[] line = reader.readLine().trim().split("\\s+");
                for (int j = 0; j < originalN; j++) {
                    custo[i][j] = Integer.parseInt(line[j]);
                }
            }
            reader.close();

            // Balancear o problema se necessário
            TransporteData transporteData = balancearProblema(oferta, demanda, custo, originalM, originalN);

            // Resolver o problema usando o método dual
            int[][] transporte = new int[transporteData.oferta.length][transporteData.demanda.length];
            int custoTotal = resolverTransporteDual(transporteData.oferta, transporteData.demanda, transporteData.custo, transporte);

            // Exibir resultados com indicação de fictícios
            System.out.println("\nSolução Ótima:");
            System.out.println("Custo Total: " + custoTotal);
            System.out.println("Quantidades Transportadas:");

            // Preparar rótulos dos destinos
            String[] rotulosDestinos = new String[transporteData.demanda.length];
            for (int j = 0; j < transporteData.demanda.length; j++) {
                if (j < originalN) {
                    rotulosDestinos[j] = "D" + (j + 1);
                } else {
                    rotulosDestinos[j] = "F"; // Destino Fictício
                }
            }

            // Imprimir cabeçalho dos destinos
            System.out.print("\t");
            for (String rotulo : rotulosDestinos) {
                System.out.print(rotulo + "\t");
            }
            System.out.println();

            // Imprimir matriz de transporte com rótulos das origens
            for (int i = 0; i < transporte.length; i++) {
                String rotuloOrigem = (i < originalM) ? "O" + (i + 1) : "F"; // Origem Fictícia
                System.out.print(rotuloOrigem + "\t");
                for (int j = 0; j < transporte[0].length; j++) {
                    System.out.print(transporte[i][j] + "\t");
                }
                System.out.println();
            }

            // Informar sobre destinos ou origens fictícios
            if (transporteData.ficticioDestino) {
                System.out.println("\nDestino Fictício adicionado para balancear o problema.");
            }
            if (transporteData.ficticioOrigem) {
                System.out.println("Origem Fictícia adicionada para balancear o problema.");
            }

        } catch (IOException e) {
            System.out.println("Erro ao ler o arquivo: " + e.getMessage());
        }
    }

    private static TransporteData balancearProblema(int[] oferta, int[] demanda, int[][] custo, int originalM, int originalN) {
        int somaOferta = Arrays.stream(oferta).sum();
        int somaDemanda = Arrays.stream(demanda).sum();

        // Clonar os arrays para evitar modificar os originais
        int[] ofertaBalanceada = Arrays.copyOf(oferta, oferta.length);
        int[] demandaBalanceada = Arrays.copyOf(demanda, demanda.length);
        int[][] custoBalanceado = new int[custo.length][custo[0].length];

        // Clonar a matriz de custos
        for (int i = 0; i < custo.length; i++) {
            custoBalanceado[i] = Arrays.copyOf(custo[i], custo[i].length);
        }

        boolean ficticioDestino = false;
        boolean ficticioOrigem = false;

        if (somaOferta > somaDemanda) {
            // Excesso de oferta: Adicionar destino fictício
            int diferenca = somaOferta - somaDemanda;
            demandaBalanceada = Arrays.copyOf(demandaBalanceada, demandaBalanceada.length + 1);
            demandaBalanceada[demandaBalanceada.length - 1] = diferenca; // Demanda fictícia

            // Adicionar uma nova coluna de custos fictícios (0) para todos os pontos de oferta
            for (int i = 0; i < custoBalanceado.length; i++) {
                custoBalanceado[i] = Arrays.copyOf(custoBalanceado[i], custoBalanceado[i].length + 1);
                custoBalanceado[i][custoBalanceado[i].length - 1] = 0;
            }

            ficticioDestino = true;
            System.out.println("\nProblema desbalanceado detectado (Excesso de oferta). Destino fictício adicionado.");
        } else if (somaDemanda > somaOferta) {
            // Excesso de demanda: Adicionar origem fictícia
            int diferenca = somaDemanda - somaOferta;
            ofertaBalanceada = Arrays.copyOf(ofertaBalanceada, ofertaBalanceada.length + 1);
            ofertaBalanceada[ofertaBalanceada.length - 1] = diferenca; // Oferta fictícia

            // Adicionar uma nova linha de custos fictícios (0) para todos os pontos de demanda
            int[][] novoCusto = new int[custoBalanceado.length + 1][custoBalanceado[0].length];
            for (int i = 0; i < custoBalanceado.length; i++) {
                System.arraycopy(custoBalanceado[i], 0, novoCusto[i], 0, custoBalanceado[i].length);
            }
            // Linha fictícia com custos 0
            for (int j = 0; j < custoBalanceado[0].length; j++) {
                novoCusto[novoCusto.length - 1][j] = 0;
            }
            custoBalanceado = novoCusto;

            ficticioOrigem = true;
            System.out.println("\nProblema desbalanceado detectado (Excesso de demanda). Origem fictícia adicionada.");
        } else {
            System.out.println("\nProblema já está balanceado.");
        }

        return new TransporteData(ofertaBalanceada, demandaBalanceada, custoBalanceado, ficticioDestino, ficticioOrigem);
    }


    private static int resolverTransporteDual(int[] oferta, int[] demanda, int[][] custo, int[][] transporte) {
        int m = oferta.length;
        int n = demanda.length;
        int custoTotal = 0;

        // Inicializar cópias de oferta e demanda
        int[] ofertaRestante = Arrays.copyOf(oferta, m);
        int[] demandaRestante = Arrays.copyOf(demanda, n);

        // Utilizar uma lista de células ordenadas por custo crescente
        List<Cell> cells = new ArrayList<>();
        for (int i = 0; i < m; i++) {
            for (int j = 0; j < n; j++) {
                cells.add(new Cell(i, j, custo[i][j]));
            }
        }
        cells.sort(Comparator.comparingInt(c -> c.custo));

        // Processar transporte
        for (Cell cell : cells) {
            int i = cell.linha;
            int j = cell.coluna;

            if (ofertaRestante[i] > 0 && demandaRestante[j] > 0) {
                int quantidade = Math.min(ofertaRestante[i], demandaRestante[j]);
                transporte[i][j] = quantidade;
                ofertaRestante[i] -= quantidade;
                demandaRestante[j] -= quantidade;
                custoTotal += quantidade * custo[i][j];
            }

            // Se todas as ofertas ou demandas forem atendidas, pode parar
            boolean todasOfertasAtendidas = true;
            for (int o : ofertaRestante) {
                if (o > 0) {
                    todasOfertasAtendidas = false;
                    break;
                }
            }
            boolean todasDemandasAtendidas = true;
            for (int d : demandaRestante) {
                if (d > 0) {
                    todasDemandasAtendidas = false;
                    break;
                }
            }
            if (todasOfertasAtendidas || todasDemandasAtendidas) {
                break;
            }
        }

        return custoTotal;
    }

    private static class Cell {
        int linha;
        int coluna;
        int custo;

        Cell(int linha, int coluna, int custo) {
            this.linha = linha;
            this.coluna = coluna;
            this.custo = custo;
        }
    }

    
    private static class TransporteData {
        int[] oferta;
        int[] demanda;
        int[][] custo;
        boolean ficticioDestino;
        boolean ficticioOrigem;

        TransporteData(int[] oferta, int[] demanda, int[][] custo, boolean ficticioDestino, boolean ficticioOrigem) {
            this.oferta = oferta;
            this.demanda = demanda;
            this.custo = custo;
            this.ficticioDestino = ficticioDestino;
            this.ficticioOrigem = ficticioOrigem;
        }
    }
}
