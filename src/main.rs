use std::collections::VecDeque;

/// 发牌堆
///
/// 输入value     输入index
/// | 1 |          0 牌堆顶部
/// \ 2 |          1
/// \ 3 |          2
/// \ 4 |          3
/// \ 5 |          4
/// \ 6 |          5
/// \ 7 |          6
/// \ 8 |          7 底部
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
///
/// 输入value     输入index
/// | 8 |          0 牌堆顶部
/// \ 4 |          1
/// \ 6 |          2
/// \ 2 |          3
/// \ 7 |          4
/// \ 5 |          5
/// \ 3 |          6
/// \ 1 |          7 底部
fn recover_order(arr: &[u32]) -> Vec<u32> {
    let mut temp = arr.to_vec();
    temp.reverse();
    let mut queue = VecDeque::new();
    loop {
        if temp.len() > 0 {
            // 双端队列，取出最后一个牌放到最前面，恢复将最上层牌放到底层的操作
            if queue.len() > 0 {
                let bottom: u32 = queue.pop_back().unwrap();
                queue.push_front(bottom);
                let val = temp.pop().unwrap();
                queue.push_front(val);
            } else {
                let val = temp.pop().unwrap();
                queue.push_front(val);
            }
        } else {
            break;
        }
    }

    queue.iter().map(|val|*val).collect::<Vec<_>>()
}

fn main() {
    let input = [1_u32, 2, 3, 4, 5, 6, 7, 8];
    let res = sort_dispatch(&input);
    let recover = recover_order(res.as_slice());
    println!("start: {:?}", input);
    println!("result: {:?}", res);
    println!("recover: {:?}", recover);
}
