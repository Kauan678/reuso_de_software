public class Logger {
    private static Logger instance;
    private StringBuilder logs;

    private Logger() {
        logs = new StringBuilder();
    }

    public static Logger getInstance() {
        if (instance == null) {
            instance = new Logger();
        }
        return instance;
    }

    public void log(String message) {
        logs.append(message).append("\n");
    }

    public String getLogs() {
        return logs.toString();
    }
}

// Possível solução para o problema:
/*
public class Logger {
    private static volatile Logger instance;
    private StringBuilder logs;

    private Logger() {
        logs = new StringBuilder();
    }

    public static Logger getInstance() {
        if (instance == null) {
            synchronized (Logger.class) {
                if (instance == null) {
                    instance = new Logger();
                }
            }
        }
        return instance;
    }

    public synchronized void log(String message) {
        logs.append(message).append("\n");
    }

    public synchronized String getLogs() {
        return logs.toString();
    }
}
 */