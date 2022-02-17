enum FooKind {
    Bar,
    Baz
}

const kind = FooKind::Bar;

if (kind is FooKind::Baz) {
    print('should not print');
}

if (kind is FooKind::Bar) {
    print('should print');
}