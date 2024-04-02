// Merge sort overview: Split the collection in half until it's impossible to do it again,
//  then merge the halves back in order.

pub fn merge_sort_sequential<T: PartialOrd + Clone + Default>(collection: &[T]) -> Vec<T> {
    if collection.len() > 1 {
        let (left, right) = collection.split_at(collection.len() / 2);
        let (sorted_left, sorted_right) =
            (merge_sort_sequential(left), merge_sort_sequential(right));
        sorted_merge(sorted_left, sorted_right)
    } else {
        collection.to_vec()
    }
}

fn sorted_merge<T: Default + Clone + PartialOrd>(
    sorted_left: Vec<T>,
    sorted_right: Vec<T>,
) -> Vec<T> {
    let mut result: Vec<T> = vec![Default::default(); sorted_left.len() + sorted_right.len()];

    let (mut i, mut j) = (0, 0);
    let mut k = 0;

    while i < sorted_left.len() && j < sorted_right.len() {
        if sorted_left[i] <= sorted_right[j] {
            result[k] = sorted_left[i].clone();
            i += 1;
        } else {
            result[k] = sorted_right[j].clone();
            j += 1;
        }
        k += 1;
    }

    while i < sorted_left.len() {
        result[k] = sorted_left[i].clone();
        k += 1;
        i += 1;
    }

    while j < sorted_right.len() {
        result[k] = sorted_right[j].clone();
        k += 1;
        j += 1;
    }
    result
}

fn main() {
    let collection = vec![5, 3, 2, 4, 1];
    let sorted_collection = merge_sort_sequential(&collection);
    println!("{:?}", sorted_collection);
}
