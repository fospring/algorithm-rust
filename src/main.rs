use std::collections::VecDeque;

/// 发牌堆
/// 输入slice索引越小则在牌堆中的位置越高
fn sort_dispatch(arr: &[u32]) -> Vec<u32> {
    let mut res = Vec::new();
    let mut queue: VecDeque<_> = arr.iter().map(|elem| *elem).collect();
    loop {
        if queue.len() > 1 {
            let val = queue.pop_front().unwrap();
            res.push(val);
            let tmp = queue.pop_front().unwrap();
            queue.push_back(tmp)
        } else if queue.len() == 1 {
            let val = queue.pop_front().unwrap();
            res.push(val);
            break;
        } else {
            break;
        }
    }
    res.reverse();
    res
}


/// 恢复牌堆
/// 输入slice索引越小则在牌堆中的位置越高
fn recover_order(arr: &[u32]) -> Vec<u32> {
    let temp = arr.to_vec();
    let mut queue = VecDeque::new();
    for val in temp {
        if queue.len() > 0 {
            let bottom: u32 = queue.pop_back().unwrap();
            queue.push_front(bottom);
            queue.push_front(val);
        } else {
            queue.push_front(val);
        }
    }

    queue.iter().map(|val| *val).collect::<Vec<_>>()
}

fn main() {
    let input = [1_u32, 2, 3, 4, 5, 6, 7, 8];
    let res = sort_dispatch(&input);
    let recover = recover_order(res.as_slice());
    println!("start: {:?}", input);
    println!("result: {:?}", res);
    println!("recover: {:?}", recover);
}
