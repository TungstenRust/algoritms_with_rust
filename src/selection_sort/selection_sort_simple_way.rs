fn main(){
    let mut list_nums:[i32; 7]=[10, 23, 4, 5, 66, 7, -3];
    println!(":?", list_nums);

    let list_len = list_nums.len();
    for i in 0..list_len {
        for j in i+1..list_len {
            if list_nums[j] < list_nums[i] {
                swap_us(&mut list_nums, j, i);
            }
            println!("{:?}", list_nums);
        }
    }
}

fn swap_us(m_list: &mut [i32;7], j:usize, i:usize){
    let temp:i32;
    temp=m_list[i];
    m_list[i] = m_list[j];
    m_list[j] = temp;
}