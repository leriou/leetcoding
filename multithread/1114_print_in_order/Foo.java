class Foo {

  public Foo() {}

    Semaphore s = new Semaphore(0);
    Semaphore t = new Semaphore(0);
    
    public void first(Runnable printFirst) throws InterruptedException {
        printFirst.run();
        s.release();
    }

    public void second(Runnable printSecond) throws InterruptedException {
        s.acquire();
        printSecond.run();
        t.release();
    }

    public void third(Runnable printThird) throws InterruptedException {
        t.acquire();
        printThird.run();
    }
}