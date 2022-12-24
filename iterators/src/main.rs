fn main() {
    let projects = vec!["Kalbero", "New Project"];

    // iterators are lazy this won't do anything unless we call it
    let _projects_iter_dumb = projects.iter();

    let projects_iter = projects.iter();

    // the iterator takes care of handling an index, incrementing, checking when we are done etc.
    for project in projects_iter {
        println!("Project: {}", project);
    }

    let scores = vec![1, 2, 3];

    // this iterator takes ownership of scores
    // we can't use score outside it so we return our results in a new vec
    let scores_iter = scores.into_iter();

    let mut scores_double = Vec::new();

    for score in scores_iter {
        scores_double.push(score * 2);
    }

    println!("The new scores are {:?}", scores_double);
}

#[test]
fn iterator_demo() {
    let projects = vec!["Kalbero", "New Project"];

    // because next() consumes the iterator the reference should be mutable because the internal state of the iterator will change (to point to the next item)
    let mut projects_iter = projects.iter();

    // all the below are immutable references to the values in the vector
    assert_eq!(projects_iter.next(), Some(&"Kalbero"));
    assert_eq!(projects_iter.next(), Some(&"New Project"));
    assert_eq!(projects_iter.next(), None);
}

#[test]
fn iterator_sum() {
    let scores = vec![2, 4, 6];

    let scores_iter = scores.iter();

    let total: i32 = scores_iter.sum();

    assert_eq!(total, 12);
}

#[test]
fn iterator_map() {
    let scores = vec![2, 4, 6];

    // we leverage a closure passed to the iterator returned by map
    // collect clubs those results into a vec returned to our fn scope into scores_incremented
    let scores_incremented: Vec<_> = scores.iter().map(|x| x + 1).collect();

    assert_eq!(scores_incremented, vec![3, 5, 7]);
}
