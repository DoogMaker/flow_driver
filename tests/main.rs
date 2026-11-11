use std::time;
use std::thread;

fn main1(){
    let now_time = time::Instant::now();
    println!("now_time: {:?}", now_time);

    thread::sleep(time::Duration::from_secs(1));

    let elapsed_time = now_time.elapsed();
    println!("elapsed_time: {:?}", elapsed_time);

}

mod test {
    use std::time;
    #[test]
    fn test_time() {
        let now_time = time::Instant::now();
        println!("now_time: {:?}", now_time);

        std::thread::sleep(time::Duration::from_secs(1));

        let elapsed_time = now_time.elapsed();
        println!("elapsed_time: {:?}", elapsed_time);
    }
}