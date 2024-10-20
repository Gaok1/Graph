import java.util.Objects;

public class Pair<A, B> {
    public final A first;
    public final B second;

    public Pair(A a, B b) {
        this.first = a;
        this.second = b;
    }

    @Override
    public String toString() {
        return "(" + first + ", " + second + ")";
    }

    @Override
    public boolean equals(Object obj) {
        if (!(obj instanceof Pair)) return false;
        Pair<?, ?> other = (Pair<?, ?>) obj;
        return Objects.equals(this.first, other.first) &&
               Objects.equals(this.second, other.second);
    }

    @Override
    public int hashCode() {
        return Objects.hash(first, second);
    }
}
