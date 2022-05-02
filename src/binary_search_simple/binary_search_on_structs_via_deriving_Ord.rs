pub fn find_via_binary_search <T: Ord, V: AsRef<[T]>>(array: V, key: T) -> Option<usize> {
    let array = array.as_ref();
    let mut left_point = 0;
    let mut right_point = array.len();
    while left_point < right_point {
        let mid_point = (left_point + right_point) / 2;
        match array[mid_point].cmp(&key) {
            Ordering::Equal => return Some(mid_point),
            Ordering::Less => left_point = mid_point + 1,
            Ordering::Greater => right_point = mid_point
        }
    }
    None
}
struct Person  {
    id: u32,
    name: String,
    height: u32
}
#[test]
#[ignore]
fn binary_search_within_people_struct() {
    let mut people: Vec<Person> = Vec::new();
    people.push(Person{
        id: 9,
        name: String::from("Shahin"),
        height: 6
    });
    people.push(Person{
        id: 5,
        name: String::from("Asad"),
        height: 4
    });
    people.push(Person{
        id: 3,
        name: String::from("Mitxar"),
        height: 5
    });
    people.push(Person{
        id: 4,
        name: String::from("Shakhla"),
        height: 3
    });
    people.sort();
    println!("{:?}", people);
    assert_eq!(
        find_via_binary_search(
        people,
        Person{
            id: 3,
            name:String::from("Mitxar"),
            height: 5
        }
        ),
    Some(3)
    );
}