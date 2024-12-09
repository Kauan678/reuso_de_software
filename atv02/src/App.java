public class App {
    public static void main(String[] args) {
        Thread thread1 = new Thread(() -> {
            Logger logger = Logger.getInstance();
            logger.log("Thread 1: Log message 1");
            logger.log("Thread 1: Log message 2");
        });

        Thread thread2 = new Thread(() -> {
            Logger logger = Logger.getInstance();
            logger.log("Thread 2: Log message 1");
            logger.log("Thread 2: Log message 2");
        });

        thread1.start();
        thread2.start();

        try {
            thread1.join();
            thread2.join();
        } catch (InterruptedException e) {
            e.printStackTrace();
        }

        Logger logger = Logger.getInstance();
        System.out.println("Logs collected:");
        System.out.println(logger.getLogs());
    }
}
