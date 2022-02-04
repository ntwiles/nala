

func passFoo(fn: Func<String>) {
    fn('foo');
}

passFoo(print);
passFoo(len);