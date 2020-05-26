use std::thread;
use std::time::Duration;
use std::collections::HashMap;

struct Cacher<T, U, V> 
where
    U: std::cmp::Eq + std::hash::Hash + Copy,
    T: Fn(U) -> V,
    V: Copy
{
    calculation: T,
    cache_map: HashMap<U, V>,

}

impl<T, U, V> Cacher<T, U, V> 
    where T: Fn(U) -> V,
    U: std::cmp::Eq + std::hash::Hash + Copy,
    V: Copy
{
    fn new(calculation: T) ->   Cacher<T, U, V>{
        Cacher{
            calculation,
            cache_map: HashMap::new(),
        }
    }

    fn value(&mut self, arg: U) -> V{
        let v = self.cache_map.get(&arg);
        match v {
            Some(n) => *n,
            None => {
                let n = (self.calculation)(arg);
                // integer get passed as copy since they implement Copy trait.
                // If we were storing diff types, we might have to store references.
                self.cache_map.insert(arg, n);
                n
            }
        }
    }
}

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("Running expensive calculation...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn generate_workout(intensity: u32, random_number: u32){

    // let expensive_result = simulated_expensive_calculation(intensity);
    let expensive_closure = |intensity| {
        println!("Running expensive calculation closure...");
        thread::sleep(Duration::from_secs(2));
        intensity
    };

    let mut cache = Cacher::new(expensive_closure);

    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            cache.value(intensity)
        );
        println!(
            "Next, do {} situps!",
            cache.value(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                cache.value(intensity)
            );
        }
    }
}

#[derive(PartialEq, Debug)]
struct Shoe{
    size: u32,
    style: String,
}

struct Counter{
    count: u32,
}

impl Counter{
    fn new() -> Counter{
        Counter{count: 0}
    }
}

impl Iterator for Counter{
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item>{
        if self.count < 5{
            self.count += 1;
            return Some(self.count);
        }else{
            return None;
        }

    }
}

fn shoes_in_my_size(shoes: Vec<Shoe>, size: u32) -> Vec<Shoe>{
    shoes.into_iter().filter(|shoe| {shoe.size == size}).collect()
}

fn main() {
    let x = |c| {c};
    let foo = x(String::from("foo"));
    // below will error because we first called closure with string. So it assumes data type of x is string.
    // let one = x(1);

    let user_specified_value = 10;
    let random_num = 7;

    generate_workout(user_specified_value, random_num);
    println!("Done!");

    println!("Iterators..");
    let v1: Vec<i32> = vec![1, 2, 3];
    let vp1 = v1.iter().map(|x| x + 1);
    for i in vp1 {
        println!("iter: {:?}", i);
    }

}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn call_with_different_values(){
        let mut cache = Cacher::new(|a| a);
        let v1 = cache.value(4);
        let v2 = cache.value(5);
        assert_eq!(v2, 5);
    }

    #[test]
    fn call_with_string_values(){
        let mut cache = Cacher::new(|a| a);
        let v1 = cache.value("four");
        let v2 = cache.value("five");
        assert_eq!(v2, "five");
    }

    #[test]
    fn filters_by_size(){
        let shoes = vec![
            Shoe{size: 1, style: String::from("white")},
            Shoe{size: 1, style: String::from("black")},
            Shoe{size: 2, style: String::from("orange")},
            Shoe{size: 3, style: String::from("blue")},
        ];

        let exp  = vec![
            Shoe{size: 1, style: String::from("white")},
            Shoe{size: 1, style: String::from("black")},
        ];

        let recv = shoes_in_my_size(shoes, 1);
        assert_eq!(exp, recv);
        // below will error because we gave ownership to shoes_in_my_size.
        // for shoe in shoes.iter(){
        //     println!("{:?}", *shoe);
        // }
    }

    #[test]
    fn calling_next_directly(){
        let mut counter = Counter::new();
        assert_eq!(Some(1), counter.next());
        assert_eq!(Some(2), counter.next());
        assert_eq!(Some(3), counter.next());
        assert_eq!(Some(4), counter.next());
        assert_eq!(Some(5), counter.next());
        assert_eq!(None, counter.next());
    }

    #[test]
    fn other_iter_methods_on_counter(){
        let mut counter = Counter::new();
        let s:u32 = counter.zip(Counter::new().skip(1))
            .map(|(x, y)| {x + y})
            .filter(|x| {x % 3 == 0})
            .sum();
        assert_eq!(12, s);
    }
}
