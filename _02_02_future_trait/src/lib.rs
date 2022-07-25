use std::sync::PoisonError;

trait SimpleFuture {
    type Output;
    fn poll(&mut self, wake: fn()) -> Poll<Self::Output>;
}

enum Poll<T> {
    Ready(T),
    Pending,
}

struct Socket;
impl Socket {
    fn has_data_to_read(&self) -> bool {
        true
    }

    fn read_buf(&self) -> Vec<u8>{
        vec![]
    }

    fn set_readable_callback(&self , _wake : fn()){

    } 
    
}


struct SocketRead<'a> {
    socket : &'a Socket,
}

impl SimpleFuture for SocketRead<'a> {
    type Vec<u8>;
    fn poll(&mut self, wake: fn()) -> Poll<Self::Output>{
        if self.socket.has_data_to_read(){
            Poll::Ready(self.socket.read_buf())
        } else {
            self.socket.set_readable_callback(wake);
            Poll::Pending
        }
    }
}

pub struct  Join<FutureA,FutureB>{
    a: Option<FutureA>,
    b: Option<FutureB>,
}

impl<FutureA, FutureB>  SimpleFuture for Join<FutureA,FutureB> 
where
    FutureA: SimpleFuture<Output = ()>,
    FutureB: SimpleFuture<Output = ()>,  {
        type Output = ();

        fn poll(&mut self, wake: fn()) -> Poll<Self::Output> {
            // Attempt to complete future `a`.
            if let Some(a) = &mut self.a {
                if let Poll::Ready(()) = a.poll(wake) {
                    self.a.take();
                }
            }

            // Attempt to complete future `b`.
            if let Some(b) = &mut self.b {
                if let Poll::Ready(()) = b.poll(wake) {
                    self.b.take();
                }
            }

            if self.a.is_none() && self.b.is_none() {
                // Both futures have completed -- we can return successfully
                Poll::Ready(())
            } else {
                // One or both futures returned `Poll::Pending` and still have
                // work to do. They will call `wake()` when progress can be made.
                Poll::Pending
            }
        }
}

pub struct AndThenFut<FutureA, FutureB> {
    first: Option<FutureA>,
    second: FutureB, 
}

impl<FutureA, FutureB> SimpleFuture for AndThenFut<FutureA, FutureB>
where
    FutureA: SimpleFuture<Output = ()>,
    FutureB: SimpleFuture<Output = ()>,
{
    type Output = ();
    fn poll(&mut self, wake: fn()) -> Poll<Self::Output> {
        if let Some(first) = &mut self.first {
            match first.poll(wake){
                // We've completed the first future -- remove it and start on
                // the second!
                Poll::Ready(()) => self.first.take(),
                // We couldn't yet complete the first future.
                Poll::Pending => return Poll::Pending,
            }
        }
        // Now that the first future is done, attempt to complete the second.
        self.second.poll(wake)
    }
}


mod real_future {
    use std::{
        future::Future as RealFuture,
        pin::Pin,
        task::{Context, Poll},
    };

    trait Future {
        type Output;
        fn poll(
            // Note the change from `&mut self` to `Pin<&mut Self>`:
            self: Pin<&mut Self>,
            // and the change from `wake: fn()` to `cx: &mut Context<'_>`:
            cx: &mut Context<'_>,
        ) -> Poll<Self::Output>;
    }


    impl<O> Future for RealFuture<O>  {
        type Output = O;
        fn poll(
            // Note the change from `&mut self` to `Pin<&mut Self>`:
            self: Pin<&mut Self>,
            // and the change from `wake: fn()` to `cx: &mut Context<'_>`:
            cx: &mut Context<'_>,
        ) -> Poll<Self::Output>{
            RealFuture::poll(self, cx)
        }
        
    }
}