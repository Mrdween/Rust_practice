use std::fmt;

fn largest<T>(v: &[T]) -> Result<&T, std::io::Error>
where T: PartialOrd { // <- сортировка по типам, подлежащим прроверке
    match v.is_empty() {
        true => Err(std::io::Error::other("список пустует")),
        false => Ok({
            let mut maxi = &v[0];
            for i in 0..v.len() {
                if maxi < &v[i] {
                    maxi = &v[i];
                }
            }
            maxi
        })
    }
}
fn main() {
    let vec = vec![1, 5, 3, 6];
    let result = largest(&vec);

    match result {
        Ok(value) => {
            let text = format!("Из чисел {:?} максимальное - {:?}", vec, value);
            println!("{}", text);
        },
        Err(e) => println!("ужас {}", e),
    }
}

