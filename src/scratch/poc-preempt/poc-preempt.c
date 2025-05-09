/*
	Rough proof-of-concept for running tasks in a pre-emptive manner, to
	simulate how the task switcher will work for the 'host-rust_std' target
	for smeg.  Each created thread will run a single task, and using this
	architecture we can run N of them simultaneously to simulate multiple
	cores - clearly imperfectly, but the aim of the 'host-*' target is easier
	local development and testing, not a perfect 1:1 hardware simulation.

	The basic premise relies on raising POSIX signals on specific threads,
	essentially an interrupt, then using a blocking call from within the
	handler to prevent an immediate return.  Coupled with a separate thread
	for co-ordination of the blocked threads (ie. analogue of 'systick'), we
	can ensure we proceed through the process in lock-step.  This only works
	when there are no other sources of the signal and when the task threads
	themselves do not exit prematurely; in practice this is achievable with
	the intended implementation.

	Useful resources, particularly 'signal-safety' to determine what is and
	isn't available from within a signal handler:

		$ man 7 signal-safety
		$ man 2 sigaction
		$ man 2 sigreturn

	Note the list of 'pthread_*' functions does not include 'pthread_equal()',
	access to which would have simplified the implementation somewhat by
	removing the need for the extra 'fdControl' and 'fdAck' pipes.  What would
	have been really awesome would have been able to use 'getcontext()' and
	'sigreturn()' to do actual context swapping :(

	Compiling and running:

		$ gcc -Wall -pedantic -o poc-preempt poc-preempt.c -lpthread
		$ ./poc-preempt
*/

#include <errno.h>
#include <stdio.h>
#include <stdlib.h>

#include <pthread.h>
#include <signal.h>
#include <unistd.h>

#define NUM_THREADS 5

#define SUSPEND SIGUSR1

struct ThreadControlBlock
{
	int id;
	pthread_t handle;
	int isHandleValid;
	int fdPipe[2];
	char name[16];
	int nameLength;
	volatile int isThreadDone;
};

static struct ThreadControlBlock threads[NUM_THREADS] = {0};

static int fdControl[2] = {0};
static int fdAck[2] = {0};

void onSuspend(int sig, siginfo_t *info, void *ucontext)
{
	if (sig != SUSPEND)
		return;

	if (fdControl[0] < 3 || fdAck[1] < 3)
		exit(-1);

	// Technically we should save and restore errno, too - but how that would
	// work in a pthreads-environment without TLS, I don't know !
	int threadId;
	if (read(fdControl[0], &threadId, sizeof(threadId)) != sizeof(threadId))
		exit(-2);

	if (threadId < 0 || threadId >= NUM_THREADS)
		exit(-3);

	if (write(fdAck[1], &threadId, sizeof(threadId)) != sizeof(threadId))
		exit(-4);

	struct ThreadControlBlock *thisThread = &threads[threadId];

	write(STDOUT_FILENO, thisThread->name, thisThread->nameLength);
	write(STDOUT_FILENO, " SUSPEND\n", 9);
	while (1)
	{
		char cmd;
		int result = read(thisThread->fdPipe[0], &cmd, 1);
		write(STDOUT_FILENO, thisThread->name, thisThread->nameLength);

		if (result == 0 || result == EBADF)
		{
			write(STDOUT_FILENO, " PIPE CLOSED\n", 13);
			return;
		}

		if (result == 1 && cmd == 'R')
		{
			write(STDOUT_FILENO, " RESUME\n", 8);
			return;
		}

		write(STDOUT_FILENO, " WEIRDNESS\n", 11);
	}
}

void threadEntrypoint(struct ThreadControlBlock *args)
{
	if (!args)
		return;

	sigset_t mask;
	sigfillset(&mask);
	sigdelset(&mask, SUSPEND);
	if (pthread_sigmask(SIG_SETMASK, &mask, NULL) != 0)
	{
		perror("pthread_sigmask(thread)");
		goto done;
	}

	printf("[%2d] Waiting to be initialised...\n", args->id);
	pthread_kill(pthread_self(), SUSPEND);

	printf("[%2d] Initialised.  Running loop...\n", args->id);
	for (int counter = 0; counter < 5; counter++)
	{
		printf("[%2d] Loop counter = %d\n", args->id, counter);
		sleep(1);
	}

done:
	args->isThreadDone = 1;
	while (1)
	{
		printf("[%2d] Thread is a zombie...going to sleep...\n", args->id);

		// Expect EINTR; this demonstrates it's possible this won't sleep for
		// the full 10 seconds and shows syscalls can be shorted by signals
		sleep(10);
	}
}

void systicker(void)
{
	sleep(1);
	printf("Systick started...\n");

	int currentThreadId = -1;
	int nextThreadId = -1;
	for (int i = 0; i < 50; i++)
	{
		for (int j = 0; j < NUM_THREADS; j++)
		{
			if (++nextThreadId >= NUM_THREADS)
				nextThreadId = 0;

			if (!threads[nextThreadId].isThreadDone)
				break;
		}

		if (threads[nextThreadId].isThreadDone)
		{
			printf("[TICK] All threads have terminated\n");
			break;
		}

		printf("[TICK] Context-switching threads %d -> %d\n", currentThreadId, nextThreadId);

		if (currentThreadId >= 0)
		{
			int result = pthread_kill(threads[currentThreadId].handle, SUSPEND);
			if (result != 0)
			{
				perror("pthread_kill(suspend)");
				return;
			}

			if (write(fdControl[1], &currentThreadId, sizeof(currentThreadId)) != sizeof(currentThreadId))
			{
				perror("write(threadId)");
				return;
			}

			int ack;
			if (read(fdAck[0], &ack, sizeof(ack)) != sizeof(ack))
			{
				perror("read(threadId)");
				return;
			}

			if (ack != currentThreadId)
			{
				printf("[TICK] Signal handler didn't ack with the correct thread ID.\n");
				return;
			}
		}

		if (write(threads[nextThreadId].fdPipe[1], "R", 1) != 1)
		{
			perror("write(resume)");
			printf("[TICK] Thread %d pipe write (resume) went weird\n", nextThreadId);
			return;
		}
		else
			currentThreadId = nextThreadId;

		sleep(2);
	}
}

int main(int argc, char *argv[])
{
	int errorCode = 0;

	sigset_t mask;
	sigemptyset(&mask);
	sigaddset(&mask, SUSPEND);
	if (pthread_sigmask(SIG_BLOCK, &mask, NULL) != 0)
	{
		perror("pthread_sigmask(main)");
		errorCode = 1;
		goto cleanUp;
	}

	struct sigaction handler = {0};
	handler.sa_flags = SA_SIGINFO;
	handler.sa_sigaction = &onSuspend;
	if (sigaction(SUSPEND, &handler, NULL) != 0)
	{
		perror("sigaction(suspend)");
		errorCode = 2;
		goto cleanUp;
	}

	if (pipe(fdControl) != 0)
	{
		perror("pipe(control)");
		errorCode = 3;
		goto cleanUp;
	}

	if (pipe(fdAck) != 0)
	{
		perror("pipe(ack)");
		errorCode = 4;
		goto cleanUp;
	}

	for (int i = 0; i < NUM_THREADS; i++)
	{
		threads[i].id = i;
		sprintf(threads[i].name, "[%2d]", i);
		threads[i].nameLength = 4;

		if (pipe(threads[i].fdPipe) != 0)
		{
			perror("pipe(thread)");
			errorCode = 5;
			goto cleanUp;
		}

		int result = pthread_create(
			&threads[i].handle,
			NULL,
			(void * (*)(void *)) &threadEntrypoint,
			&threads[i]);

		if (result != 0)
		{
			perror("pthread_create(threads)");
			errorCode = 6;
			goto cleanUp;
		}

		threads[i].isHandleValid = 1;

		if (write(fdControl[1], &i, sizeof(i)) != sizeof(i))
		{
			perror("write(threadId)");
			errorCode = 7;
			goto cleanUp;
		}

		int ack;
		if (read(fdAck[0], &ack, sizeof(ack)) != sizeof(ack))
		{
			perror("read(threadId)");
			errorCode = 8;
			goto cleanUp;
		}

		if (ack != i)
		{
			printf("Signal handler didn't ack with the correct thread ID.\n");
			errorCode = 9;
			goto cleanUp;
		}
	}

	pthread_t systick;
	int result = pthread_create(
		&systick,
		NULL,
		(void * (*)(void *)) &systicker,
		threads);

	if (result != 0)
	{
		perror("pthread_create(systick)");
		errorCode = 10;
		goto cleanUp;
	}

	printf("Joining on systick thread...\n");
	pthread_join(systick, NULL);

cleanUp:
	for (int i = 0; i < NUM_THREADS; i++)
	{
		if (threads[i].isHandleValid)
		{
			pthread_cancel(threads[i].handle);
			pthread_join(threads[i].handle, NULL);
		}

		if (threads[i].fdPipe[0] > 2)
			close(threads[i].fdPipe[0]);

		if (threads[i].fdPipe[1] > 2)
			close(threads[i].fdPipe[1]);
	}

	if (fdAck[0] > 2)
		close(fdAck[0]);

	if (fdAck[1] > 2)
		close(fdAck[1]);

	if (fdControl[0] > 2)
		close(fdControl[0]);

	if (fdControl[1] > 2)
		close(fdControl[1]);

	return errorCode;
}
