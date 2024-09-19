use tokio::signal;
use tokio_util::sync::CancellationToken;

#[tokio::main]
async fn main() {
    let token = CancellationToken::new(); // создаем токен отмены
    let mut tasks = vec![]; // вектор для хранения тасков

    // Создаем 10 воркеров
    for i in 0..10 {
        let cloned_token = token.clone(); // копируем токен для передачи каждому воркеру
        let task = tokio::spawn(async move {
            cloned_token.cancelled().await; // ожидаем отмену токена
            println!("Worker {} was stoped", i);
        });
        tasks.push(task); // добавляем каждый созданный таск в вектор
    }

    // Создаем таск который будет ожидать получения сигнала ctrl_c и завершать работу всех тасков
    tokio::spawn(async move {
        signal::ctrl_c().await.unwrap();
        token.cancel();
    });

    // Ожидание завершения тасков
    for task in tasks {
        task.await.unwrap();
    }

    println!("All the workers have completed their work.");
}
