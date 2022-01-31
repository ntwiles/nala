enum FooKind {
    Bar,
    Baz
}

const kind = FooKind::Bar;

if (kind == FooKind::Baz) {
    print('should not print');
}

if (kind == FooKind::Bar) {
    print('should print');
}