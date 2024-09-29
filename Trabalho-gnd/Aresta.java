public class Aresta {
    private Vertice v;
    private Vertice w;

    public Aresta(Vertice v, Vertice w) {
        this.v = v;
        this.w = w;
    }

    public Vertice get_vertice_incidente(Vertice vertice) {
        if (vertice == v) {
            return w;
        }else if(vertice == w) {
            return v;
        } else{
            return null;
        }
    }

}