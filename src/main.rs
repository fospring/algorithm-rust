use std::collections::VecDeque;

/// 发牌堆
/// 输入slice索引越小则在牌堆中的位置越高
fn sort_dispatch(arr: &[u32]) -> Vec<u32> {
    let mut res = Vec::with_capacity(arr.len());
    let mut queue: VecDeque<_> = arr.iter().map(|elem|*elem).collect();
    while !queue.is_empty() {
        queue.pop_front().map(|val| res.push(val));
        queue.pop_front().map(|val|queue.push_back(val));
    }
    res.reverse();
    res
}


/// 恢复牌堆
/// 输入slice索引越小则在牌堆中的位置越高
fn recover_order(arr: &[u32]) -> Vec<u32> {
    let mut queue = VecDeque::with_capacity(arr.len());
    for val in arr.iter() {
        if queue.len() > 0 {
            let bottom: u32 = queue.pop_back().unwrap();
            queue.push_front(bottom);
            queue.push_front(*val);
        } else {
            queue.push_front(*val);
        }
    }

    queue.iter().map(|val| *val).collect::<Vec<_>>()
}

fn main() {
    let input = [1_u32, 2, 3, 4, 5, 6, 7, 8,9];
    let res = sort_dispatch(&input);
    let recover = recover_order(res.as_slice());
    println!("start: {:?}", input);
    println!("result: {:?}", res);
    println!("recover: {:?}", recover);
}
