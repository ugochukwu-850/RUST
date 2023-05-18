/*NOTES:::
1 - Iterators are lazy => not useful unitil call
2- All iterators implement a trait called iterators
//like this
/*
pub trait Iterator {
type Item;
fn next(&mut self) -> Option<Self::Item>;
// methods with default implementations elided
}

 */
3 - Methods that call next are called consuming adaptors, because calling them uses up the
One example is the sum method, which takes ownership of the iterator and iterates
through the items by repeatedly calling next , thus consuming the iterator. As it iterates
through, it adds each item to a running total and returns the total when iteration is complete.

iterator.

4 - Iterator adaptors are methods defined on the Iterator trait that don’t consume the iterator.
Instead, they produce different iterators by changing some aspect of the original iterator.
*/

pub fn sub_main() {
    consuming_adaptor_sum();
}

fn consuming_adaptor_sum() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum();
    assert_eq!(total, 6);
}
#[test]
fn iter_adaptor() {
    let v1: Vec<i32> = vec![1, 2, 3];
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    assert_eq!(v2, vec![2, 3, 4]);
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}
fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];
        let in_my_size = shoes_in_size(shoes, 10);
        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoe {
                    size: 10,
                    style: String::from("boot")
                },
            ]
        );
    }
}


//Removing a clone Using an Iterator
