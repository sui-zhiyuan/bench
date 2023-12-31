public static class Program
{
    public static async Task Main(string[] args)
    {
        var tasks = new List<Task>();
        foreach (var _ in Enumerable.Range(0, (int)THREAD_COUNT))
        {
            tasks.Add(Running());
        }
        tasks.Add(Print());
        await Task.WhenAll(tasks);

        var task_finished = Interlocked.Read(ref counter);
        var task_required = TASK_COUNT * THREAD_COUNT;
        if (task_finished != task_required)
        {
            throw new Exception($"Finish finished {task_finished} tasks, require {task_required}");
        }
        Console.WriteLine($"Finish finished {task_finished} tasks, require {task_required}");
    }

    static Int64 counter = 0;
    static Int64 finish = THREAD_COUNT;
    const Int64 TASK_SPAN = 15;
    const Int64 TASK_COUNT = 15 * 1000 / TASK_SPAN;
    const Int64 THREAD_COUNT = 10_000;
    private static async Task Running()
    {
        var task_span = TimeSpan.FromMilliseconds(TASK_SPAN);
        foreach (var _ in Enumerable.Range(0, (int)TASK_COUNT))
        {
            await Task.Delay(task_span);
            Interlocked.Increment(ref counter);
        }
        Interlocked.Decrement(ref finish);
    }

    private static async Task Print() {
        var last_count = 0L;
        var start_time = DateTime.Now;
        while (true)
        {
            await Task.Delay(1000);
            var current_count = Interlocked.Read(ref counter);
            var span = DateTime.Now - start_time;
            var change = current_count - last_count;
            Console.WriteLine($"{change} tasks per second at {span}");
            last_count = current_count;
            if (Interlocked.Read(ref finish) == 0)
            {
                break;
            }
        }
    }
}