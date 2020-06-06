int main() {
    volatile short a = 0;
    volatile short b = 4;

    a = a + b;

    return 0;
}