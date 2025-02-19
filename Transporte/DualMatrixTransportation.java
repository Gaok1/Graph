import java.io.*;
import java.util.*;

public class DualMatrixTransportation {

    private static int[][] custo; // Matriz de custos
    private static int[][] transporte; // Matriz de transporte
    private static int[][] custoRelativo; // Matriz de custos relativos
    private static int[] oferta;
    private static int[] demanda;
    private static boolean desbalanceado = false;

    public static void main(String[] args) {
        Scanner scanner = new Scanner(System.in);
        System.out.print("Digite o nome do arquivo contendo os dados: ");
        String fileName = scanner.nextLine();

        try {
            // Ler o arquivo
            BufferedReader reader = new BufferedReader(new FileReader(fileName));
            String[] firstLine = reader.readLine().trim().split("\\s+");
            int originalM = Integer.parseInt(firstLine[0]);
            int originalN = Integer.parseInt(firstLine[1]);

            // Ler as ofertas
            oferta = new int[originalM];
            for (int i = 0; i < originalM; i++) {
                oferta[i] = Integer.parseInt(reader.readLine().trim());
            }

            // Ler as demandas
            demanda = new int[originalN];
            for (int i = 0; i < originalN; i++) {
                demanda[i] = Integer.parseInt(reader.readLine().trim());
            }

            // Ler os custos
            custo = new int[originalM][originalN];
            for (int i = 0; i < originalM; i++) {
                String[] line = reader.readLine().trim().split("\\s+");
                for (int j = 0; j < originalN; j++) {
                    custo[i][j] = Integer.parseInt(line[j]);
                }
            }
            reader.close();

            // Balancear o problema se necessário
            balancearProblema();

            // Resolver usando a estratégia de DualMatrixTransport
            transporte = new int[oferta.length][demanda.length];
            custoRelativo = new int[oferta.length + 1][demanda.length + 1];
            resolverDualMatrixTransport();

            // Calcular e exibir o custo total
            int custoTotal = calcularCustoTotal();
            System.out.println("Custo total de transporte: " + custoTotal);

        } catch (IOException e) {
            System.out.println("Erro ao ler o arquivo: " + e.getMessage());
        }
    }

    private static void balancearProblema() {
        int totalOferta = Arrays.stream(oferta).sum();
        int totalDemanda = Arrays.stream(demanda).sum();

        if (totalOferta > totalDemanda) {
            // Adicionar demanda fictícia
            int diferenca = totalOferta - totalDemanda;
            demanda = Arrays.copyOf(demanda, demanda.length + 1);
            demanda[demanda.length - 1] = diferenca;

            // Adicionar coluna de custos fictícios (0)
            for (int i = 0; i < custo.length; i++) {
                custo[i] = Arrays.copyOf(custo[i], custo[i].length + 1);
                custo[i][custo[i].length - 1] = 0;
            }

            System.out.println("Problema desbalanceado detectado (Excesso de oferta)");
            desbalanceado = true;

        } else if (totalDemanda > totalOferta) {
            // Adicionar oferta fictícia
            int diferenca = totalDemanda - totalOferta;
            oferta = Arrays.copyOf(oferta, oferta.length + 1);
            oferta[oferta.length - 1] = diferenca;

            // Adicionar linha de custos fictícios (0)
            int[][] novoCusto = new int[custo.length + 1][custo[0].length];
            for (int i = 0; i < custo.length; i++) {
                System.arraycopy(custo[i], 0, novoCusto[i], 0, custo[i].length);
            }
            // Linha fictícia com custos 0
            Arrays.fill(novoCusto[novoCusto.length - 1], 0);
            custo = novoCusto;

            System.out.println("Problema desbalanceado detectado (Excesso de demanda)");
            desbalanceado = true;
        }
    }

    private static void resolverDualMatrixTransport() {
        // Alocação inicial
        int total = Arrays.stream(oferta).sum();
        int fluxo = 0;
        int i = 0;

        while (fluxo < total) {
            for (int j = 0; j < demanda.length; j++) {
                if (demanda[j] == 0) continue;

                if (oferta[i] > demanda[j]) {
                    transporte[i][j] = demanda[j];
                    oferta[i] -= demanda[j];
                    fluxo += demanda[j];
                    demanda[j] = 0;
                } else {
                    transporte[i][j] = oferta[i];
                    demanda[j] -= oferta[i];
                    fluxo += oferta[i];
                    oferta[i] = 0;
                    break;
                }
            }
            i++;
        }

        // Refinamento usando matriz de custos relativos
        boolean recalcular;
        do {
            recalcular = recalcularCustosRelativos();
        } while (recalcular);
    }

    private static boolean recalcularCustosRelativos() {
        for (int i = 0; i <= oferta.length; i++) {
            Arrays.fill(custoRelativo[i], 0);
        }

        // Calcular valores não básicos e verificar custos negativos
        boolean negativo = false;
        for (int i = 0; i < oferta.length; i++) {
            for (int j = 0; j < demanda.length; j++) {
                if (transporte[i][j] == 0) {
                    int valor = custo[i][j] - (custoRelativo[oferta.length][j] + custoRelativo[i][demanda.length]);
                    custoRelativo[i][j] = valor;
                    if (valor < 0) negativo = true;
                }
            }
        }

        return negativo;
    }

    private static int calcularCustoTotal() {
        int custoTotal = 0;
        for (int i = 0; i < oferta.length; i++) {
            for (int j = 0; j < demanda.length; j++) {
                custoTotal += transporte[i][j] * custo[i][j];
            }
        }
        return custoTotal;
    }
}
