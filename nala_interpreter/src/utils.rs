pub fn intersection<T: PartialEq + Clone>(lefts: &Vec<T>, rights: &Vec<T>) -> Vec<T> {
    let mut intersection = Vec::<T>::new();

    for left in lefts.iter() {
        if rights.iter().find(|right| *right == left).is_some() {
            intersection.push(left.clone());
        }
    }

    intersection
}
