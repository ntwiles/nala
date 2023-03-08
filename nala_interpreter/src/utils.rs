use crate::errors::RuntimeError;

pub fn intersection<T: PartialEq + Clone>(lefts: &Vec<T>, rights: &Vec<T>) -> Vec<T> {
    let mut intersection = Vec::<T>::new();

    for left in lefts.iter() {
        if rights.iter().find(|right| *right == left).is_some() {
            intersection.push(left.clone());
        }
    }

    intersection
}

pub fn accept_results<T>(results: Vec<Result<T, RuntimeError>>) -> Result<Vec<T>, RuntimeError> {
    let mut accepted = Vec::<T>::new();

    for result in results {
        match result {
            Ok(value) => accepted.push(value),
            Err(err) => return Err(err),
        }
    }

    Ok(accepted)
}
