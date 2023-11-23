




fn main() {


    let mut nums = [10,9,8,7,6,5,4,3,2,1,0,-1];
    println!("{:?}", bubble_sort_with_flag(&mut nums));

    println!("{}", recur(12));

    

    
}


/* 冒泡排序（标志优化） */
fn bubble_sort_with_flag(nums: &mut [i32]) -> &mut [i32]{
    // 外循环：未排序区间为 [0, i]
    for i in (1..nums.len()).rev() {
        let mut flag = false; // 初始化标志位
        // 内循环：将未排序区间 [0, i] 中的最大元素交换至该区间的最右端 
        for j in 0..i {
            if nums[j] > nums[j + 1] {
                // 交换 nums[j] 与 nums[j + 1]
                let tmp = nums[j];
                nums[j] = nums[j + 1];
                nums[j + 1] = tmp;
                flag = true;  // 记录交换元素
            }
        }
        if !flag {break};  // 此轮冒泡未交换任何元素，直接跳出
    }
    nums
}


/* 递归 */
fn recur(n: i32) -> i32 {
    // 终止条件
    if n == 1 {
        return 1;
    }
    // 递：递归调用
    let res = recur(n - 1);
    // 归：返回结果
    n + res
}
