fn divide_into_n_strands(lst: &[String], n: usize) -> Vec<Vec<String>> {
    let length = lst.len();
    let sublist_length = length / n as usize;
    let remainder = length % n as usize;

    let mut sublists = Vec::new();
    let mut start = 0;

    for i in 0..n {
        let sublist_size = sublist_length + if i < remainder { 1 } else { 0 };
        let end = start + sublist_size;
        if end > lst.len() {
            break;
        }
        sublists.push(lst[start..end].to_vec());
        start = end;
    }
    sublists
}
