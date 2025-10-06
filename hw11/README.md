Напишем что-то очень похожее на декораторы из питона.

Допустим, мы пишем библиотеку для трассировки программ и хотим измерять время работы функций.

Для этого требуется написать функцию measure_time, которая принимает любую функцию F, аргументы и возвращает результат работы F и время работы F.

Пример использования:

// fn measure_time - требуется реализовать

```rust
fn sum(a: u64, b: u64) -> u64 {
    a + b
}

fn concat(a: &[&str]) -> String {
    a.join()
}

fn main() {
    let (res, execution_time) = measure_time(|(a, b)| sum(a, b), (10, 20));
    assert_eq!(30, res);
    println!(sum took {execution_time:?});

    let (res, execution_time) = measure_time(|strs| concat(strs), &[hello, world]);
    assert_eq!(hello world, res);
    println!(concat took {execution_time:?});

    let (_, execution_time) = measure_time(
        |_| {
            println!(printing to stdout);
        },
        (),
    );
    println!(printing took {execution_time:?});
}