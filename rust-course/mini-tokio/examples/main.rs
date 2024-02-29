use libc::sched_getcpu;
use tokio::select;

async fn print_with_cpu_id(task_name: &str) {
    for i in 0..1000 {
        if i % 100 == 0 {
            let cpu_id = unsafe { sched_getcpu() };
            println!("{} 在 CPU {} 上执行", task_name, cpu_id);
        }
        tokio::time::sleep(tokio::time::Duration::from_millis(1)).await;
    }
    println!("{} 完成", task_name);
}

#[tokio::main]
async fn main() {
    select! {
        _ = print_with_cpu_id("操作1") => {
            println!("操作1完成");
        },
        _ = print_with_cpu_id("操作2") => {
            println!("操作2完成");
        },
        _ = print_with_cpu_id("操作3") => {
            println!("操作3完成");
        },
        _ = print_with_cpu_id("操作4") => {
            println!("操作4完成");
        },
        _ = print_with_cpu_id("操作5") => {
            println!("操作5完成");
        },
        _ = print_with_cpu_id("操作6") => {
            println!("操作6完成");
        },
        _ = print_with_cpu_id("操作7") => {
            println!("操作7完成");
        },
        _ = print_with_cpu_id("操作8") => {
            println!("操作8完成");
        },
        _ = print_with_cpu_id("操作9") => {
            println!("操作9完成");
        },
        _ = print_with_cpu_id("操作10") => {
            println!("操作10完成");
        },
        _ = print_with_cpu_id("操作11") => {
            println!("操作11完成");
        },
        _ = print_with_cpu_id("操作12") => {
            println!("操作12完成");
        },
        _ = print_with_cpu_id("操作13") => {
            println!("操作13完成");
        },
        _ = print_with_cpu_id("操作14") => {
            println!("操作14完成");
        },
        _ = print_with_cpu_id("操作15") => {
            println!("操作15完成");
        },
        _ = print_with_cpu_id("操作16") => {
            println!("操作16完成");
        },
        _ = print_with_cpu_id("操作17") => {
            println!("操作17完成");
        },
        _ = print_with_cpu_id("操作18") => {
            println!("操作18完成");
        },
        _ = print_with_cpu_id("操作19") => {
            println!("操作19完成");
        },
        _ = print_with_cpu_id("操作20") => {
            println!("操作20完成");
        },
    }
}
