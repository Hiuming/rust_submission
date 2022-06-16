
/* 
@Exercise 1:
 */

pub fn solution_ex_one (){

    println!("Change value with references input parameter");
    let x = change_value(1, &mut 100);
    println!("{}",x);
}






fn change_value(input:u32, output: &mut u32) -> u32{
    if input ==1 {
        *output = 3;
    }
    else {
        *output = 4;
    }

    return *output;
}


/* 
@Exercise 2:
    Mục đích: giải quyết vấn đề ownership và borrowing ko dùng clone()
    Các bạn có thể sửa thêm logic để đúng với mục đichs bài này là liệt kê các số nguyên tố

@Explaination:
    Appoarching #1 : Theo tài liệu RUST, về owning và borrowing, thì hàm vector_is_prime() hiện tại được truyền vào tham trị, đòi hỏi ownership của parameter, với các dạng dữ liệu không có define method copy,
thì khi p được truyền vào scope của hàm vector_is_prime(), p ở scope ngoài đã bị remove. Vì vậy RUST compiler báo lỗi khi compile. Để tránh việc này, thì ta sẽ truyền
tham chiếu cho hàm. ( vì lúc này sẽ có kiểu dữ liệu là &Vec<u64>, nên phải destructuring với '&')
    Appoarching #2 : Tạo một mutable reference mượn dữ liệu của biến hiện tại. ( lúc này có kiểu dữ liệu là &mut u64, nên phải derefernce với '*'), ta vẫn sẽ in ra giá trị chủ ( prime ) để kiểm chứng
sự thay đổi ( chỉ borrow 1 lần cho 1 biến trong 1 scope, nhiều hơn sẽ bị báo lỗi)

 */


 pub fn solution_2(){
     println!(" Appoarching #1 :");
     solution_ex_two_first();
     println!(" Appoarching #2 :");
     solution_ex_two_second();
 }


fn solution_ex_two_first() {
    let mut count: u32 = 1;
    let mut num: u64 = 2;
    let mut primes: Vec<u64> = Vec::new();
    primes.push(2);

    while count < 10 {
        num += 1;
        if vector_is_prime_first(num, &primes) {
            count += 1;
            primes.push(num);
        }
    }
    println!("{:?}", primes);
}
fn solution_ex_two_second() {
    let mut count: u32 = 1;
    let mut num: u64 = 2;
    let mut primes: Vec<u64> = Vec::new();
    primes.push(2);
    let borrow_vec = &mut primes;
    

    while count < 10 {
        num += 1;
        if vector_is_prime_second(num, borrow_vec) {
            count += 1;
            borrow_vec.push(num);
        }
    }
    println!("{:?}", primes);
}



// Appoarching #1
fn vector_is_prime_first(num: u64, p: &Vec<u64>) -> bool {
   
    for &i in p {
        if num > i && num % i == 0 {
            return false;
        }
    }

    true
}

// Appoarching #2 :
fn vector_is_prime_second(num: u64, p: &mut Vec<u64>) -> bool {
   
    for i in p {
        if num > *i && num % *i == 0 {
            return false;
        }
    }
    true
}




/*
@Exercise 3:
    - Mục đích: giải quyết vấn đề ownership and borrowing ko dùng clone()
@Method :
    - Khi v được sừ dụng trong vòng for đầu tiên, đến khi vòng for kết thúc thì v đã được tính là out of scope, cho nên không thể được tái sử dụng ở lần tiếp theo ( theo tài liệu RUST),
vì vậy ta phải tạo thêm 1 biến mượn sau khi v đã out of scope để tránh xảy ra lỗi, ở đây với v thì chỉ timf ra phần tử max - min nên không cần mutable reference, ta sẽ tạo thêm 1 biến mượn là c sau 
khi v đã "mượn" xong và sử dụng xong.
*/
pub fn solution_3() {
    let mut values = vec![10, 11, 12];
    let v = &values;
   

    let mut max = 0;
    
    //for n in &mut values 

    for n in v {
        max = std::cmp::max(max, *n);
    }

    let c  = &mut values;

    println!("max is {}", max);
    println!("Converting to percentages of maximum value...");
    //for n in &mut values {
        
    for n in c {
        *n = 100 * (*n) / max;
    }
    println!("values: {:#?}", values);
}


/* Exercise 4
Mục đích : giải quyết vấn đề ownership và borrowing ko dùng clone()
Logic hiện tại đang sai (cho 1 vec -> đảo chiều vector đó)
*/
pub fn solution_ex_4(){
    let mut a = vec![1,2,3,4,5];
    let mut i = 5;
    let mut c = 0;
    let borrow_value = &mut a;
    loop {
        (*borrow_value, c) = test(borrow_value);
        println!("{}",c);
        if i >=5 {break;}
        i = i + 1;
    }
    println!("{:?}",borrow_value);
}

fn test( a: &mut Vec<u8>) -> (Vec<u8>, i32) {
    let mut b:Vec<u8>  = Vec::new();
    let mut c:u8 = 0;
    loop {
        if a.len() == 0 { break; }
        let mut d = a.pop().unwrap();
        c = c+d;
        b.push(d);
    }
    (b, c as i32)
}
