package main

import (
	"fmt"
	"sync"
	"sync/atomic"
	"time"
)

func main() {
	var i int64
	wg := sync.WaitGroup{}
	for i = 0; i < THREAD_COUNT; i++ {
		wg.Add(1)
		go sub_task(&wg)
	}
	finish := wait_all(&wg)
	print_result(finish)
}

const TASK_SPAN int64 = 15
const TASK_COUNT int64 = 15 * 1000 / TASK_SPAN
const THREAD_COUNT int64 = 10_000

var count int64 = 0

func sub_task(finish *sync.WaitGroup) {
	defer finish.Done()
	task_span := time.Millisecond * (time.Duration)(TASK_SPAN)
	var i int64
	for i = 0; i < TASK_COUNT; i++ {
		<-time.After(task_span)
		atomic.AddInt64(&count, 1)
	}
}

func wait_all(finish *sync.WaitGroup) <-chan struct{} {
	result := make(chan struct{}, 1)
	go func() {
		finish.Wait()
		result <- struct{}{}
		close(result)
	}()
	return result
}

func print_result(finish <-chan struct{}) {
	var last_count int64 = 0
	start_time := time.Now()
outer:
	for {
		select {
		case <-finish:
			break outer
		case <-time.After(time.Second):
		}
		current_count := atomic.LoadInt64(&count)
		span := time.Since(start_time)
		change := current_count - last_count
		fmt.Printf("%v tasks per second at %v\n", change, span)
		last_count = current_count
	}
}
