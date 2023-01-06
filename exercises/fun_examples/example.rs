fn merge_vecs<'a, 'b>(v1: Vec<&'a i32>, v2: Vec<&'b i32>) -> Vec<&'a i32> {
    v2
}

fn main() {
    let v1 = vec![&1, &2];
    let ans = {
        let v2 = vec![&3, &2];
        merge_vecs(v1, v2)
    };
    println!("{ans:?}");
}
