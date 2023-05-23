use std::{
    fmt::Debug,
    mem::ManuallyDrop,
    ops::{Deref, DerefMut},
    ptr,
    sync::Arc,
};

use crossbeam_queue::ArrayQueue;

pub trait Clear {
    fn clear(&mut self);
}

#[derive(Debug)]
pub struct Pool<T> {
    pub(crate) values: ArrayQueue<T>,
    pub(crate) max_size: usize,
}

impl<T> Pool<T>
where
    T: Clear,
{
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            values: ArrayQueue::new(capacity),
            max_size: capacity,
        }
    }

    #[inline]
    pub fn create(self: Arc<Self>) -> PoolObjectContainer<T>
    where
        T: Clear + Default,
    {
        let val = self.values.pop().unwrap_or_else(|| Default::default());
        PoolObjectContainer::new(val, self)
    }

    #[inline]
    pub fn max_size(&self) -> usize {
        self.max_size
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.values.len()
    }

    #[inline]
    pub fn push(&self, value: T) -> Result<(), T> {
        self.values.push(value)
    }
}

#[derive(Debug)]
pub struct PoolObjectContainer<T: Clear> {
    pool: Arc<Pool<T>>,
    inner: ManuallyDrop<T>,
}

impl<T: Clear> PoolObjectContainer<T> {
    #[inline]
    fn new(val: T, pool: Arc<Pool<T>>) -> Self {
        Self {
            pool,
            inner: ManuallyDrop::new(val),
        }
    }
}

impl<T: Clear> Deref for PoolObjectContainer<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T: Clear> DerefMut for PoolObjectContainer<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<T: Clear> Drop for PoolObjectContainer<T> {
    fn drop(&mut self) {
        let val = unsafe { ptr::read(&self.inner) };
        let mut val = ManuallyDrop::into_inner(val);

        let pool = &self.pool;
        if pool.len() >= pool.max_size() {
            drop(val);
        } else {
            val.clear();
            if let Err(val) = pool.push(val) {
                drop(val);
            }
        }
    }
}
